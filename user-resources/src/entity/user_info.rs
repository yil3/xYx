use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, FromRow)]
pub struct UserInfoEntity {
    pub userid: String,
    pub nickname: String,
    pub id_card: String,
    pub birthday: String,
    pub bio: String,
    pub image: String,

    #[sqlx(default)]
    pub id: Option<String>,
    #[sqlx(default)]
    pub account: Option<String>,
    #[sqlx(default)]
    pub email: Option<String>,
    #[sqlx(default)]
    pub mobile: Option<String>,
    #[sqlx(default)]
    pub password: Option<String>,
    #[sqlx(default)]
    pub created_at: Option<OffsetDateTime>,
    #[sqlx(default)]
    pub updated_at: Option<OffsetDateTime>,
}
