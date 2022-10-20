use axum::{response::IntoResponse, routing::post, Json, Router};
use x_common::model::response::R;

use crate::{dto::request::client_requests::ClientRequest, service::client_service::ClientService};

pub fn route() -> Router {
    Router::new().route("/save_client", post(save_client))
}

pub async fn save_client(record: Json<ClientRequest>) -> impl IntoResponse {
    match ClientService.save(&record).await {
        Ok(record) => Json(R::success(record)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}
