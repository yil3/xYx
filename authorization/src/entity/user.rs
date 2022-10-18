use sqlx::FromRow;
use x_common::utils::code;

use crate::dto::response::user_responses::UserDto;

#[derive(FromRow)]
pub struct UserEntity {
    pub id: String,
    pub account: String,
    pub origin: Option<String>,
    pub password: String,
}

impl UserEntity {
    pub fn into_dto(self, token: String) -> UserDto {
        UserDto {
            id: self.id,
            account: self.account,
        }
    }
}

impl Default for UserEntity {
    fn default() -> Self {
        UserEntity {
            id: code::unique_id(),
            account: Default::default(),
            origin: Default::default(),
            password: String::from("hashed password"),
        }
    }
}
