use serde::Serialize;
use x_common::model::page::Pageable;

#[derive(Serialize)]
pub struct UserDto {
    pub id: String,
    pub account: String,
    pub nickname: Option<String>,
    pub origin: Option<String>,
    pub total: Option<i64>,
}

impl Pageable for UserDto {
    fn total(&self) -> i64 {
        self.total.unwrap_or_default()
    }
}

#[derive(Serialize)]
pub struct UserProfileDto {
    pub id: String,
    pub nickname: Option<String>,
    pub total: Option<i64>,
}

impl Pageable for UserProfileDto {
    fn total(&self) -> i64 {
        self.total.unwrap_or_default()
    }
}
