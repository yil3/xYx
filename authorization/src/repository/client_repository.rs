use crate::entity::client::ClientEntity;
use anyhow::Result;
use sqlx::postgres::PgQueryResult;
use x_core::application::POOL;

pub struct ClientRepository;

impl ClientRepository {
    pub async fn insert(&self, record: ClientEntity) -> Result<ClientEntity> {
        sqlx::query_as!(
            ClientEntity,
            r#"
            INSERT INTO sys_client (id, name, secret, redirect_uri, scope, owner)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, name, secret, redirect_uri, scope, owner, created_at, updated_at
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
        .execute(&*POOL)
        .await
        .map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn test(&self) {
        let res = sqlx::query_as!(ClientEntity ,"select * from sys_client")
            .fetch_all(&*POOL)
            .await
            .map_err(|e| {
                println!("{:?}", e);
            })
            .unwrap();
        println!("{:#?}", res);
    }

    pub async fn test_inser(&self, record: ClientEntity) {
        let a = sqlx::query_as!(ClientEntity,
            r#"
            INSERT INTO sys_client (id, name, secret, redirect_uri, scope, owner)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, name, secret, redirect_uri, scope, owner, created_at, updated_at
            "#,
            record.id,
            record.name,
            record.secret,
            record.redirect_uri,
            record.scope,
            record.owner,
        )
        .fetch_one(&*POOL)
        .await;
        println!("{a:#?}");
    }
}
