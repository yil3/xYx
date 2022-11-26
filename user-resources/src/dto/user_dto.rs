use serde::Serialize;
use serde_with::serde_as;
use time::OffsetDateTime;
use x_common::model::page::Pageable;
use x_common::utils::date::DateTimeFormat;

#[serde_as]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    pub id: String,
    pub nickname: String,
    pub origin: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub total: Option<i64>,
    #[serde_as(as = "DateTimeFormat")]
    pub created_at: OffsetDateTime,
}

impl Pageable for UserDto {
    fn total(&self) -> i64 {
        self.total.unwrap_or_default()
    }
}

#[derive(Serialize)]
pub struct UserProfileDto {
    pub id: String,
    pub nickname: String,
    pub total: Option<i64>,
}

impl Pageable for UserProfileDto {
    fn total(&self) -> i64 {
        self.total.unwrap_or_default()
    }
}
