use crate::entity::token::TokenEntity;
use anyhow::Result;
use sqlx::query_as;
use x_core::application::POOL;

pub struct TokenRepository;

impl TokenRepository {
    pub async fn insert(&self, record: TokenEntity) -> Result<TokenEntity> {
        Ok(query_as!(
            TokenEntity,
            r#"
            insert into sys_token 
            (id, owner, access_token, refresh_token, expires_in, scope, token_type) 
            values 
            ($1, $2, $3, $4, $5, $6 ,$7) 
            returning *
            "#,
            record.id,
            record.owner,
            record.access_token,
            record.refresh_token,
            record.expires_in,
            record.scope,
            record.token_type,
        )
        .fetch_one(&*POOL)
        .await?)
    }

    pub fn update(&self) {}

    pub fn delete_by_id(&self, _id: String) {}

    pub fn fetch_page(&self) {}
}
