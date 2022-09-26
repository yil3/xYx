use serde::Serialize;


#[derive(Debug, Serialize, Default)]
pub struct TokenResponses {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: usize,
    pub refresh_token: String,
    pub scope: String,
}

impl TokenResponses {
    pub fn new() -> Self {
        Self {
            access_token: "".to_string(),
            token_type: "".to_string(),
            expires_in: 0,
            refresh_token: "".to_string(),
            scope: "".to_string(),
        }
    }
}
