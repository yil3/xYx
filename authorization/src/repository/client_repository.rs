use anyhow::Result;
use x_common::model::page::PageParam;
use x_core::application::PG_POOL;

use crate::{dto::client_dto::ClientRecord, entity::client::ClientEntity};

pub struct ClientRepository;

impl ClientRepository {
    pub async fn fetch_by_id(&self, id: &str) -> Result<ClientEntity, sqlx::Error> {
        sqlx::query_as!(
            ClientEntity,
            r#"
            SELECT * FROM sys_client WHERE id = $1
            "#,
            id
        )
        .fetch_one(&*PG_POOL)
        .await
    }

    pub async fn insert(&self, record: &ClientEntity) -> Result<ClientEntity, sqlx::Error> {
        sqlx::query_as!(
            ClientEntity,
            r#"
            INSERT INTO sys_client (id, name, secret, redirect_uri, scope, owner)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
            record.id,
            record.name,
            record.secret,
            record.redirect_uri,
            record.scope,
            record.owner,
        )
        .fetch_one(&*PG_POOL)
        .await
    }

    pub async fn update(&self, record: &ClientEntity) -> Result<ClientEntity, sqlx::Error> {
        sqlx::query_as!(
            ClientEntity,
            r#"
            UPDATE sys_client
            SET name = coalesce($1, name),
                secret = coalesce($2, secret),
                redirect_uri = coalesce($3, redirect_uri),
                scope = coalesce($4, scope),
                owner = coalesce($5, owner),
                updated_at = now()
            WHERE id = $6
            RETURNING *
            "#,
            record.name,
            record.secret,
            record.redirect_uri,
            record.scope,
            record.owner,
            record.id,
        )
        .fetch_one(&*PG_POOL)
        .await
    }

    pub async fn fetch_page(&self, param: &PageParam) -> Result<Vec<ClientRecord>, sqlx::Error> {
        sqlx::query_as!(
            ClientRecord,
            r#"
            SELECT *, count(*) OVER() AS total
            FROM sys_client
            ORDER BY created_at DESC
            limit $1 offset $2
            "#,
            param.limit(),
            param.offset(),
        )
        .fetch_all(&*PG_POOL)
        .await
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM sys_client WHERE id = $1
            "#,
            id
        )
        .execute(&*PG_POOL)
        .await?;
        Ok(())
    }
}
