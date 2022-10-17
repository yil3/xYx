use axum::Router;

use crate::controller::client_controller;

pub fn router() -> Router {
    Router::new().nest("/", client_controller::route())
}
