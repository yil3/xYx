use crate::entity::token::TokenEntity;
use anyhow::Result;
use sqlx::{query_as, query, postgres::PgRow};
use x_core::application::PG_POOL;

pub struct TokenRepository;

impl TokenRepository {
    pub async fn fetch_user_by_account(&self, account: &str) -> Result<PgRow> {
        let record= query(
            r#"
            SELECT id, password FROM sys_user WHERE account = $1
            "#,
        )
        .bind(account)
        .fetch_one(&*PG_POOL)
        .await?;
        Ok(record)
    }
    pub async fn insert(&self, record: TokenEntity) -> Result<TokenEntity> {
        Ok(query_as!(
            TokenEntity,
            r#"
            insert into sys_token 
            (id, owner, access_token, refresh_token, expires_in, scope, token_type, jwt_token, client_id)
            values 
            ($1, $2, $3, $4, $5, $6 ,$7, $8, $9)
            returning *
            "#,
            record.id,
            record.owner,
            record.access_token,
            record.refresh_token,
            record.expires_in,
            record.scope,
            record.token_type,
            record.jwt_token,
            record.client_id
        )
        .fetch_one(&*PG_POOL)
        .await?)
    }

    pub async fn find_by_refresh_token(&self, refresh_token: &str) -> Result<TokenEntity> {
        Ok(query_as!(
            TokenEntity,
            r#"
            select * from sys_token where refresh_token = $1
            "#,
            refresh_token
        )
        .fetch_one(&*PG_POOL)
        .await?)
    }

    pub async fn update_by_id(&self, record: TokenEntity) -> Result<TokenEntity> {
        Ok(query_as!(
            TokenEntity,
            r#"
            update sys_token set 
            access_token = coalesce($1, access_token),
            refresh_token = coalesce($2, refresh_token),
            expires_in = coalesce($3, expires_in),
            scope = coalesce($4, scope),
            token_type = coalesce($5, token_type),
            jwt_token = coalesce($6, jwt_token),
            client_id = coalesce($7, client_id),
            updated_at = now()
            where id = $8
            returning *
            "#,
            record.access_token,
            record.refresh_token,
            record.expires_in,
            record.scope,
            record.token_type,
            record.jwt_token,
            record.client_id,
            record.id
        )
        .fetch_one(&*PG_POOL)
        .await?)
    }

    pub fn delete_by_id(&self, _id: String) {}

    pub fn fetch_page(&self) {}
}
