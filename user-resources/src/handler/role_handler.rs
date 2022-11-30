use std::collections::HashMap;

use axum::{
    extract::Query,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use x_common::model::{response::R, param::Param};
use x_core::middleware::authentication::CurrentUser;

use crate::{
    service::role_service::RoleService,
    vo::role_vo::{RoleAddUserParam, RoleParam},
};

/**
* @Author xYx
* @Date 2022-11-25 14:26:46
*/

pub fn route() -> Router {
    Router::new()
        .route("/save", post(save_role))
        .route("/tree", get(tree_role))
        .route("/delete", delete(delete_role))
        .route("/list", get(get_roles_by_user_id))
        .route("/signs", get(get_role_sign_by_user_id))
        .route("/add/user", post(insert_user_to_role))
        .route("/remove/user", delete(remove_user_from_role))
}

pub async fn save_role(current_user: CurrentUser, mut record: Json<RoleParam>) -> impl IntoResponse {
    match RoleService.save(&mut record, &current_user.user_id).await {
        Ok(role) => Json(R::success(role)),
        Err(e) => Json(R::fail(&e.to_string())),
    }
}

pub async fn tree_role() -> impl IntoResponse {
    match RoleService.tree().await {
        Ok(role) => Json(R::success(role)),
        Err(e) => Json(R::fail(&e.to_string())),
    }
}

pub async fn delete_role(params: Query<Param>) -> impl IntoResponse {
    if let Some(id) = &params.id {
        match RoleService.delete(&id).await {
            Ok(role) => Json(R::success(role)),
            Err(e) => Json(R::fail(&e.to_string())),
        }
    } else {
        Json(R::fail("id is required"))
    }
}

pub async fn get_roles_by_user_id(params: Query<Param>) -> impl IntoResponse {
    if let Some(user_id) = &params.user_id {
        match RoleService.get_roles_by_user_id(&user_id).await {
            Ok(roles) => Json(R::success(roles)),
            Err(e) => Json(R::fail(&e.to_string())),
        }
    } else {
        Json(R::fail("user_id is required"))
    }
}

pub async fn get_role_sign_by_user_id(params: Query<HashMap<String, String>>) -> impl IntoResponse {
    if let Some(user_id) = params.get("userId") {
        match RoleService.get_role_sign_by_user_id(&user_id).await {
            Ok(roles) => Json(R::success(roles)),
            Err(e) => Json(R::fail(&e.to_string())),
        }
    } else {
        Json(R::fail("userId is required"))
    }
}

pub async fn insert_user_to_role(param: Json<RoleAddUserParam>) -> impl IntoResponse {
    match RoleService.insert_users_to_role(&param.role_id, &param.user_ids).await {
        Ok(roles) => Json(R::success(roles)),
        Err(e) => Json(R::fail(&e.to_string())),
    }
}

pub async fn remove_user_from_role(param: Json<RoleAddUserParam>) -> impl IntoResponse {
    match RoleService
        .remove_users_from_role(&param.role_id, &param.user_ids)
        .await
    {
        Ok(roles) => Json(R::success(roles)),
        Err(e) => Json(R::fail(&e.to_string())),
    }
}
