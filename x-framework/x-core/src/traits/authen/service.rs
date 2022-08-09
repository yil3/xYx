use async_trait::async_trait;
use x_domain::users::{UserDto, requests::LoginUserDto};

use crate::errors::XResult;

#[async_trait]
pub trait IAuthenService {
    fn login(&self, param: LoginUserDto) -> XResult<UserDto>;
    fn logout(&self) -> XResult<()>;
}
    
