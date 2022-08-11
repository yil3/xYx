use sqlx::FromRow;

#[derive(FromRow)]
pub struct PermissionEntity {
    pub id: String,
    pub name: String,
}
