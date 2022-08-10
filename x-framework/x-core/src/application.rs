use std::env;
use std::future::ready;
use std::path::Path;
use std::sync::Arc;

use crate::layer::{handle_timeout_error, track_metrics};
use anyhow::{Context, Ok};
use axum::error_handling::HandleErrorLayer;
use axum::middleware;
use axum::{routing::get, Router};
use clap::Parser;
use http::{HeaderValue, Method};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::config::AppConfig;
use crate::constant::EXPONENTIAL_SECONDS;

pub struct Application {
    pub router: Router,
}

impl Application {
    pub fn serve(router: Router) -> Self {
        Self { router }
    }
    pub async fn run(self) -> anyhow::Result<()> {
        Self::init_env();
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

        let router = self
            .router
            .route("/metrics", get(move || ready(recorder_handle.render())))
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(HandleErrorLayer::new(handle_timeout_error))
                    .timeout(Duration::from_secs(config.http_timeout)),
            )
            .layer(
                CorsLayer::new()
                    .allow_origin(config.cors_origin.parse::<HeaderValue>().unwrap())
                    .allow_methods([Method::GET]),
            )
            .route_layer(middleware::from_fn(track_metrics));

        info!("starting server on port {}", config.port);
        axum::Server::bind(&format!("0.0.0.0:{}", config.port).parse().unwrap())
            .serve(router.into_make_service())
            .await
            .unwrap();
        Ok(())
    }

    fn init_env() {
        let current_bin = env::current_exe().unwrap();
        let current_dir = env::current_dir().unwrap();
        let bin_dir_vec: Vec<&str> = current_bin.to_str().unwrap().split("/").collect();
        let mut path_vec: Vec<&str> = current_dir.to_str().unwrap().split("/").collect();
        if path_vec.last() != bin_dir_vec.last() {
            path_vec.push(bin_dir_vec.last().unwrap());
        }
        path_vec.push("application.properties");
        let path = path_vec.join("/").to_string();
        dotenv::from_path(Path::new(&path)).ok();
        dotenv::dotenv().ok();
    }
}
