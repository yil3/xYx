use axum::Router;

use crate::controller::{client_controller, user_controller, authorize};

pub fn router() -> Router {
    Router::new()
        .nest("/", authorize::route())
        .nest("/client", client_controller::route())
        .nest("/user", user_controller::route())
}
