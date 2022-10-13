use std::ops::Deref;

use anyhow::{Context, Ok};
use authorization::{dto::request::client_requests::ClientRequest, repository::client_repository::ClientRepository};
use axum::{
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use x_core::application::Application;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Application::serve(
        Router::new()
            .route("/authorize", get(test))
            .route("/token", post(test_token)),
    )
    .run()
    .await
    .context("could not initialize application routes")?;
    Ok(())
}

pub async fn test() {
    ClientRepository.test().await;
}

pub async fn test_token(record: Json<ClientRequest>) -> impl IntoResponse {
    let record = record.deref().into_entity();
    ClientRepository.test_inser(record).await;
}
