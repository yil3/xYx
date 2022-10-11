use crate::entity::client::ClientEntity;
use anyhow::Result;
use sqlx::postgres::PgQueryResult;
use x_core::constant::POOL;

pub struct ClientRepository;

impl ClientRepository {
    pub async fn insert(record: ClientEntity) -> Result<PgQueryResult> {
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
        .execute(&*POOL)
        .await
        .map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn update(record: ClientEntity) -> Result<PgQueryResult> {
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
        .execute(&*POOL)
        .await
        .map_err(|e| anyhow::anyhow!(e))
    }

    pub fn delete_by_id(_id: String) {}

    pub fn fetch_page() {}
}
