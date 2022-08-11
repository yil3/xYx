use std::time::SystemTime;

use sqlx::FromRow;
use time::OffsetDateTime;
use crate::utils::code;
use x_domain::{users::UserDto, profiles::ProfileDto};

#[derive(FromRow)]
pub struct UserEntity {
    pub id: String,
    pub account: String,
    pub email: String,
    pub mobile: String,
    pub password: String,
    pub bio: String,
    pub image: String,
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
            bio: Some(self.bio),
            image: Some(self.image),
            token,
        }
    }

    pub fn into_profile(self, following: bool) -> ProfileDto {
        ProfileDto {
            account: self.account,
            bio: Some(self.bio),
            image: Some(self.image),
            following,
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
            image: String::from("sub"),
            bio: String::from("stub bio"),
            created_at: OffsetDateTime::from(SystemTime::now()),
            updated_at: OffsetDateTime::from(SystemTime::now()),
        }
    }
}

