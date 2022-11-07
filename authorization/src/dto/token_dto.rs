use serde::Deserialize;
use serde::Serialize;
use x_common::model::page::Pageable;

#[derive(Deserialize)]
pub struct TokenParam {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
    pub code: Option<String>,
    pub scope: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize)]
pub struct TokenRefreshParam {
    pub refresh_token: String,
}

#[derive(Serialize, Default)]
pub struct TokenRecord {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub scope: Option<String>,
    #[serde(skip)]
    pub total: Option<i64>
}

impl Pageable for TokenRecord {
    fn total(&self) -> i64 {
        self.total.unwrap_or_default()
    }
}
