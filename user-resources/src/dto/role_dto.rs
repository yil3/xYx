use serde::Serialize;
use serde_with::serde_as;
use sqlx::FromRow;
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
    pub children: Vec<RoleDto>,
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
            children: vec![],
        }
    }
}

impl Treeable<RoleDto> for RoleDto {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_parent_id(&self) -> &str {
        &self.parent_id
    }

    fn set_children(&mut self, children: Vec<RoleDto>) {
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
    pub total: Option<i64>,
}

impl Pageable for RolePageDto {
    fn total(&self) -> i64 {
        self.total.unwrap_or_default()
    }
}
