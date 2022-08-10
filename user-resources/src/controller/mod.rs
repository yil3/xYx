use std::future::ready;

use axum::{routing::get, Router};
use x_core::handlers::metrics::metrics_handle;

pub mod user;

pub fn init_router() -> Router {
    Router::new()
        .nest("/user", user::Controller::new_router())
        .route("/metrics", get(move || ready(metrics_handle())))
}
