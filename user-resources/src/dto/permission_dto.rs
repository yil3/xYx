use serde::Serialize;
use serde_with::serde_as;
use sqlx::FromRow;
use time::OffsetDateTime;
use x_common::utils::date::DateTimeFormat;

use crate::po::permission::Permission;

/**
* @Author xYx
* @Date 2022-11-23 15:22:09
*/

#[serde_as]
#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PermissionDto {
    pub id: String,
    pub owner: String,
    pub name: String,
    pub code: String,
    pub role_id: String,
    pub description: String,
    pub status: bool,
    #[serde_as(as = "DateTimeFormat")]
    pub created_at: OffsetDateTime,
    #[serde_as(as = "DateTimeFormat")]
    pub updated_at: OffsetDateTime,
    pub created_by: String,
    pub updated_by: String,
    #[serde(skip_serializing)]
    #[sqlx(default)]
    pub total: Option<i64>,
}

impl From<Permission> for PermissionDto {
    fn from(entity: Permission) -> Self {
        Self {
            id: entity.id,
            owner: entity.owner,
            name: entity.name,
            code: entity.code,
            role_id: entity.role_id,
            description: entity.description,
            status: entity.status,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            created_by: entity.created_by,
            updated_by: entity.updated_by,
            total: None,
        }
    }
}
