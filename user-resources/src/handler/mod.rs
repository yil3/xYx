pub mod user_handler;
pub mod user_group_handler;
pub mod role_handler;
pub mod permission_handler;



// use std::future::ready;
// use axum::{routing::get, Router};
// use x_core::handlers::metrics::metrics_handle;

// pub fn init_router() -> Router {
//     Router::new()
//         .route("/metrics", get(move || ready(metrics_handle())))
// }
