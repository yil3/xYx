use axum::{BoxError, Json};
use http::StatusCode;
use serde_json::json;

pub async fn handle_timeout_error(err: BoxError) -> (StatusCode, Json<serde_json::Value>) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            Json(json!({
                "error":
                    format!(
                        "request took longer than the configured {} second timeout",
                        std::env::var("HTTP_TIMEOUT").unwrap_or_else(|_| "30".to_string())
                    )
            })),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("unhandled internal error: {}", err) })),
        )
    }
}
