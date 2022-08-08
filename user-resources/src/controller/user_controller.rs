use axum::Router;



pub struct UserController;

impl UserController {
    pub fn new_router() -> Router {
        Router::new()
    }
}

