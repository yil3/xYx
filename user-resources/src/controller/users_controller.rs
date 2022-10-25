use axum::{
    body::Body,
    http::Request,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use x_common::model::response::R;
use x_core::middleware::authorize::UserId;

use crate::{dto::request::users_requests::RegisterUserRequest, service::user_service::UserService};

pub fn route() -> Router {
    Router::new()
        .route("/register", post(register_user))
        .route("/page", get(fetch_page))
}

pub async fn register_user(input: Json<RegisterUserRequest>) -> impl IntoResponse {
    match UserService.register(&input).await {
        Ok(out) => Json(R::success(out)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

pub async fn fetch_page(request: Request<Body>) -> impl IntoResponse {
    let userid = request.extensions().get::<UserId>().unwrap();
    Json(R::success(userid.0.to_owned()))
}
