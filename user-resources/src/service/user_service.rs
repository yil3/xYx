use async_trait::async_trait;
use x_common::errors::XResult;
use x_domain::users::{requests::UpdateUserDto, UserDto};

use crate::traits::user_traits::IUserService;

pub struct UserService {}

#[async_trait]
impl IUserService for UserService {
    async fn get_current_user(&self, user_id: String) -> XResult<UserDto> {
        todo!()
    }

    async fn updated_user(&self, param: UpdateUserDto) -> XResult<UserDto> {
        todo!()
    }
}
