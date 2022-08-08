
use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;

use x_domain::users::requests::{RegisterUserDto, UpdateUserDto};
use x_domain::users::UserDto;

use crate::errors::XResult;

/// A reference counter for our user service allows us safely pass instances user utils
/// around which themselves depend on the user repostiory, and ultimately, our Posgres connection pool.
pub type DynUsersService = Arc<dyn IUserService + Send + Sync>;

#[automock]
#[async_trait]
pub trait IUserService {
    async fn register_user(&self, param: RegisterUserDto) -> XResult<UserDto>;

    async fn get_current_user(&self, user_id: i64) -> XResult<UserDto>;

    async fn updated_user(&self, user_id: i64, param: UpdateUserDto) -> XResult<UserDto>;
}
