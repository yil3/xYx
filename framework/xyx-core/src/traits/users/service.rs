
use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;

use xyx_domain::users::requests::{LoginUserDto, RegisterUserDto, UpdateUserDto};
use xyx_domain::users::UserDto;

use crate::errors::XResult;

/// A reference counter for our user service allows us safely pass instances user utils
/// around which themselves depend on the user repostiory, and ultimately, our Posgres connection pool.
pub type DynUsersService = Arc<dyn IUserService + Send + Sync>;

#[automock]
#[async_trait]
pub trait IUserService {
    async fn register_user(&self, request: RegisterUserDto) -> XResult<UserDto>;

    async fn login_user(&self, request: LoginUserDto) -> XResult<UserDto>;

    async fn get_current_user(&self, user_id: i64) -> XResult<UserDto>;

    async fn updated_user(&self, user_id: i64, request: UpdateUserDto) -> XResult<UserDto>;
}
