use async_trait::async_trait;
use x_common::errors::XResult;

use crate::{dto::{request::user_requests::RegisterUserRequest, response::user_responses::UserDto}, entity::user::UserEntity};


#[async_trait]
pub trait IUserRepository {
    async fn create_user(&self, email: &str, account: &str, hashed_password: &str) -> anyhow::Result<UserEntity>;
}

#[async_trait]
pub trait IUserService {
    async fn register_user(&self, param: RegisterUserRequest) -> XResult<UserDto>;
}
