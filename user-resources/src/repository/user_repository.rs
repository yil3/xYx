use async_trait::async_trait;
use sqlx::query_as;
use x_core::{
    connection_pool::PgConnPool,
    traits::users::repository::{IUserRepository, UserEntity},
};

#[derive(Clone)]
pub struct PostgresUsersRepository {
    pool: PgConnPool,
}

impl PostgresUsersRepository {
    pub fn new(pool: PgConnPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IUserRepository for PostgresUsersRepository {
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

    async fn get_user_by_id(&self, id: i64) -> anyhow::Result<UserEntity> {
        todo!()
    }

    async fn update_user(
        &self,
        id: i64,
        email: String,
        username: String,
        password: String,
        bio: String,
        image: String,
    ) -> anyhow::Result<UserEntity> {
        todo!()
    }
}
