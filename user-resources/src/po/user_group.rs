use time::OffsetDateTime;

/**
* @Author xYx
* @Date 2022-11-16 11:25:24
*/
#[derive(Clone)]
pub struct UserGroup {
    pub id: String,
    pub owner: String,
    pub name: String,
    pub description: Option<String>,
    pub status: bool,
    pub created_by: String,
    pub updated_by: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

pub struct UserUserGroup {
    pub id: String,
    pub user_id: String,
    pub user_group_id: String,
}
