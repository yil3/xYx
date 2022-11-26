use axum::{
    extract::Query,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use x_common::model::response::R;
use x_core::middleware::authentication::CurrentUser;

use crate::{
    service::permission_service::PermissionService,
    vo::permission_vo::{PermissionParam, PermissionTypeParam},
};

/**
* @Author xYx
* @Date 2022-11-25 15:19:22
*/

pub fn route() -> Router {
    Router::new()
        .route("/save", post(save_permission))
        .route("/delete", delete(delete_permission))
        .route("/list", get(get_permission_by_role))
        .route("/signs", get(get_permission_sign_by_user))
        .route("/type/save", post(save_permission_type))
        .route("/type/delete", delete(delete_permission_type))
        .route("/type/list", get(get_permission_type))
}

pub async fn save_permission(user: CurrentUser, mut body: Json<PermissionParam>) -> impl IntoResponse {
    match PermissionService.save(&mut body, &user.user_id).await {
        Ok(record) => Json(R::success(record)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

pub async fn delete_permission(id: Query<String>) -> impl IntoResponse {
    match PermissionService.delete_by_id(&id).await {
        Ok(record) => Json(R::success(record)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

pub async fn get_permission_by_role(role_id: Query<String>) -> impl IntoResponse {
    match PermissionService.get_by_role(&role_id).await {
        Ok(record) => Json(R::success(record)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

pub async fn get_permission_sign_by_user(user_id: Query<String>) -> impl IntoResponse {
    match PermissionService.get_permission_sign_by_user(&user_id).await {
        Ok(record) => Json(R::success(record)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

pub async fn get_permission_type() -> impl IntoResponse {
    match PermissionService.get_permission_type().await {
        Ok(record) => Json(R::success(record)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

pub async fn save_permission_type(body: Json<PermissionTypeParam>) -> impl IntoResponse {
    match PermissionService.save_permission_type(&body).await {
        Ok(data) => Json(R::success(data)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

pub async fn delete_permission_type(id: Query<String>) -> impl IntoResponse {
    match PermissionService.delete_permission_type(&id).await {
        Ok(data) => Json(R::success(data)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}
