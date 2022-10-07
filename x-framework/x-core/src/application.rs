use std::env;

use std::path::Path;
use std::sync::Arc;

use crate::handlers::http_time_out::handle_timeout_error;
use crate::middleware::authorize::XAuthorize;
use crate::middleware::functions::metrics::track_metrics;
use axum::error_handling::HandleErrorLayer;
use axum::middleware;
use axum::Router;
use clap::Parser;
use http::{HeaderValue, Method};
use tower_http::auth::AsyncRequireAuthorizationLayer;
// use tower_http::auth::AsyncRequireAuthorizationLayer;
// use axum::response::Redirect;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::config::AppConfig;

pub struct Application {
    pub router: Router,
    pub config: Arc<AppConfig>,
}

impl Application {
    pub fn serve(router: Router) -> Self {
        Self {
            router,
            config: Arc::new(AppConfig::default()),
        }
    }
    pub async fn run(mut self) -> anyhow::Result<()> {
        self.init_env();
        self.init_config();
        self.init_router();
        info!("starting server on port {}", self.config.port);
        axum::Server::bind(&format!("0.0.0.0:{}", self.config.port).parse().unwrap())
            .serve(self.router.into_make_service())
            .await
            .unwrap();
        Ok(())
    }

    fn init_env(&self) {
        let current_bin = env::current_exe().unwrap();
        let current_dir = env::current_dir().unwrap();
        let bin_name = current_bin.to_str().unwrap().split("/").last().unwrap();
        let last_dir = current_dir.to_str().unwrap().split("/").last().unwrap();
        let mut path = String::from(current_dir.to_str().unwrap());
        if bin_name != last_dir {
            path.push_str("/");
            path.push_str(bin_name);
        }
        path.push_str("/application.env");
        let path = Path::new(&path);
        dotenv::from_path(Path::new(path)).ok();
        dotenv::dotenv().ok();
    }

    pub fn init_config(&mut self) {
        self.config = Arc::new(AppConfig::parse());
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(&self.config.rust_log))
            .with(tracing_subscriber::fmt::layer())
            .init();
    }

    pub fn init_router(&mut self) {
        self.router = self
            .router
            .to_owned()
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(HandleErrorLayer::new(handle_timeout_error))
                    .timeout(Duration::from_secs(self.config.http_timeout)),
            )
            .layer(
                CorsLayer::new()
                    .allow_origin(self.config.cors_origin.parse::<HeaderValue>().unwrap())
                    .allow_methods([Method::GET]),
            )
            .layer(AsyncRequireAuthorizationLayer::new(XAuthorize))
            .route_layer(middleware::from_fn(track_metrics));
    }
}
