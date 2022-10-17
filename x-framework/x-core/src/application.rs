use std::env;

use std::sync::Arc;

use crate::handlers::http_time_out::handle_timeout_error;
use crate::middleware::functions::metrics::track_metrics;
use axum::error_handling::HandleErrorLayer;
use axum::extract::Extension;
use axum::middleware;
use axum::Router;
// use http::{HeaderValue, Method};
use lazy_static::lazy_static;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::time::Duration;
use tower::ServiceBuilder;
// use tower_http::auth::AsyncRequireAuthorizationLayer;
// use crate::middleware::authorize::XAuthorize;
// use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::config::AppConfig;

lazy_static! {
    static ref APP_CONFIG: Arc<AppConfig> = Arc::new(AppConfig::parse());
    pub static ref POOL: Pool<Postgres> = Application::create_pool();
}

pub struct Application {
    pub config: Arc<AppConfig>,
    pub router: Router,
}

impl Application {
    pub fn serve(router: Router) -> Self {
        Self {
            router,
            config: APP_CONFIG.clone(),
        }
    }
    pub async fn run(mut self) -> anyhow::Result<()> {
        self.init_env();
        self.init_log();
        self.init_router();
        info!(
            "Starting server bind on {} listening port {}",
            self.config.server.host, self.config.server.port
        );
        axum::Server::bind(
            &format!("{}:{}", self.config.server.host, self.config.server.port)
                .parse()
                .unwrap(),
        )
        .serve(self.router.into_make_service())
        .await?;
        Ok(())
    }

    fn init_env(&self) {
        info!("init env");
        if let Some(uri) = Application::generate_db_uri() {
            env::set_var("DATABASE_URL", uri.get(1).unwrap());
        } else {
            panic!("Please check the database configuration");
        }
    }

    pub fn config() -> Arc<AppConfig> {
        APP_CONFIG.clone()
    }

    pub fn init_log(&mut self) {
        info!("init log");
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(&self.config.log.level))
            .with(tracing_subscriber::fmt::layer())
            .init();
    }

    fn generate_db_uri() -> Option<Vec<String>> {
        let config = APP_CONFIG.clone();
        let uri = match APP_CONFIG.clone().database.category.as_str() {
            "postgres" => {
                format!(
                    "postgres://{}:{}@{}:{}/{}",
                    config.database.username,
                    config.database.password,
                    config.database.host,
                    config.database.port,
                    config.database.db_name
                )
            },
            _ => {
                return None;
            },
        };
        Some(vec![config.database.category.clone(), uri])
    }

    fn create_pool() -> Pool<Postgres> {
        if let Some(uri) = Application::generate_db_uri() {
            let key = uri.get(0).unwrap().clone();
            match key.as_str() {
                "postgres" => {
                    let pool = PgPoolOptions::new()
                        .max_connections(5)
                        .connect_lazy(&uri.get(1).unwrap())
                        .expect("could not initialize the database connection pool");
                    pool
                },
                _ => {
                    panic!("Database type not supported");
                },
            }
        } else {
            panic!("Please check the database configuration");
        }
    }

    pub fn pgpool() -> Pool<Postgres> {
        POOL.to_owned()
    }

    pub fn init_router(&mut self) {
        info!("init router");
        self.router = self
            .router
            .to_owned()
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(HandleErrorLayer::new(handle_timeout_error))
                    .timeout(Duration::from_secs(self.config.server.http_timeout)),
            )
            // .layer(
            //     CorsLayer::new()
            //         .allow_origin(self.config.server.cors_origin.parse::<HeaderValue>().unwrap())
            //         .allow_methods([Method::GET]),
            // )
            .layer(Extension(self.config.as_ref().clone()))
            .layer(Extension(Application::pgpool()))
            // .layer(AsyncRequireAuthorizationLayer::new(XAuthorize))
            .route_layer(middleware::from_fn(track_metrics));
    }
}
