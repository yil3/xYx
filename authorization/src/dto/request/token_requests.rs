use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct TokenRequest {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
    pub code: Option<String>,
    pub scope: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}
