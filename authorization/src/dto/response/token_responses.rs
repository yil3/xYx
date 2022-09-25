use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct TokenResponses {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: usize,
    pub refresh_token: String,
    pub scope: String,
}
