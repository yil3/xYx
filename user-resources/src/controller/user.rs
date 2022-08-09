use axum::{Router, response::IntoResponse, routing::get};

pub struct Controller;

impl Controller {
    pub fn new_router() -> Router {
        Router::new().route("/", get(test))
    }
}

async fn test() -> impl IntoResponse {
    "Hello World".into_response()
}
