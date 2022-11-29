use axum::{response::IntoResponse, extract::Query, Json};
use x_common::model::{page::PageParam, response::R};

use crate::service::token_service::TokenService;



pub async fn page(params: Query<PageParam>) -> impl IntoResponse {
    match TokenService.get_page(&params).await {
        Ok(page) => Json(R::success(page)),
        Err(e) => Json(R::fail(&e.to_string())),
    }
}
