use sqlx::FromRow;


#[derive(FromRow)]
pub struct TokenEntity {
    pub id: String,
    pub client_id: String,
    pub owner: String,
    pub scope: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub created_at: String,
    pub updated_at: String,
}

pub struct ScopeEntity {
    pub id: String,
    pub name: String,
    pub description: String,
}

pub struct Claims {
    pub sub: String,
    pub scope: String,
    pub exp: usize,
}
