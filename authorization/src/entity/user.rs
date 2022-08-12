use std::time::SystemTime;

use sqlx::FromRow;
use time::OffsetDateTime;
use x_common::utils::code;

use crate::dto::response::user_responses::UserDto;

#[derive(FromRow)]
pub struct UserEntity {
    pub id: String,
    pub account: String,
    pub email: String,
    pub mobile: String,
    pub password: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl UserEntity {
    pub fn into_dto(self, token: String) -> UserDto {
        UserDto {
            id: self.id,
            email: self.email,
            mobile: self.mobile,
            account: self.account,
            token,
        }
    }
}

impl Default for UserEntity {
    fn default() -> Self {
        UserEntity {
            id: code::unique_id(),
            account: String::default(),
            email: String::default(),
            mobile: String::default(),
            password: String::from("hashed password"),
            created_at: OffsetDateTime::from(SystemTime::now()),
            updated_at: OffsetDateTime::from(SystemTime::now()),
        }
    }
}
