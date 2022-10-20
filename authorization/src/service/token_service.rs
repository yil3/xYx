use crate::{repository::token_repository::TokenRepository, entity::token::TokenEntity};
use anyhow::Result;
use x_common::utils::token::TokenUtils;


pub struct TokenService;

impl TokenService {
    pub async fn generate_token(&self, client_id: &str, user_id: &str, scope: &Option<String>) -> Result<TokenEntity> {
        let mut record = TokenEntity::default();
        record.client_id = client_id.to_owned();
        record.owner = user_id.to_owned();
        record.scope = scope.to_owned();
        let jwt_token = TokenUtils::generate_jwt_token(user_id.to_string(), "")?;
        record.jwt_token = jwt_token;
        TokenRepository.insert(record).await
    }
}
