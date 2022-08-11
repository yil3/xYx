use async_trait::async_trait;

use crate::{entity::user::UserEntity, errors::XResult};

#[async_trait]
pub trait IAuthenRepository {
    async fn create_user(&self, email: &str, account: &str, hashed_password: &str) -> anyhow::Result<UserEntity>;

    async fn find_user_by_mobile_or_email_or_account(&self, arg: &str) -> XResult<Option<UserEntity>>;
}
