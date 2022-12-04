use serde::Serialize;
use serde_with::serde_as;
use time::OffsetDateTime;
use x_common::{model::page::Pageable, utils::date::DateTimeFormat};
/**
* @Author xYx
* @Date 2022-12-04 22:40:16
*/

#[serde_as]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RolePermissionGroupPageDto {
    pub id: String,
    pub owner: String,
    pub name: String,
    pub code: String,
    pub description: String,
    pub status: bool,
    pub admin_role_id: String,
    #[serde_as(as = "DateTimeFormat")]
    pub created_at: OffsetDateTime,
    #[serde_as(as = "DateTimeFormat")]
    pub updated_at: OffsetDateTime,
    pub created_by: String,
    pub updated_by: String,
    pub total: Option<i64>,
}

impl Pageable for RolePermissionGroupPageDto {
    fn total(&self) -> i64 {
        self.total.unwrap_or_default()
    }
}
