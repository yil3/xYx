use axum::{
    extract::Query,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use x_common::model::{page::PageParam, response::R};
use x_core::middleware::authentication::CurrentUser;

use crate::{
    service::user_group_service::UserGroupService,
    vo::user_group_vo::{RoleUserGroupParam, UserGroupParam, UserUserGroupParam},
};

/**
* @Author xYx
* @Date 2022-11-25 11:07:09
*/
pub fn route() -> Router {
    Router::new()
        .route("/save", post(save_user_group))
        .route("/delete", delete(delete_user_group))
        .route("/page", get(get_user_group_page))
        .route("/user/page", get(get_user_page_by_group_id))
        .route("/user/add", post(add_users_to_user_group))
        .route("/user/remove", delete(remove_users_from_user_group))
        .route("/role/add", post(add_roles_to_user_group))
        .route("/role/remove", delete(remove_roles_from_user_group))
}

pub async fn save_user_group(mut param: Json<UserGroupParam>, current_user: CurrentUser) -> impl IntoResponse {
    match UserGroupService.save(&mut param, &current_user.user_id).await {
        Ok(user_group) => Json(R::success(user_group)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

pub async fn delete_user_group(id: Query<String>) -> impl IntoResponse {
    match UserGroupService.delete(&id).await {
        Ok(_) => Json(R::success(())),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

pub async fn get_user_group_page(param: Query<PageParam>) -> impl IntoResponse {
    match UserGroupService.get_user_group_page(&param).await {
        Ok(user_groups) => Json(R::success(user_groups)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

pub async fn get_user_page_by_group_id(param: Query<PageParam>) -> impl IntoResponse {
    match UserGroupService.get_user_page_by_group_id(&param).await {
        Ok(page) => Json(R::success(page)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

pub async fn add_users_to_user_group(param: Json<UserUserGroupParam>) -> impl IntoResponse {
    match UserGroupService
        .insert_users_by_group_id(&param.user_group_id, &param.user_ids)
        .await
    {
        Ok(_) => Json(R::success(())),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

pub async fn remove_users_from_user_group(param: Json<UserUserGroupParam>) -> impl IntoResponse {
    match UserGroupService
        .remove_users_by_group_id(&param.user_group_id, &param.user_ids)
        .await
    {
        Ok(_) => Json(R::success(())),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

pub async fn add_roles_to_user_group(param: Json<RoleUserGroupParam>) -> impl IntoResponse {
    match UserGroupService
        .insert_role_by_user_group(&param.user_group_id, &param.role_ids)
        .await
    {
        Ok(count) => Json(R::success(count)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

pub async fn remove_roles_from_user_group(param: Json<RoleUserGroupParam>) -> impl IntoResponse {
    match UserGroupService
        .remove_role_from_user_group(&param.user_group_id, &param.role_ids)
        .await
    {
        Ok(count) => Json(R::success(count)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}
