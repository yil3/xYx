use serde::Deserialize;
use serde::Serialize;
use serde_with::serde_as;
use sqlx::FromRow;
use time::OffsetDateTime;
use x_common::{model::page::Pageable, utils::date::DateTimeFormat};

use crate::entity::user_group::UserGroupEntity;

/**
* @Author xYx
* @Date 2022-11-16 16:02:58
*/

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupParam {
    pub id: Option<String>,
    pub owner: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<bool>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

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

impl UserGroupParam {
    pub fn into_entity(self) -> UserGroupEntity {
        UserGroupEntity {
            id: self.id.unwrap_or_default(),
            owner: self.owner.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
            description: self.description,
            status: self.status.unwrap_or_default(),
            created_by: self.created_by.unwrap_or_default(),
            updated_by: self.updated_by,
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}
