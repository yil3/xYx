use axum::{
    body::Body,
    extract::{Path, Query},
    http::Request,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use x_common::model::{page::PageParam, response::R};

use crate::{dto::client_dto::ClientParam, service::client_service::ClientService};

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

pub async fn save_client(record: Json<ClientParam>, request: Request<Body>) -> impl IntoResponse {
    println!("request: {request:#?}");
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
