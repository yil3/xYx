use serde::Deserialize;

use crate::entity::permission::PermissionEntity;

/**
* @Author xYx
* @Date 2022-11-23 10:54:30
*/

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionParam {
    pub id: Option<String>,
    pub owner: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub role_id: Option<String>,
    pub status: Option<bool>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

impl PermissionParam {
    pub fn into_entity(&self) -> PermissionEntity {
        PermissionEntity {
            id: self.id.to_owned().unwrap_or_default(),
            owner: self.owner.to_owned().unwrap_or_default(),
            name: self.name.to_owned().unwrap_or_default(),
            code: self.code.to_owned().unwrap_or_default(),
            description: self.description.to_owned().unwrap_or_default(),
            role_id: self.role_id.to_owned().unwrap_or_default(),
            status: self.status.to_owned().unwrap_or_default(),
            created_by: self.created_by.to_owned().unwrap_or_default(),
            updated_by: self.updated_by.to_owned().unwrap_or_default(),
            created_at: time::OffsetDateTime::now_utc(),
            updated_at: time::OffsetDateTime::now_utc(),
        }
    }
}
