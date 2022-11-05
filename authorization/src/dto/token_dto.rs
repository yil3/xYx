use serde::Deserialize;
use serde::Serialize;

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
    pub scope: String,
}
