use async_trait::async_trait;
use x_common::entities::user::UserEntity;

use crate::traits::user_traits::IUserRepository;

pub struct UserRepository;

#[async_trait]
impl IUserRepository for UserRepository {
    async fn find_user_by_email_or_username(&self, email: &str, username: &str) -> anyhow::Result<Option<UserEntity>> {
        todo!()
    }

    async fn get_user_by_email(&self, email: &str) -> anyhow::Result<Option<UserEntity>> {
        todo!()
    }

    async fn get_user_by_username(&self, username: &str) -> anyhow::Result<Option<UserEntity>> {
        todo!()
    }

    async fn get_user_by_id(&self, id: &str) -> anyhow::Result<UserEntity> {
        todo!()
    }

    async fn update_user(
        &self,
        id: String,
        email: String,
        username: String,
        password: String,
        bio: String,
        image: String,
    ) -> anyhow::Result<UserEntity> {
        todo!()
    }
}
