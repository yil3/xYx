use time::OffsetDateTime;
use x_common::utils::code::uuid;
use x_core::application::Application;

use crate::dto::token_dto::TokenDto;

/**
* @Author xYx
* @Date 2022-09-26 11:07:22
*/
pub struct Token {
    pub id: String,
    pub client_id: String,
    pub owner: String,
    pub scope: Option<String>,
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i32,
    // pub jwt_token: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
impl Default for Token {
    fn default() -> Self {
        Token {
            id: Default::default(),
            client_id: Default::default(),
            owner: Default::default(),
            scope: Default::default(),
            access_token: uuid(),
            refresh_token: uuid(),
            token_type: "Bearer".to_string(),
            expires_in: Application::config().auth.token_expired.unwrap_or(3600 * 24) as i32,
            // jwt_token: Default::default(),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}

impl Token {
    pub fn into_dto(self) -> TokenDto {
        TokenDto {
            access_token: self.access_token,
            refresh_token: self.refresh_token,
            expires_in: self.expires_in,
            token_type: self.token_type,
            scope: Some(self.scope.unwrap_or_default()),
        }
    }
}

pub struct ScopeEntity {
    pub id: String,
    pub name: String,
    pub description: String,
}
