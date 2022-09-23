use sqlx::FromRow;


#[derive(FromRow)]
pub struct TokenEntity {
    pub id: String,
    pub client_id: String,
    pub user_id: String,
    pub scope: String,
    pub access_token: String,
    pub refresh_token: String,
    pub access_token_expires_at: String,
    pub refresh_token_expires_at: String,
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
