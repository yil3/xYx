use std::ops::DerefMut;

use axum::{
    extract::Query,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use x_common::model::{page::PageParam, response::R};

use crate::{
    service::user_service::UserService,
    vo::user_vo::{LoginUserParam, RegisterUserParam},
};

pub fn route() -> Router {
    Router::new()
        .route("/register", post(register_user))
        .route("/page", get(page))
        .route("/validate", post(validate_user))
}

pub async fn register_user(mut param: Json<RegisterUserParam>) -> impl IntoResponse {
    match UserService.register(param.deref_mut()).await {
        Ok(id) => Json(R::success(id)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

pub async fn page(param: Query<PageParam>) -> impl IntoResponse {
    match UserService.get_user_page(&param).await {
        Ok(v) => Json(R::success(v)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

pub async fn validate_user(param: Json<LoginUserParam>) -> impl IntoResponse {
    match UserService
        .validate_user(
            param.username.as_ref().expect("username is not empty"),
            param.password.as_ref().expect("password is not empty"),
        )
        .await
    {
        Ok(user_id) => Json(R::success(user_id)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}
