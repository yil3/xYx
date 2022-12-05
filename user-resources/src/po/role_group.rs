use time::OffsetDateTime;

/**
* @Author xYx
* @Date 2022-12-03 16:55:50
*/

pub struct RolePermissionGroup {
    pub id: String,
    pub owner: String,
    pub name: String,
    pub code: String,
    pub description: String,
    pub status: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub created_by: String,
    pub updated_by: String,
}
