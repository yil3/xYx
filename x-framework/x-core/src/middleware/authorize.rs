use async_trait::async_trait;
use axum::extract::FromRequest;
use futures_util::future::BoxFuture;
use http::header::AUTHORIZATION;
use http::StatusCode;
use http_body::Body;
use redis::Commands;
use tower_http::auth::AsyncAuthorizeRequest;

use anyhow::Result;
use axum::body::Bytes;
use axum::response::Response;
use axum::Error;
use http_body::combinators::UnsyncBoxBody;
use hyper::Request;
use x_common::errors::XError;
use x_common::model::response::R;
use x_common::utils::token::TokenUtils;

use crate::application::Application;

async fn find_jwt_token(access_token: &str) -> Result<String> {
    let jwt: String = Application::redis().get(access_token)?;
    Ok(jwt)
}

#[derive(Clone, Copy)]
pub struct XAuthorize;

#[derive(Default, Debug, Clone)]
pub struct UserId(pub String);

#[async_trait]
impl<B> FromRequest<B> for UserId
where
    B: Send + Sync,
{
    type Rejection = XError;

    async fn from_request(req: &mut axum::extract::RequestParts<B>) -> Result<Self, Self::Rejection> {
        let uri = req.uri().path();
        let paths = Application::config().auth.ignore.to_owned().unwrap_or_default();
        for path in paths {
            if uri.starts_with(&path) {
                return Ok(UserId("".to_string()));
            }
        }
        if let Some(header_value) = req.headers().get(AUTHORIZATION) {
            let access_token = header_value.to_str().unwrap().replace("Bearer ", "");
            let jwt_token = find_jwt_token(&access_token).await.unwrap_or_default();
            match TokenUtils::fetch_current_user_id_from_jwt_token(jwt_token) {
                Ok(user_id) => Ok(UserId(user_id)),
                Err(e) => Err(XError::AnyhowError(anyhow::anyhow!(e))),
            }
        } else {
            Err(XError::Unauthorized)
        }
    }
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
            if let Some(header_value) = request.headers().get(AUTHORIZATION) {
                if header_value.to_str().unwrap().starts_with("Bearer ") {
                    let access_token = header_value.to_str().unwrap().replace("Bearer ", "");
                    let jwt_token = find_jwt_token(&access_token).await.unwrap_or_default();
                    match TokenUtils::fetch_current_user_id_from_jwt_token(jwt_token) {
                        Ok(userid) => {
                            request.extensions_mut().insert(UserId(userid));
                            return Ok(request);
                        },
                        Err(_) => {
                            return Err(Response::builder()
                                .status(StatusCode::UNAUTHORIZED)
                                .header("content-type", "application/json")
                                .body(UnsyncBoxBody::new(
                                    hyper::Body::from(
                                        serde_json::to_string(&R::<&str>::error("token is invalid"))
                                            .unwrap_or_default(),
                                    )
                                    .map_err(|e| Error::new(e)),
                                ))
                                .unwrap());
                        },
                    }
                }
            }
            Err(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header("content-type", "application/json")
                .body(UnsyncBoxBody::new(
                    hyper::Body::from(serde_json::to_string(&R::<&str>::error("Unauthorized")).unwrap_or_default())
                        .map_err(|e| Error::new(e)),
                ))
                .unwrap())
        })
    }
}
