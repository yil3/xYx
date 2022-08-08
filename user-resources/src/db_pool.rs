use anyhow::Context;
use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;
use x_core::connection_pool::{IConnectionManager, PgConnPool};


pub struct PgPool;

#[async_trait]
impl IConnectionManager<PgConnPool> for PgPool {
    async fn new_pool(connection_string: &str) -> anyhow::Result<PgConnPool> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_string)
            .await
            .context("error while initializing the database connection pool")?;
        Ok(pool)
    }
}
