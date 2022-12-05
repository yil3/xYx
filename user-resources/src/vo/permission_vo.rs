use axum::Json;
use serde::Deserialize;

use crate::po::permission::Permission;

/**
* @Author xYx
* @Date 2022-11-23 10:54:30
*/

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionParam {
    pub id: Option<String>,
    pub owner: Option<String>,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub role_group_id: String,
    pub status: Option<bool>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionTypeParam {
    pub id: Option<String>,
    pub owner: Option<String>,
    pub name: String,
    pub description: Option<String>,
}

impl PermissionParam {
    pub fn into_po(&self) -> Permission {
        Permission {
            id: self.id.to_owned().unwrap_or_default(),
            owner: self.owner.to_owned().unwrap_or_default(),
            name: self.name.to_owned(),
            code: self.code.to_owned(),
            description: self.description.to_owned().unwrap_or_default(),
            role_group_id: self.role_group_id.to_owned(),
            status: self.status.to_owned().unwrap_or_default(),
            created_by: Default::default(),
            updated_by: Default::default(),
            created_at: time::OffsetDateTime::now_utc(),
            updated_at: time::OffsetDateTime::now_utc(),
        }
    }
}

impl Into<Permission> for Json<PermissionParam> {
    fn into(self) -> Permission {
        Permission {
            id: self.id.to_owned().unwrap_or_default(),
            owner: self.owner.to_owned().unwrap_or_default(),
            name: self.name.to_owned(),
            code: self.code.to_owned(),
            description: self.description.to_owned().unwrap_or_default(),
            status: self.status.to_owned().unwrap_or_default(),
            role_group_id: self.role_group_id.to_owned(),
            created_by: Default::default(),
            updated_by: Default::default(),
            created_at: time::OffsetDateTime::now_utc(),
            updated_at: time::OffsetDateTime::now_utc(),
        }
    }
}
