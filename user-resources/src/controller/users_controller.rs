use std::ops::DerefMut;

use axum::{
    body::Body,
    http::Request,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use x_common::model::response::R;
use x_core::middleware::authorize::UserId;

use crate::{dto::user_dto::RegisterUserParam, service::user_service::UserService};

pub fn route() -> Router {
    Router::new()
        .route("/register", post(register_user))
        .route("/page", get(fetch_page))
}

pub async fn register_user(mut input: Json<RegisterUserParam>) -> impl IntoResponse {
    match UserService.register(input.deref_mut()).await {
        Ok(output) => Json(R::success(output)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

pub async fn fetch_page(request: Request<Body>) -> impl IntoResponse {
    let userid = request.extensions().get::<UserId>().unwrap();
    Json(R::success(userid.0.to_owned()))
}
