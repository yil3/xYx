use anyhow::Context;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub type PgConnectionPool = Pool<Postgres>;

pub struct ConnectionManager;

impl ConnectionManager {
    pub async fn new_pool(connection_string: &str) -> anyhow::Result<PgConnectionPool> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_string)
            .await
            .context("error while initializing the database connection pool")?;
        Ok(pool)
    }
}
