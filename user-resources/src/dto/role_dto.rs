use serde::{Serialize, Deserialize};
use serde_with::serde_as;
use time::OffsetDateTime;
use x_common::utils::date::DateTimeFormat;
use sqlx::FromRow;

use crate::entity::role::RoleEntity;

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

#[derive(Deserialize)]
pub struct RoleParam {
    pub id: Option<String>,
    pub owner: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<String>,
    pub status: Option<bool>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

impl RoleParam {
    pub fn into_entity(self) -> RoleEntity {
        RoleEntity {
            id: self.id.unwrap_or_default(),
            owner: self.owner.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
            code: self.code.unwrap_or_default(),
            description: self.description.unwrap_or_default(),
            parent_id: self.parent_id.unwrap_or_default(),
            status: self.status.unwrap_or_default(),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
            created_by: self.created_by.unwrap_or_default(),
            updated_by: self.updated_by.unwrap_or_default(),
        }
    }
}
