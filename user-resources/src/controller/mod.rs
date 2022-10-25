use std::future::ready;

use axum::{routing::get, Router};
use x_core::handlers::metrics::metrics_handle;

pub mod users_controller;

pub fn init_router() -> Router {
    Router::new()
        .route("/metrics", get(move || ready(metrics_handle())))
}
