use serde::Deserialize;
use time::OffsetDateTime;

use crate::entity::user_group::{UserGroupEntity, UserUserGroupEntity};

/**
* @Author xYx
* @Date 2022-11-22 22:47:30
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserUserGroupBody {
    pub id: Option<String>,
    pub user_id: String,
    pub user_group_id: String,
}

impl UserUserGroupBody {
    pub fn into_entity(&self) -> UserUserGroupEntity {
        UserUserGroupEntity {
            id: self.id.to_owned().unwrap_or_default(),
            user_id: self.user_id.to_owned(),
            user_group_id: self.user_group_id.to_owned(),
        }
    }
}

impl UserGroupParam {
    pub fn into_entity(&self) -> UserGroupEntity {
        UserGroupEntity {
            id: self.id.to_owned().unwrap_or_default(),
            owner: self.owner.to_owned().unwrap_or("system".to_string()),
            name: self.name.to_owned().unwrap_or_default(),
            description: self.description.to_owned(),
            status: self.status.unwrap_or_default(),
            created_by: self.created_by.to_owned().unwrap_or_default(),
            updated_by: self.updated_by.to_owned(),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}
