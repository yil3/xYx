use async_trait::async_trait;
use x_core::{traits::users::repository::IUserRepository, entity::user::UserEntity};

pub struct Dao;

#[async_trait]
impl IUserRepository for Dao {
    async fn find_user_by_email_or_username(&self, email: &str, username: &str) -> anyhow::Result<Option<UserEntity>> {
        todo!()
    }

    async fn create_user(&self, email: &str, username: &str, hashed_password: &str) -> anyhow::Result<UserEntity> {
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
