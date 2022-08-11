use std::sync::Arc;

use async_trait::async_trait;
// use mockall::automock;

use x_common::entities::user::UserEntity;
use x_common::errors::XResult;
use x_domain::users::requests::UpdateUserDto;
use x_domain::users::UserDto;

/// A reference counter for our user service allows us safely pass instances user utils
/// around which themselves depend on the user repostiory, and ultimately, our Posgres connection pool.
pub type DynUsersService = Arc<dyn IUserService + Send + Sync>;

pub type DynUsersRepository = Arc<dyn IUserRepository + Send + Sync>;

// #[automock]
#[async_trait]
pub trait IUserService {
    async fn get_current_user(&self, user_id: String) -> XResult<UserDto>;

    async fn updated_user(&self, param: UpdateUserDto) -> XResult<UserDto>;
}

// #[automock]
#[async_trait]
pub trait IUserRepository {
    async fn find_user_by_email_or_username(&self, email: &str, username: &str) -> anyhow::Result<Option<UserEntity>>;

    async fn get_user_by_email(&self, email: &str) -> anyhow::Result<Option<UserEntity>>;

    async fn get_user_by_username(&self, username: &str) -> anyhow::Result<Option<UserEntity>>;

    async fn get_user_by_id(&self, id: &str) -> anyhow::Result<UserEntity>;

    async fn update_user(
        &self,
        id: String,
        email: String,
        username: String,
        password: String,
        bio: String,
        image: String,
    ) -> anyhow::Result<UserEntity>;
}
