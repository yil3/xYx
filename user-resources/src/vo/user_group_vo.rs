use serde::Deserialize;
use time::OffsetDateTime;

use crate::po::user_group::UserGroup;

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
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserUserGroupParam {
    pub user_ids: Vec<String>,
    pub user_group_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleUserGroupParam {
    pub role_ids: Vec<String>,
    pub user_group_id: String,
}

impl UserGroupParam {
    pub fn into_entity(&self) -> UserGroup {
        UserGroup {
            id: self.id.to_owned().unwrap_or_default(),
            owner: self.owner.to_owned().unwrap_or("system".to_string()),
            name: self.name.to_owned().unwrap_or_default(),
            description: self.description.to_owned(),
            status: self.status.unwrap_or_default(),
            created_by: Default::default(),
            updated_by: Default::default(),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}
