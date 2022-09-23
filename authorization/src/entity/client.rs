use sqlx::FromRow;


#[derive(FromRow, Default)]
pub struct ClientEntity {
    pub id: String,
    pub secret: String,
    pub name: String,
    pub redirect_uri: String,
    pub scope: String,
    pub user_id: String,
}
