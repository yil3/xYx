use async_trait::async_trait;
use x_core::{errors::XResult, traits::authen::service::IAuthenService};
use x_domain::users::{requests::LoginUserDto, UserDto};

pub struct Service;

#[async_trait]
impl IAuthenService for Service {
    fn login(&self, param: LoginUserDto) -> XResult<UserDto> {
        todo!()
    }
    fn logout(&self) -> XResult<()> {
        todo!()
    }
}
