use serde::Serialize;
use x_common::model::page::Pageable;



#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TokenRecord {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub scope: Option<String>,
    pub jwt_token: String,
    #[serde(skip)]
    pub total: Option<i64>
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TokenDto {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub scope: Option<String>,
}

impl Pageable for TokenRecord {
    fn total(&self) -> i64 {
        self.total.unwrap_or_default()
    }
}
