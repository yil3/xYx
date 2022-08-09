use async_trait::async_trait;
use x_core::{traits::users::service::IUserService, errors::XResult};
use x_domain::users::{UserDto, requests::{RegisterUserDto, UpdateUserDto}};
use crate::repository::user;

pub struct Service {
}

#[async_trait]
impl IUserService for Service {
    async fn register_user(&self, param: RegisterUserDto) -> XResult<UserDto> {
        todo!()
    }

    async fn get_current_user(&self, user_id: i64) -> XResult<UserDto> {
        todo!()
    }

    async fn updated_user(&self, user_id: i64, param: UpdateUserDto) -> XResult<UserDto> {
        todo!()
    }
}
