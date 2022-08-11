use async_trait::async_trait;

use x_common::{entities::user::UserEntity, errors::XResult};
use x_domain::users::{requests::RegisterUserDto, UserDto};

#[async_trait]
pub trait IUserRepository {
    async fn create_user(&self, email: &str, account: &str, hashed_password: &str) -> anyhow::Result<UserEntity>;
}

#[async_trait]
pub trait IUserService {
    async fn register_user(&self, param: RegisterUserDto) -> XResult<UserDto>;
}
