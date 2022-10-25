use axum::Router;

use crate::controller::users_controller;

pub fn router() -> Router {
    Router::new()
        .nest("/user", users_controller::route())
}
