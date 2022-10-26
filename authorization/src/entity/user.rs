use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use x_common::utils::code;

/**
* @Author xYx
* @Date 2022-09-26 11:07:54
*/
#[derive(FromRow, Serialize, Deserialize)]
pub struct UserEntity {
    pub id: String,
    pub account: String,
    pub origin: Option<String>,
    pub password: String,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

impl Default for UserEntity {
    fn default() -> Self {
        UserEntity {
            id: code::unique_id(),
            account: Default::default(),
            origin: Default::default(),
            password: String::from("hashed password"),
            created_at: OffsetDateTime::now_utc(),
            updated_at: Some(OffsetDateTime::now_utc()),
        }
    }
}
