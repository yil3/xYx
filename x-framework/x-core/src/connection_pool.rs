use sqlx::{Pool, Postgres};
use async_trait::async_trait;

pub type PgConnPool = Pool<Postgres>;

#[async_trait]
pub trait IConnectionManager<T> 
{
    async fn new_pool(connection_string: &str) -> anyhow::Result<T>;
}
