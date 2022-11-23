use axum::Router;

use crate::handler::users_handler;

pub fn router() -> Router {
    Router::new()
        .nest("/user", users_handler::route())
}
