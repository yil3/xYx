use serde::Serialize;


#[derive(Debug, Serialize, Default)]
pub struct TokenResponses {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub scope: String,
}

