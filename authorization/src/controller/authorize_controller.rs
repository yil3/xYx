use crate::{
    dto::{
        authorize_dto::AuthorizeParam,
        token_dto::{TokenParam, TokenRefreshParam},
    },
    service::authorize_service::AuthorizeService,
};
use axum::{
    extract::Query,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use x_common::model::response::R;
use x_core::middleware::authentication::CurrentUser;

/**
* @Author xYx
* @Date 2022-09-26 11:08:19
*/

pub fn route() -> Router {
    Router::new()
        .route("/", get(authorize))
        .route("/token", post(token))
        .route("/refresh", post(refresh_token))
}

pub async fn authorize(params: Query<AuthorizeParam>, user: CurrentUser) -> impl IntoResponse {
    let url = AuthorizeService.authorize(params, user.id()).await.unwrap();
    Redirect::to(&url).into_response()
}

pub async fn token(params: Json<TokenParam>) -> impl IntoResponse {
    match AuthorizeService.token(&params).await {
        Ok(token) => Json(R::success(token)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}

pub async fn refresh_token(request: Json<TokenRefreshParam>) -> impl IntoResponse {
    match AuthorizeService.refresh_token(&request.refresh_token).await {
        Ok(token) => Json(R::success(token)),
        Err(e) => Json(R::error(&e.to_string())),
    }
}
