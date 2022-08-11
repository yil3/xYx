use async_trait::async_trait;

use x_common::{errors::XResult, entities::user::UserEntity};
use x_domain::users::{requests::LoginUserDto, UserDto};

#[async_trait]
pub trait IAuthenRepository {
    async fn find_user_by_mobile_or_email_or_account(&self, arg: &str) -> XResult<Option<UserEntity>>;
}

#[async_trait]
pub trait IAuthenService {
    async fn login(&self, param: LoginUserDto) -> XResult<UserDto>;

    async fn logout(&self) -> XResult<()>;
}
