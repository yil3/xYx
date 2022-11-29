use async_trait::async_trait;
use axum::body::Bytes;
use axum::extract::FromRequestParts;
use axum::Error;
use futures_util::future::BoxFuture;
use http::header::AUTHORIZATION;
use http::request::Parts;
use http::{HeaderMap, StatusCode};
use http_body::combinators::UnsyncBoxBody;
use http_body::Body;
use hyper::Request;
use serde::Deserialize;
use tower_http::auth::AsyncAuthorizeRequest;

use anyhow::Result;
use axum::response::Response;
use redis::Commands;
use x_common::model::response::R;

use crate::application::Application;

/**
* @Author xYx
* @Date 2022-10-26 21:36:23
*/

#[derive(Clone, Copy)]
pub struct XAuthorize;

#[derive(Deserialize, Default)]
pub struct CurrentUser {
    pub user_id: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

#[async_trait]
impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = Response<hyper::Body>;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let uri = parts.uri.path();
        let paths = Application::config().auth.ignore.to_owned().unwrap_or_default();
        for path in paths {
            if uri.starts_with(&path) {
                match find_jwt_token(&parts.headers).await {
                    Ok(current_user) => return Ok(current_user),
                    _ => return Ok(CurrentUser::default()),
                }
            }
        }
        match find_jwt_token(&parts.headers).await {
            Ok(current_user) => Ok(current_user),
            _ => Err(build_json_respones(StatusCode::UNAUTHORIZED, "unauthorized")),
        }
    }
}

async fn find_jwt_token(headers: &HeaderMap) -> Result<CurrentUser> {
    if let Some(header_value) = headers.get(AUTHORIZATION) {
        let access_token = header_value.to_str().unwrap().replace("Bearer ", "");
        match Application::redis().get::<_, String>(access_token) {
            Ok(user_info) => Ok(serde_json::from_str(&user_info).expect("user info is invalid")),
            Err(_) => Err(anyhow::anyhow!("not fount user info")),
        }
    } else {
        Err(anyhow::anyhow!("not fount header: Authorization"))
    }
}

fn build_json_respones(status: StatusCode, message: &str) -> Response<hyper::Body> {
    Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(hyper::Body::from(
            serde_json::to_string(&R::error(message)).unwrap_or_default(),
        ))
        .unwrap_or_default()
}

impl<B> AsyncAuthorizeRequest<B> for XAuthorize
where
    B: Send + Sync + 'static,
{
    type RequestBody = B;
    type ResponseBody = UnsyncBoxBody<Bytes, Error>;
    type Future = BoxFuture<'static, Result<Request<B>, Response<Self::ResponseBody>>>;

    fn authorize(&mut self, mut request: Request<B>) -> Self::Future {
        Box::pin(async {
            let uri = request.uri().path();
            let paths = Application::config().auth.ignore.to_owned().unwrap_or_default();
            for path in paths {
                if uri.starts_with(&path) {
                    return Ok(request);
                }
            }
            match find_jwt_token(request.headers()).await {
                Ok(current_user) => {
                    request.extensions_mut().insert(current_user);
                    Ok(request)
                },
                Err(_) => Err(Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header("content-type", "application/json")
                    .body(UnsyncBoxBody::new(
                        hyper::Body::from(serde_json::to_string(&R::error("Unauthorized")).unwrap_or_default())
                            .map_err(|e| Error::new(e)),
                    ))
                    .unwrap()),
            }
        })
    }
}
