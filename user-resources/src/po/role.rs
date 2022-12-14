use sqlx::FromRow;
use time::OffsetDateTime;

/**
* @Author xYx
* @Date 2022-11-16 11:31:09
*/
#[derive(Debug, Clone, FromRow)]
pub struct Role {
    pub id: String,
    pub owner: String,
    pub name: String,
    pub code: String,
    pub parent_id: String,
    pub description: Option<String>,
    pub role_group_id: String,
    pub status: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub created_by: String,
    pub updated_by: String,
}

pub struct UserRole {
    pub id: String,
    pub user_id: String,
    pub role_id: String,
}

pub struct UserGroupRole {
    pub id: String,
    pub user_group_id: String,
    pub role_id: String,
}

