use serde::Deserialize;
use time::OffsetDateTime;

use crate::po::role::Role;

/**
* @Author xYx
* @Date 2022-11-22 22:48:30
*/

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
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
    pub fn into_entity(&self) -> Role {
        Role {
            id: self.id.to_owned().unwrap_or_default(),
            owner: self.owner.to_owned().unwrap_or("system".into()),
            name: self.name.to_owned().unwrap_or_default(),
            code: self.code.to_owned().unwrap_or_default(),
            description: self.description.to_owned(),
            parent_id: self.parent_id.to_owned().unwrap_or_default(),
            status: self.status.unwrap_or_default(),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
            created_by: self.created_by.to_owned().unwrap_or_default(),
            updated_by: self.updated_by.to_owned().unwrap_or_default(),
        }
    }
}
