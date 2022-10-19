use axum::Router;

use crate::controller::{client_controller, user_controller};

pub fn router() -> Router {
    Router::new()
        .nest("/client", client_controller::route())
        .nest("/user", user_controller::route())
}
