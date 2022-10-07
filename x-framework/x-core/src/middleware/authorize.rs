use futures_util::future::BoxFuture;
use http::header::AUTHORIZATION;
use http::{Method, StatusCode};
use http_body::Body;
use tower_http::auth::AsyncAuthorizeRequest;

use axum::body::Bytes;
use axum::response::Response;
use axum::Error;
use http_body::combinators::UnsyncBoxBody;
use hyper::Request;


#[derive(Clone, Copy)]
pub struct XAuthorize;

impl<B> AsyncAuthorizeRequest<B> for XAuthorize
where
    B: Send + Sync + 'static,
{
    type RequestBody = B;

    type ResponseBody = UnsyncBoxBody<Bytes, Error>;

    type Future = BoxFuture<'static, Result<Request<B>, Response<Self::ResponseBody>>>;

    fn authorize(&mut self, request: Request<B>) -> Self::Future {
        Box::pin(async {
            if request.uri().path() == "/authorize" && request.method() == Method::GET {
                return Ok(request);
            }
            if request.uri().path() == "/token" {
                return Ok(request);
            }
            if let Some(header_value) = request.headers().get(AUTHORIZATION) {
                if header_value.to_str().unwrap().starts_with("Bearer ") {
                    let _token = header_value.to_str().unwrap().replace("Bearer ", "");
                    // TODO: check token
                }
            }
            Err(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(UnsyncBoxBody::new(hyper::Body::empty().map_err(|e| Error::new(e))))
                .unwrap())
        })
    }
}
