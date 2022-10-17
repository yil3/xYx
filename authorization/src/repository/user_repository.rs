use anyhow::{anyhow, Result};
use sqlx::{postgres::PgQueryResult, query, query_scalar};
use x_common::utils::code;
use x_core::application::POOL;

use crate::dto::request::user_requests::{RegisterUserRequest, UpdateUserRequest};

pub struct UserRepository;

impl UserRepository {
    pub async fn register_user(&self, record: &RegisterUserRequest) -> Result<String> {
        let id = code::unique_id();
        query_scalar!(
            "INSERT INTO sys_user (id, password, email) VALUES ($1, $2, $3) returning id",
            id,
            record.password,
            record.email,
        )
        .fetch_one(&*POOL)
        .await
        .map_err(|e| anyhow!("Failed to register user: {}", e))
    }

    pub async fn update(&self, record: &UpdateUserRequest) -> Result<PgQueryResult, sqlx::Error> {
        query!(
            "UPDATE sys_user SET password = $1, email = $2 WHERE id = $3",
            record.password,
            record.email,
            record.id,
        )
        .execute(&*POOL)
        .await
    }

    pub async fn delete_by_id(&self, id: String) -> Result<PgQueryResult, sqlx::Error> {
        query!("DELETE FROM sys_user WHERE id = $1", id,).execute(&*POOL).await
    }

    pub fn fetch_page() {}
}
