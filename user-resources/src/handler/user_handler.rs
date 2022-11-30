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
        Err(e) => Json(R::fail(&e.to_string())),
    }
}

pub async fn page(param: Query<PageParam>) -> impl IntoResponse {
    match UserService.get_user_page(&param).await {
        Ok(v) => Json(R::success(v)),
        Err(e) => Json(R::fail(&e.to_string())),
    }
}

pub async fn validate_user(param: Json<LoginUserParam>) -> impl IntoResponse {
    let username = if let Some(username) = &param.username {
        username
    } else {
        return Json(R::fail("username is required"));
    };
    let password = if let Some(password) = &param.password {
        password
    } else {
        return Json(R::fail("password is required"));
    };
    match UserService.validate_user(username, password).await {
        Ok(user_id) => Json(R::success(user_id)),
        Err(e) => Json(R::fail(&e.to_string())),
    }
}

pub async fn add_roles_to_user() {}

pub async fn remove_roles_from_user() {}

pub async fn get_user_info() {}

