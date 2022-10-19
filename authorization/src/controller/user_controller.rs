use axum::{response::IntoResponse, routing::post, Json, Router};
use x_common::model::response::R;

use crate::{dto::request::user_requests::RegisterUserRequest, service::user_service::UserService};

pub fn route() -> Router {
    Router::new().route("/register", post(register_user))
}

pub async fn register_user(input: Json<RegisterUserRequest>) -> impl IntoResponse {
    match UserService.register(&input).await {
        Ok(out) => Json(R::success(out)),
        Err(e) => Json(R::fail(&e.to_string())),
    }
}
