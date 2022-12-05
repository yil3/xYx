use serde::Serialize;
use time::OffsetDateTime;
/**
* @Author xYx
* @Date 2022-11-16 11:32:54
*/
pub struct Permission {
    pub id: String,
    pub owner: String,
    pub name: String,
    pub code: String,
    pub description: String,
    pub status: bool,
    pub role_group_id: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionType {
    pub id: String,
    pub owner: String,
    pub name: String,
    pub description: String,
}

pub struct RolePermission {
    pub id: String,
    pub role_id: String,
    pub permission_id: String,
    pub permission_type_id: String,
}
