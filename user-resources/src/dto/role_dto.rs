use serde::Serialize;
use serde_with::serde_as;
use sqlx::FromRow;
use time::OffsetDateTime;
use x_common::utils::date::DateTimeFormat;

/**
* @Author xYx
* @Date 2022-11-16 16:55:48
*/

#[serde_as]
#[derive(Serialize, FromRow)]
pub struct RoleDto {
    pub id: String,
    pub owner: String,
    pub name: String,
    pub code: String,
    pub description: String,
    pub parent_id: String,
    pub status: bool,
    #[serde_as(as = "DateTimeFormat")]
    pub created_at: OffsetDateTime,
    #[serde_as(as = "DateTimeFormat")]
    pub updated_at: OffsetDateTime,
    pub created_by: String,
    pub updated_by: String,
    #[sqlx(default)]
    #[serde(skip_serializing)]
    pub total: i64,
}
