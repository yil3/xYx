use std::sync::Arc;
use async_trait::async_trait;
use mockall::automock;

use crate::entity::user::UserEntity;

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynUsersRepository = Arc<dyn IUserRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait IUserRepository {
    async fn find_user_by_email_or_username(&self, email: &str, username: &str)
        -> anyhow::Result<Option<UserEntity>>;

    async fn create_user(&self, email: &str, username: &str, hashed_password: &str) -> anyhow::Result<UserEntity>;

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

