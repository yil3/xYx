use axum::Router;

pub mod user;


pub fn init_router() -> Router {
    Router::new()
        .nest("/user", user::Controller::new_router())
}
