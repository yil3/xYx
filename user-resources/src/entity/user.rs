use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use x_common::utils::code;

#[derive(FromRow, Serialize, Deserialize)]
pub struct UserEntity {
    pub id: String,
    pub account: String,
    pub origin: Option<String>,
    pub password: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Default for UserEntity {
    fn default() -> Self {
        UserEntity {
            id: code::unique_id(),
            account: Default::default(),
            origin: Default::default(),
            password: String::from("hashed password"),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}
