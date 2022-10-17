use sqlx::FromRow;
use x_common::utils::code;

use crate::dto::response::user_responses::UserDto;

#[derive(FromRow)]
pub struct UserEntity {
    pub id: String,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub password: String,
}

impl UserEntity {
    pub fn into_dto(self, token: String) -> UserDto {
        UserDto {
            id: self.id,
            email: self.email.unwrap_or_default(),
            mobile: self.mobile.unwrap_or_default(),
            token,
        }
    }
}

impl Default for UserEntity {
    fn default() -> Self {
        UserEntity {
            id: code::unique_id(),
            email: Default::default(),
            mobile: Default::default(),
            password: String::from("hashed password"),
        }
    }
}
