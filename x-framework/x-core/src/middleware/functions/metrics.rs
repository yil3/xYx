use std::time::Instant;

use axum::{middleware::Next, response::IntoResponse, extract::MatchedPath};
use http::Request;


pub async fn track_metrics<B>(request: Request<B>, next: Next<B>) -> impl IntoResponse {
    let path = if let Some(matched_path) = request.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        request.uri().path().to_owned()
    };

    let start = Instant::now();
    let method = request.method().clone();
    let response = next.run(request).await;
    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    let labels = [("method", method.to_string()), ("path", path), ("status", status)];

    metrics::increment_counter!("http_requests_total", &labels);
    metrics::histogram!("http_requests_duration_seconds", latency, &labels);

    response
}
