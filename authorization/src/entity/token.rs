use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use time::OffsetDateTime;
use x_common::utils::{self, code::uuid_v4};

use crate::dto::response::token_responses::TokenResponses;

#[derive(FromRow, Serialize, Deserialize)]
#[serde_as]
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
    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde_as(as = "Rfc3339")]
    pub updated_at: OffsetDateTime,
}
impl Default for TokenEntity {
    fn default() -> Self {
        TokenEntity {
            id: utils::code::unique_id(),
            client_id: Default::default(),
            owner: Default::default(),
            scope: Default::default(),
            access_token: uuid_v4(),
            refresh_token: uuid_v4(),
            token_type: "Bearer".to_string(),
            expires_in: 60 * 60 * 24 * 30,
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

pub struct Claims {
    pub sub: String,
    pub scope: String,
    pub exp: usize,
}
