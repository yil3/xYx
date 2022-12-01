use serde::Serialize;
use serde_with::serde_as;
use sqlx::{postgres::PgRow, FromRow, Row};
use time::OffsetDateTime;
use x_common::{
    model::page::Pageable,
    utils::{date::DateTimeFormat, vector::Treeable},
};

use crate::po::role::Role;

/**
* @Author xYx
* @Date 2022-11-16 16:55:48
*/

#[serde_as]
#[derive(Serialize, FromRow, Clone)]
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
}
impl From<Role> for RoleDto {
    fn from(role: Role) -> Self {
        Self {
            id: role.id,
            owner: role.owner,
            name: role.name,
            code: role.code,
            description: role.description.unwrap_or_default(),
            parent_id: role.parent_id,
            status: role.status,
            created_at: role.created_at,
            updated_at: role.updated_at,
            created_by: role.created_by,
            updated_by: role.updated_by,
        }
    }
}

#[serde_as]
#[derive(Serialize, Clone)]
pub struct RoleTreeDto {
    pub id: String,
    pub owner: String,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub parent_id: String,
    pub status: bool,
    #[serde_as(as = "DateTimeFormat")]
    pub created_at: OffsetDateTime,
    #[serde_as(as = "DateTimeFormat")]
    pub updated_at: OffsetDateTime,
    pub created_by: String,
    pub updated_by: String,
    pub children: Vec<RoleTreeDto>,
}

impl FromRow<'_, PgRow> for RoleTreeDto {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            owner: row.try_get("owner")?,
            name: row.try_get("name")?,
            code: row.try_get("code")?,
            description: row.try_get("description")?,
            parent_id: row.try_get("parent_id")?,
            status: row.try_get("status")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            created_by: row.try_get("created_by")?,
            updated_by: row.try_get("updated_by")?,
            children: vec![],
        })
    }
}

impl Treeable<RoleTreeDto> for RoleTreeDto {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_parent_id(&self) -> &str {
        &self.parent_id
    }

    fn set_children(&mut self, children: Vec<RoleTreeDto>) {
        self.children = children;
    }
}

#[serde_as]
#[derive(Serialize, FromRow, Clone)]
pub struct RolePageDto {
    pub id: String,
    pub owner: String,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
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
    pub total: Option<i64>,
}

impl Pageable for RolePageDto {
    fn total(&self) -> i64 {
        self.total.unwrap_or_default()
    }
}
