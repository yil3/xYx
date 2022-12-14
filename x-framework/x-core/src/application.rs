use std::env;

use std::net::SocketAddr;
use std::sync::Arc;

use crate::handlers::http_time_out::handle_timeout_error;
use crate::middleware::authentication::CurrentUser;
// use crate::middleware::functions::metrics::track_metrics;
use axum::error_handling::HandleErrorLayer;
use axum::extract::Extension;
use axum::middleware::from_extractor;
use axum::Router;
use http::{HeaderValue, Method};
use lazy_static::lazy_static;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::time::Duration;
use time::{macros::format_description, UtcOffset};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::debug;
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, fmt::time::OffsetTime, EnvFilter};

use crate::config::AppConfig;

lazy_static! {
    static ref APP_CONFIG: Arc<AppConfig> = Arc::new(AppConfig::parse());
    pub static ref PG_POOL: Pool<Postgres> = Application::create_pool();
    pub static ref REDIS_CLIENT: redis::Client = Application::create_redis_client();
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
        debug!(
            "Starting server bind on {} listening port {}",
            self.config.server.host, self.config.server.port
        );
        let addr = format!("{}:{}", self.config.server.host, self.config.server.port)
            .parse::<SocketAddr>()
            .map_err(|_| anyhow::anyhow!("Invalid server address"))?;
        axum::Server::bind(&addr)
            .serve(self.router.into_make_service())
            .await
            .unwrap();
        Ok(())
    }

    fn init_env(&self) {
        if let Some(uri) = Application::generate_db_uri() {
            env::set_var("DATABASE_URL", uri.get(1).unwrap());
        } else {
            panic!("Please check the database configuration");
        }
        env::set_var("ARGON_SALT", self.config.auth.argon_salt.as_str());
        env::set_var("TOKEN_SECRET", self.config.auth.token_secret.as_str());
        env::set_var(
            "TOKEN_EXPIRED",
            self.config.auth.token_expired.unwrap().to_string().as_str(),
        );
    }

    pub fn config() -> Arc<AppConfig> {
        APP_CONFIG.to_owned()
    }

    pub fn init_log(&mut self) {
        let exe = std::env::current_exe()
            .as_ref()
            .map(|p| p.file_name().unwrap().to_str().unwrap())
            .unwrap_or_default()
            .to_owned();
        let debug_log = rolling::daily("logs", String::from(&exe) + "_debug");
        let error_log = rolling::daily("logs", String::from(&exe) + "_error").with_max_level(tracing::Level::ERROR);
        let local_time = OffsetTime::new(
            UtcOffset::from_hms(8, 0, 0).unwrap(),
            format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
        );
        tracing_subscriber::registry()
            .with(
                fmt::layer()
                    .with_writer(debug_log.and(error_log))
                    .with_ansi(false)
                    .with_timer(local_time.to_owned()),
            )
            .with(fmt::layer().with_timer(local_time))
            .with(EnvFilter::new(&self.config.log.level))
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

    fn create_redis_client() -> redis::Client {
        let config = APP_CONFIG.redis.to_owned().unwrap();
        let uri = format!(
            "redis://{}:{}@{}:{}/{}",
            config.username.unwrap_or(String::from("")),
            config.password.unwrap_or(String::from("")),
            config.host,
            config.port,
            config.db.unwrap_or(0)
        );
        debug!("Creating redis client: {}", &uri);
        redis::Client::open(uri.as_str()).unwrap()
    }

    fn create_pool() -> Pool<Postgres> {
        debug!("Creating database connection pool");
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
        PG_POOL.to_owned()
    }

    pub fn redis() -> redis::Connection {
        REDIS_CLIENT.to_owned().get_connection().unwrap()
    }

    pub fn init_router(&mut self) {
        debug!("init router...");
        self.router = self
            .router
            .to_owned()
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(HandleErrorLayer::new(handle_timeout_error))
                    .timeout(Duration::from_secs(self.config.server.http_timeout)),
            )
            .layer(Extension(self.config.as_ref().clone()))
            .layer(Extension(Application::pgpool()));
        if self.config.auth.status {
            debug!("init auth middleware...");
            self.router = self.router.to_owned().layer(from_extractor::<CurrentUser>())
            // .layer(AsyncRequireAuthorizationLayer::new(XAuthorize));
        }
        if self.config.server.cors.status {
            debug!("init cors middleware...");
            let mut origins = vec![];
            for ele in self.config.server.cors.origins.to_owned().into_iter() {
                origins.push(ele.parse::<HeaderValue>().unwrap());
            }
            self.router = self
                .router
                .to_owned()
                .layer(CorsLayer::new().allow_origin(origins).allow_methods([Method::GET]));
        }
        // self.router = self.router.to_owned().route_layer(middleware::from_fn(track_metrics));
    }
}
