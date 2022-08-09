use std::env;
use std::future::ready;
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Context;
use axum::error_handling::HandleErrorLayer;
use axum::extract::MatchedPath;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{middleware, BoxError, Json, Router};
use clap::Parser;
use http::{HeaderValue, Method, Request};
use lazy_static::lazy_static;
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder};
use serde_json::json;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use x_core::config::AppConfig;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::controller::user;

lazy_static! {
    static ref EXPONENTIAL_SECONDS: &'static [f64] = &[0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,];
}

pub struct Application;

impl Application {
    pub async fn run() -> anyhow::Result<()> {
        dotenv::dotenv().ok();
        let config = Arc::new(AppConfig::parse());

        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(&config.rust_log))
            .with(tracing_subscriber::fmt::layer())
            .init();

        let recorder_handle = PrometheusBuilder::new()
            .set_buckets_for_metric(
                Matcher::Full(String::from("http_requests_duration_seconds")),
                *EXPONENTIAL_SECONDS,
            )
            .context("could not setup buckets for metrics, verify matchers are correct")?
            .install_recorder()
            .context("could not install metrics recorder")?;

        let router = Router::new()
            .nest("/user", user::Controller::new_router())
            .route("/metrics", get(move || ready(recorder_handle.render())))
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(HandleErrorLayer::new(Self::handle_timeout_error))
                    .timeout(Duration::from_secs(config.http_timeout)),
            )
            .layer(
                CorsLayer::new()
                    .allow_origin(config.cors_origin.parse::<HeaderValue>().unwrap())
                    .allow_methods([Method::GET]),
            )
            .route_layer(middleware::from_fn(Self::track_metrics));

        info!("starting server on port {}", config.user_resources_port);
        axum::Server::bind(&format!("0.0.0.0:{}", config.user_resources_port).parse().unwrap())
            .serve(router.into_make_service())
            .await
            .context("error while starting API server")?;

        Ok(())
    }

    /// Adds a custom handler for tower's `TimeoutLayer`, see https://docs.rs/axum/latest/axum/middleware/index.html#commonly-used-middleware.
    async fn handle_timeout_error(err: BoxError) -> (StatusCode, Json<serde_json::Value>) {
        if err.is::<tower::timeout::error::Elapsed>() {
            (
                StatusCode::REQUEST_TIMEOUT,
                Json(json!({
                    "error":
                        format!(
                            "request took longer than the configured {} second timeout",
                            env::var("HTTP_TIMEOUT").unwrap()
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

    async fn track_metrics<B>(request: Request<B>, next: Next<B>) -> impl IntoResponse {
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

    // async fn ping() -> Json<PingResponse> {
    //     info!("received ping request");
    //     Json(PingResponse::default())
    // }
}
