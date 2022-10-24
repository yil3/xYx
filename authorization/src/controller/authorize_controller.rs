use crate::{
    dto::request::{authorize_requests::AuthorizeRequest, token_requests::TokenRequest},
    service::authorize_service::AuthorizeService,
};
use axum::{
    body::Body,
    extract::Query,
    http::Request,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use x_common::model::response::R;
use x_core::middleware::authorize::UserId;

pub fn route() -> Router {
    Router::new().route("/", get(authorize)).route("/token", post(token))
}

pub async fn authorize(params: Query<AuthorizeRequest>, request: Request<Body>) -> impl IntoResponse {
    match request.extensions().get::<UserId>() {
        Some(userid) => {
            let url = AuthorizeService.authorize(params, &userid.0).await.unwrap();
            Redirect::to(&url).into_response()
        },
        None => {
            Redirect::to(&format!("{}?error=Unauthorize", params.redirect_uri)).into_response()
        },
    }
}

pub async fn token(params: Json<TokenRequest>, _request: Request<Body>) -> impl IntoResponse {
    match AuthorizeService.token(&params).await {
        Ok(token) => Json(R::success(token)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

// pub async fn refresh_token(request: Json<TokenRequest>) -> impl IntoResponse {
//     match AuthorizeService.refresh_token(&request).await {
//         Ok(token) => Json(R::success(token)),
//         Err(e) => Json(R::error(&e.to_string())),
//     }
// }
