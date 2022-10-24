use axum::{
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use x_common::model::response::R;

use crate::{dto::request::client_requests::ClientRequest, service::client_service::ClientService};

pub fn route() -> Router {
    Router::new()
        .route("/save_client", post(save_client))
        .route("/list", get(get_list))
        .route("/delete/:id", delete(delete_by_id))
}

pub async fn save_client(record: Json<ClientRequest>) -> impl IntoResponse {
    match ClientService.save(&record).await {
        Ok(record) => Json(R::success(record)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

pub async fn get_list() -> impl IntoResponse {
    match ClientService.get_list().await {
        Ok(record) => Json(R::success(record)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

pub async fn delete_by_id(id: String) -> impl IntoResponse {
    match ClientService.delete(&id).await {
        Ok(record) => Json(R::success(record)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}
