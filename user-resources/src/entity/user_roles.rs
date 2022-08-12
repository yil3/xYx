use sqlx::FromRow;

#[derive(FromRow)]
pub struct UserRolesEntity {
    pub id: String,
    pub user_id: String,
    pub role_id: String,
}
