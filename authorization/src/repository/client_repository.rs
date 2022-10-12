use std::sync::Arc;

use crate::entity::client::ClientEntity;
use anyhow::Result;
use sqlx::{postgres::PgQueryResult, Pool, Postgres};
// use x_core::constant::POOL;

pub struct ClientRepository {
    pub pool: Pool<Postgres>
}

impl ClientRepository {
    pub fn new(pool: Pool<Postgres>) -> Arc<Self> {
        Arc::new(Self { pool })
    }
}

impl ClientRepository {
    pub async fn insert(&self, record: ClientEntity) -> Result<PgQueryResult> {
        sqlx::query!(
            r#"
            INSERT INTO sys_client (id, name, secret, redirect_uri, scope, owner)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            record.id,
            record.name,
            record.secret,
            record.redirect_uri,
            record.scope,
            record.owner,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn update(&self, record: ClientEntity) -> Result<PgQueryResult> {
        sqlx::query!(
            r#"
            UPDATE sys_client
            SET name = coalesce($1, name),
                secret = coalesce($2, secret),
                redirect_uri = coalesce($3, redirect_uri),
                scope = coalesce($4, scope),
                owner = coalesce($5, owner)
            WHERE id = $6
            "#,
            record.name,
            record.secret,
            record.redirect_uri,
            record.scope,
            record.owner,
            record.id,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!(e))
    }
}
