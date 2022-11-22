use serde::Serialize;

#[derive(Serialize)]
pub struct UserDto {
    pub id: String,
    pub account: String,
    pub nickname: Option<String>,
    pub origin: Option<String>,
    #[serde(skip)]
    pub total: Option<i64>,
}
