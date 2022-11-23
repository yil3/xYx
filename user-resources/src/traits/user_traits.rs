use std::sync::Arc;

use async_trait::async_trait;
// use mockall::automock;

use x_common::errors::XResult;

use crate::po::user::UserInfo;


pub type DynUsersService = Arc<dyn IUserService + Send + Sync>;
pub type DynUsersRepository = Arc<dyn IUserRepository + Send + Sync>;

// #[automock]
#[async_trait]
pub trait IUserService {
    async fn get_current_user(&self, user_id: String) -> XResult<UserInfo>;

    async fn save_userinfo(&self, param: UserInfo) -> XResult<UserInfo>;
}

// #[automock]
#[async_trait]
pub trait IUserRepository {
    async fn get_user_by_id(&self, id: &str) -> anyhow::Result<UserInfo>;

    async fn update_userinfo(&self, param: UserInfo) -> anyhow::Result<UserInfo>;
}
