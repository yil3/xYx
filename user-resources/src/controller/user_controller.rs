use axum::Router;

use crate::service_register::ServiceRegister;

pub struct UserController;

impl UserController {
    pub fn new_router(service_register: ServiceRegister) -> Router {
        Router::new()
    }
}
