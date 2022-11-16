use std::sync::Arc;

use async_trait::async_trait;
// use mockall::automock;

use x_common::errors::XResult;

use crate::entity::user::UserInfoEntity;


pub type DynUsersService = Arc<dyn IUserService + Send + Sync>;
pub type DynUsersRepository = Arc<dyn IUserRepository + Send + Sync>;

// #[automock]
#[async_trait]
pub trait IUserService {
    async fn get_current_user(&self, user_id: String) -> XResult<UserInfoEntity>;

    async fn save_userinfo(&self, param: UserInfoEntity) -> XResult<UserInfoEntity>;
}

// #[automock]
#[async_trait]
pub trait IUserRepository {
    async fn get_user_by_id(&self, id: &str) -> anyhow::Result<UserInfoEntity>;

    async fn update_userinfo(&self, param: UserInfoEntity) -> anyhow::Result<UserInfoEntity>;
}
