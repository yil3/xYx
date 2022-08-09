use async_trait::async_trait;
use axum::{Router, BoxError, Json};
use http::StatusCode;
use x_common::models::ping::PingResponse;


#[async_trait]
pub trait IApplication {
    async fn run();
    fn init_router() -> Router;
    async fn handle_timeout_error(err: BoxError) -> (StatusCode, Json<serde_json::Value>);
    async fn ping() -> Json<PingResponse>;
}
