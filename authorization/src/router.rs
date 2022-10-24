use axum::Router;

use crate::controller::{client_controller, user_controller, authorize_controller};

pub fn router() -> Router {
    Router::new()
        .nest("/authorize", authorize_controller::route())
        .nest("/client", client_controller::route())
        .nest("/user", user_controller::route())
}
