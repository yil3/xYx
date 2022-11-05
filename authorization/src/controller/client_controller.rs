use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use x_common::model::{page::CommonPageRequest, response::R};

use crate::{dto::client_dto::ClientRequest, service::client_service::ClientService};

/**
* @Author xYx
* @Date 2022-09-26 11:08:48
*/

pub fn route() -> Router {
    Router::new()
        .route("/save_client", post(save_client))
        .route("/list", get(page))
        .route("/delete/:id", delete(delete_by_id))
}

pub async fn save_client(record: Json<ClientRequest>) -> impl IntoResponse {
    match ClientService.save(&record).await {
        Ok(record) => Json(R::success(record)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

pub async fn page(param: Query<CommonPageRequest>) -> impl IntoResponse {
    match ClientService.get_page(&param).await {
        Ok(records) => Json(R::success(records)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

pub async fn delete_by_id(Path(id): Path<String>) -> impl IntoResponse {
    match ClientService.delete(&id).await {
        Ok(record) => Json(R::success(record)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}
