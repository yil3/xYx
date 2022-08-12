use sqlx::FromRow;

#[derive(FromRow)]
pub struct RoleEntity {
    pub id: String,
    pub name: String,
}

