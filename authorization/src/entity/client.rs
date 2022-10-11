use sqlx::FromRow;


#[derive(FromRow, Default)]
pub struct ClientEntity {
    pub id: String,
    pub secret: String,
    pub app_name: String,
    pub redirect_uri: String,
    pub scope: String,
    pub owner: String,
    pub created_at: String,
    pub updated_at: String,
}
