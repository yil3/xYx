use crate::{dto::response::client_responses::ClientResponse, entity::client::ClientEntity};
use anyhow::Result;
use x_core::application::POOL;

pub struct ClientRepository;

impl ClientRepository {
    pub async fn insert(&self, record: &ClientEntity) -> Result<ClientResponse> {
        sqlx::query_as!(
            ClientResponse,
            r#"
            INSERT INTO sys_client (id, name, secret, redirect_uri, scope, owner)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, name, secret, redirect_uri, scope, owner, 
            to_char(created_at, 'yyyy-mm-dd hh24:mi:ss') as created_at,
            to_char(updated_at, 'yyyy-mm-dd hh24:mi:ss') as updated_at
            "#,
            record.id,
            record.name,
            record.secret,
            record.redirect_uri,
            record.scope,
            record.owner,
        )
        .fetch_one(&*POOL)
        .await
        .map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn update(&self, record: &ClientEntity) -> Result<ClientResponse> {
        sqlx::query_as!(
            ClientResponse,
            r#"
            UPDATE sys_client
            SET name = coalesce($1, name),
                secret = coalesce($2, secret),
                redirect_uri = coalesce($3, redirect_uri),
                scope = coalesce($4, scope),
                owner = coalesce($5, owner),
                updated_at = now()
            WHERE id = $6
            RETURNING id, name, secret, redirect_uri, scope, owner, 
            to_char(created_at, 'yyyy-MM-dd hh24:mi:ss') created_at,
            to_char(updated_at, 'yyyy-MM-dd hh24:mi:ss') updated_at
            "#,
            record.name,
            record.secret,
            record.redirect_uri,
            record.scope,
            record.owner,
            record.id,
        )
        .fetch_one(&*POOL)
        .await
        .map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn list(&self) -> Result<Vec<ClientResponse>> {
        sqlx::query_as!(
            ClientResponse,
            r#"
            SELECT id, name, secret, redirect_uri, scope, owner, 
            to_char(created_at, 'yyyy-MM-dd hh24:mi:ss') created_at,
            to_char(updated_at, 'yyyy-MM-dd hh24:mi:ss') updated_at
            FROM sys_client
            "#,
        )
        .fetch_all(&*POOL)
        .await
        .map_err(|e| anyhow::anyhow!(e))
    }
}
