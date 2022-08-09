use std::time::SystemTime;

use sqlx::FromRow;
use time::OffsetDateTime;
use x_common::utils::code;
use x_domain::{users::UserDto, profiles::ProfileDto};

#[derive(FromRow)]
pub struct UserEntity {
    pub id: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub username: String,
    pub email: String,
    pub mobile: String,
    pub password: String,
    pub bio: String,
    pub image: String,
}

impl UserEntity {
    pub fn into_dto(self, token: String) -> UserDto {
        UserDto {
            id: self.id,
            email: self.email,
            mobile: self.mobile,
            username: self.username,
            bio: Some(self.bio),
            image: Some(self.image),
            token,
        }
    }

    pub fn into_profile(self, following: bool) -> ProfileDto {
        ProfileDto {
            username: self.username,
            bio: self.bio,
            image: self.image,
            following,
        }
    }
}

impl Default for UserEntity {
    fn default() -> Self {
        UserEntity {
            id: code::unique_id(),
            bio: String::from("stub bio"),
            created_at: OffsetDateTime::from(SystemTime::now()),
            updated_at: OffsetDateTime::from(SystemTime::now()),
            username: String::from("stub username"),
            email: String::from("stub email"),
            mobile: String::default(),
            password: String::from("hashed password"),
            image: String::from("stub image"),
        }
    }
}
