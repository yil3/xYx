use std::collections::HashMap;

use axum::{
    extract::Query,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use x_common::model::{page::PageParam, response::R};
use x_core::middleware::authentication::CurrentUser;

use crate::{service::role_group_service::RoleGroupService, vo::role_group_vo::RoleGroupParam};

/**
* @Author xYx
* @Date 2022-12-03 17:04:04
*/
pub fn route() -> Router {
    Router::new()
        .route("/save", post(save_role_group))
        .route("/delete", delete(delete_role_group))
        .route("/page", get(page_role_group))
}

pub async fn save_role_group(user: CurrentUser, record: Json<RoleGroupParam>) -> impl IntoResponse {
    match RoleGroupService.save(&record, &user.user_id).await {
        Ok(data) => Json(R::success(data)),
        Err(e) => Json(R::fail(&e.to_string())),
    }
}

pub async fn delete_role_group(param: Query<HashMap<String, String>>) -> impl IntoResponse {
    if let Some(id) = param.get("id") {
        match RoleGroupService.delete(id).await {
            Ok(data) => Json(R::success(data)),
            Err(e) => Json(R::fail(&e.to_string())),
        }
    } else {
        Json(R::fail("id is required"))
    }
}

pub async fn page_role_group(param: Query<PageParam>) -> impl IntoResponse {
    match RoleGroupService.get_page(&param).await {
        Ok(data) => Json(R::success(data)),
        Err(e) => Json(R::fail(&e.to_string())),
    }
}
