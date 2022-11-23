use axum::Router;

use crate::handler::{client_handler, authorize_handler};

pub fn router() -> Router {
    Router::new()
        .nest("/authorize", authorize_handler::route())
        .nest("/client", client_handler::route())
}
