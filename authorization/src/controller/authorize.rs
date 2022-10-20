use crate::{
    dto::request::{authorize_requests::AuthorizeRequest, token_requests::TokenRequest},
    service::authorize_service::AuthorizeService,
};
use axum::{
    extract::Query,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use x_common::model::response::R;

pub fn route() -> Router {
    Router::new()
        .route("/authorize", get(authorize))
        .route("/token", post(token))
}

pub async fn authorize(request: Query<AuthorizeRequest>) {
    let url = AuthorizeService.authorize(request);
    Redirect::to(&url);
}

pub async fn token(request: Json<TokenRequest>) -> impl IntoResponse {
    match AuthorizeService.token(&request).await {
        Ok(token) => Json(R::success(token)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}
