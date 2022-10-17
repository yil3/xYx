use axum::{response::IntoResponse, routing::post, Json, Router};
use x_common::model::response::R;

use crate::{dto::request::client_requests::ClientRequest, repository::client_repository::ClientRepository};

pub fn route() -> Router {
    Router::new().route("/save_client", post(save_client))
}

pub async fn save_client(record: Json<ClientRequest>) -> impl IntoResponse {
    match record.id {
        Some(_) => match ClientRepository.update(&record.into_entity()).await {
            Ok(client) => Json(R::success(client)),
            Err(e) => Json(R::fail(&e.to_string())),
        },
        None => match ClientRepository.insert(&record.into_entity()).await {
            Ok(client) => Json(R::success(client)),
            Err(e) => Json(R::fail(&e.to_string())),
        },
    }
}
