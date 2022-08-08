use std::sync::Arc;

use anyhow::Context;
use clap::Parser;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use user_resources::{application::Application, db_pool::ConnectionPool, service_register::ServiceRegister};
use x_core::{config::AppConfig, connection_pool::IConnectionManager};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let config = Arc::new(AppConfig::parse());

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&config.rust_log))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("environment loaded and configuration parsed, initializing Postgres connection and running migrations...");
    let pg_pool = ConnectionPool::new_pool(&config.database_url)
        .await
        .expect("could not initialize the database connection pool");

    let service_register = ServiceRegister::new(pg_pool, config.clone());

    info!("initializing axum server...");
    Application::serve(config.port, &config.cors_origin, service_register)
        .await
        .context("could not initialize application routes")?;

    Ok(())
}
