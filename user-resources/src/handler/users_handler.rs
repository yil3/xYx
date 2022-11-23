use std::ops::DerefMut;

use axum::{
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use x_common::model::response::R;

use crate::{service::user_service::UserService, vo::user_vo::RegisterUserParam};

pub fn route() -> Router {
    Router::new()
        .route("/register", post(register_user))
        .route("/page", get(fetch_page))
}

pub async fn register_user(mut param: Json<RegisterUserParam>) -> impl IntoResponse {
    match UserService.register(param.deref_mut()).await {
        Ok(output) => Json(R::success(output)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

pub async fn fetch_page() -> impl IntoResponse {
    Json(R::success("ok"))
}
