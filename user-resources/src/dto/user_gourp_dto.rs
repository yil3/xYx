use serde::Serialize;
use serde_with::serde_as;
use sqlx::FromRow;
use time::OffsetDateTime;
use x_common::{model::page::Pageable, utils::date::DateTimeFormat};


/**
* @Author xYx
* @Date 2022-11-16 16:02:58
*/

#[serde_as]
#[derive(FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupDto {
    pub id: String,
    pub owner: String,
    pub name: String,
    pub description: Option<String>,
    pub status: bool,
    pub created_by: String,
    pub updated_by: Option<String>,
    #[serde_as(as = "DateTimeFormat")]
    pub created_at: OffsetDateTime,
    #[serde_as(as = "DateTimeFormat")]
    pub updated_at: OffsetDateTime,
    #[serde(skip)]
    #[sqlx(default)]
    pub total: Option<i64>,
}

impl Pageable for UserGroupDto {
    fn total(&self) -> i64 {
        self.total.unwrap_or_default()
    }
}

