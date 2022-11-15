use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use x_common::utils::code;

#[derive(Serialize, Deserialize)]
pub struct UserEntity {
    pub id: String,
    pub salt: String,
    pub origin: Option<String>,
    pub password: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Default for UserEntity {
    fn default() -> Self {
        UserEntity {
            id: code::unique_id(),
            origin: Default::default(),
            salt: Default::default(),
            password: String::from("hashed password"),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}
