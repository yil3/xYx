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
use x_common::model::response::R;
use x_common::utils::token::TokenUtils;

use crate::application::Application;

#[derive(Clone, Copy)]
pub struct XAuthorize;

#[derive(Debug)]
pub struct UserId(pub String);

async fn find_jwt_token(access_token: &str) -> Result<String> {
    let jwt: String = Application::redis().get(access_token)?;
    Ok(jwt)
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
                        Ok(uid) => {
                            request.extensions_mut().insert(UserId(uid));
                            return Ok(request);
                        },
                        Err(_) => {
                            return Err(Response::builder()
                                .status(StatusCode::UNAUTHORIZED)
                                .header("content-type", "application/json")
                                .body(UnsyncBoxBody::new(
                                    hyper::Body::from(
                                        serde_json::to_string(&R::<&str>::error("token is invalid")).unwrap(),
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
                    hyper::Body::from(serde_json::to_string(&R::<&str>::error("Unauthorized")).unwrap())
                        .map_err(|e| Error::new(e)),
                ))
                .unwrap())
        })
    }
}
