use axum::{response::{Redirect, IntoResponse}, extract::Query, Json};
use x_common::model::response::R;
use crate::{dto::request::{authorize_requests::AuthorizeRequest, token_requests::TokenRequest}, service::authorize_service::AuthorizeService};


pub async fn authorize(request: Query<AuthorizeRequest>) {
    let url = AuthorizeService.authorize(request);
    Redirect::to(&url);
}

pub async fn token(request: Json<TokenRequest>) -> impl IntoResponse{
    let token = AuthorizeService.token(request);
    Json(R::success(token))
}
