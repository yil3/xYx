use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use time::OffsetDateTime;
use x_common::utils::code::{unique_id, uuid};
use x_core::application::Application;

use crate::dto::response::token_responses::TokenResponses;

/**
* @Author xYx
* @Date 2022-09-26 11:07:22
*/
#[serde_as]
#[derive(FromRow, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenEntity {
    pub id: String,
    pub client_id: String,
    pub owner: String,
    pub scope: Option<String>,
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub jwt_token: String,
    pub created_at: OffsetDateTime,
    // #[serde_as(as = "Rfc3339")]
    pub updated_at: OffsetDateTime,
}
impl Default for TokenEntity {
    fn default() -> Self {
        TokenEntity {
            id: unique_id(),
            client_id: Default::default(),
            owner: Default::default(),
            scope: Default::default(),
            access_token: uuid(),
            refresh_token: uuid(),
            token_type: "Bearer".to_string(),
            expires_in: Application::config().auth.token_expired.unwrap_or(3600 * 24) as i32,
            jwt_token: Default::default(),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}

impl TokenEntity {
    pub fn into_dto(self) -> TokenResponses {
        TokenResponses {
            access_token: self.access_token,
            refresh_token: self.refresh_token,
            expires_in: self.expires_in,
            token_type: self.token_type,
            scope: self.scope.unwrap_or_default(),
        }
    }
}

pub struct ScopeEntity {
    pub id: String,
    pub name: String,
    pub description: String,
}
