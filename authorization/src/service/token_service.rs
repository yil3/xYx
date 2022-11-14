use crate::dto::token_dto::TokenRecord;
use crate::{entity::token::TokenEntity, repository::token_repository::TokenRepository};
use anyhow::anyhow;
use anyhow::Result;
use redis::Commands;
use x_common::model::page::Page;
use x_common::model::page::PageParam;
use x_common::utils::token::TokenUtils;
use x_core::application::Application;

pub struct TokenService;

impl TokenService {
    pub async fn generate_token(&self, client_id: &str, user_id: &str, scope: &Option<String>) -> Result<TokenEntity> {
        let mut record = TokenEntity::default();
        record.client_id = client_id.to_owned();
        record.owner = user_id.to_owned();
        record.scope = scope.to_owned();
        let jwt_token = TokenUtils::generate_jwt_token(user_id.to_string(), "")?;
        record.jwt_token = jwt_token;
        Application::redis().set_ex(&record.access_token, &record.jwt_token, record.expires_in as usize)?;
        Ok(TokenRepository.insert(record).await?)
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<TokenEntity> {
        let mut record = TokenRepository
            .find_by_refresh_token(refresh_token)
            .await
            .map_err(|_| anyhow!("refresh_token is invalid"))?;
        record.expires_in = Application::config().auth.token_expired.unwrap_or(3600 * 24) as i32;
        record.jwt_token = TokenUtils::generate_jwt_token(record.owner.to_string(), "")?;
        Application::redis().set_ex(&record.access_token, &record.jwt_token, record.expires_in as usize)?;
        Ok(TokenRepository.update_by_id(record).await?)
    }

    pub async fn remove_expired_token(&self) {
        TokenRepository.remove_expired_token().await.expect("remove_expired_token error");
    }
    pub async fn get_page(&self, params: &PageParam) -> Result<Page<TokenRecord>> {
        Ok(Page::build(
            params.page,
            params.size,
            TokenRepository::fetch_page(&TokenRepository, params).await?,
        ))
    }
}
