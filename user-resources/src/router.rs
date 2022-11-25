use axum::Router;

use crate::handler::{permission_handler, role_handler, user_group_handler, user_handler};

pub fn router() -> Router {
    Router::new()
        .nest("/user", user_handler::route())
        .nest("/user_group", user_group_handler::route())
        .nest("/role", role_handler::route())
        .nest("/permission", permission_handler::route())
}
