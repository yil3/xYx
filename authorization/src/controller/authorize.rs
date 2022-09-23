use axum::{response::{IntoResponse, Redirect}, extract::Query};
use crate::dto::request::authorize_requests::AuthorizeRequest;


pub async fn authorize(req: Query<AuthorizeRequest>) -> impl IntoResponse {
    let mut url = req.redirect_uri.clone();
    if req.client_id.is_empty() {
        url.push_str("?error=invalid_request");
        Redirect::to(&url);
    }
    if req.response_type == "code" {
        url.push_str("?code=");
    }
    if req.state.is_some() {
        url.push_str(format!("&state={}", req.state.as_ref().unwrap()).as_str());
    }
    Redirect::to(&req.redirect_uri);
}
