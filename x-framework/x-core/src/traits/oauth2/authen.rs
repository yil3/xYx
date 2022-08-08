use x_domain::users::{UserDto, requests::LoginUserDto};

use crate::errors::XResult;



pub trait Authen {
    fn login(&self, param: LoginUserDto) -> XResult<UserDto>;
    fn logout(&self) -> XResult<()>;
}
    
