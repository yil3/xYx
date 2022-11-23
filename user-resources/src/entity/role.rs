use time::OffsetDateTime;

/**
* @Author xYx
* @Date 2022-11-16 11:31:09
*/
pub struct RoleEntity {
    pub id: String,
    pub owner: String,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub parent_id: String,
    pub status: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub created_by: String,
    pub updated_by: String,
}

pub struct UserRoleEntity {
    pub id: String,
    pub user_id: String,
    pub role_id: String,
}

pub struct UserGroupRoleEntity {
    pub id: String,
    pub user_group_id: String,
    pub role_id: String,
}

