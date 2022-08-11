use async_trait::async_trait;
use x_domain::users::{
    requests::{LoginUserDto, RegisterUserDto},
    UserDto,
};

use crate::errors::XResult;

#[async_trait]
pub trait IAuthenService {
    async fn register_user(&self, param: RegisterUserDto) -> XResult<UserDto>;

    async fn login(&self, param: LoginUserDto) -> XResult<UserDto>;

    async fn logout(&self) -> XResult<()>;
}
