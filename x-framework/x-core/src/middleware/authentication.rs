use async_trait::async_trait;
use axum::extract::FromRequest;
use http::header::AUTHORIZATION;
use http::{HeaderMap, StatusCode};
use futures_util::future::BoxFuture;
use http_body::Body;
use axum::body::Bytes;
use axum::Error;
use http_body::combinators::UnsyncBoxBody;
use hyper::Request;
use tower_http::auth::AsyncAuthorizeRequest;

use redis::Commands;
use anyhow::Result;
use axum::response::Response;
use x_common::model::response::R;
use x_common::utils::token::TokenUtils;

use crate::application::Application;

#[derive(Clone, Copy)]
pub struct XAuthorize;

pub struct CurrentUser(String);

impl CurrentUser {
    pub fn id(&self) -> &str {
        &self.0
    }
}

#[async_trait]
impl<B> FromRequest<B> for CurrentUser
where
    B: Send + Sync,
{
    type Rejection = Response<hyper::Body>;

    async fn from_request(req: &mut axum::extract::RequestParts<B>) -> Result<Self, Self::Rejection> {
        let uri = req.uri().path();
        let paths = Application::config().auth.ignore.to_owned().unwrap_or_default();
        for path in paths {
            if uri.starts_with(&path) {
                match find_jwt_token(&req.headers()).await {
                    Ok(jwt_token) => match TokenUtils::fetch_current_user_id(&jwt_token) {
                        Ok(user_id) => return Ok(CurrentUser(user_id)),
                        _ => return Ok(CurrentUser("".to_string())),
                    },
                    _ => return Ok(CurrentUser("".to_string())),
                }
            }
        }
        match find_jwt_token(&req.headers()).await {
            Ok(jwt_token) => match TokenUtils::fetch_current_user_id(&jwt_token) {
                Ok(user_id) => Ok(CurrentUser(user_id)),
                _ => Err(build_json_respones(StatusCode::UNAUTHORIZED, "token is invalid")),
            },
            _ => Err(build_json_respones(StatusCode::UNAUTHORIZED, "unauthorized")),
        }
    }
}

async fn find_jwt_token(headers: &HeaderMap) -> Result<String> {
    if let Some(header_value) = headers.get(AUTHORIZATION) {
        let access_token = header_value.to_str().unwrap().replace("Bearer ", "");
        match Application::redis().get::<_, String>(access_token) {
            Ok(jwt) => Ok(jwt),
            Err(_) => Err(anyhow::anyhow!("not fount jwt_token")),
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
            serde_json::to_string(&R::<&str>::error(message)).unwrap_or_default(),
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
                Ok(jwt_token) => match TokenUtils::fetch_current_user_id(&jwt_token) {
                    Ok(userid) => {
                        request.extensions_mut().insert(CurrentUser(userid));
                        return Ok(request);
                    },
                    Err(_) => {
                        return Err(Response::builder()
                            .status(StatusCode::UNAUTHORIZED)
                            .header("content-type", "application/json")
                            .body(UnsyncBoxBody::new(
                                hyper::Body::from(
                                    serde_json::to_string(&R::<&str>::error("token is invalid")).unwrap_or_default(),
                                )
                                .map_err(|e| Error::new(e)),
                            ))
                            .unwrap());
                    },
                },
                Err(_) => Err(Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header("content-type", "application/json")
                    .body(UnsyncBoxBody::new(
                        hyper::Body::from(serde_json::to_string(&R::<&str>::error("Unauthorized")).unwrap_or_default())
                            .map_err(|e| Error::new(e)),
                    ))
                    .unwrap()),
            }
        })
    }
}
