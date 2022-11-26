use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use x_common::model::{page::PageParam, response::R};
use x_core::middleware::authentication::CurrentUser;

use crate::{service::client_service::ClientService, vo::client_vo::ClientParam};

/**
* @Author xYx
* @Date 2022-09-26 11:08:48
*/

pub fn route() -> Router {
    Router::new()
        .route("/save_client", post(save_client))
        .route("/page", get(page))
        .route("/delete/:id", delete(delete_by_id))
}

pub async fn save_client(user: CurrentUser, mut record: Json<ClientParam>) -> impl IntoResponse {
    record.owner = Some(user.user_id.to_string());
    match ClientService.save(&record).await {
        Ok(record) => Json(R::success(record)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

pub async fn page(param: Query<PageParam>) -> impl IntoResponse {
    match ClientService.get_page(&param).await {
        Ok(records) => Json(R::success(records)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

pub async fn delete_by_id(id: Path<String>) -> impl IntoResponse {
    match ClientService.delete(&id).await {
        Ok(record) => Json(R::success(record)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}
