use async_trait::async_trait;
use x_core::{traits::{
    oauth2::security::DynSecurity,
    users::{repository::DynUsersRepository, service::IUserService},
}, errors::XResult};
use x_domain::users::{UserDto, requests::{RegisterUserDto, UpdateUserDto}};

pub struct UserService {
    pub repository: DynUsersRepository,
    pub security_utils: DynSecurity,
}

impl UserService {
    pub fn new(repository: DynUsersRepository, security_utils: DynSecurity) -> Self {
        Self {
            repository,
            security_utils
        }
    }
}

#[async_trait]
impl IUserService for UserService {
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
