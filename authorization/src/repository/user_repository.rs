use anyhow::{anyhow, Result};
use sqlx::{postgres::PgQueryResult, query, query_as, query_scalar};
use x_common::utils::code;
use x_core::application::POOL;

use crate::dto::{
    request::user_requests::{RegisterUserRequest, UpdateUserRequest},
    response::user_responses::UserDto,
};

pub struct UserRepository;

impl UserRepository {
    pub async fn register_user(&self, record: &RegisterUserRequest) -> Result<String> {
        let id = code::unique_id();
        query_scalar!(
            "INSERT INTO sys_user (id, account, password, origin) VALUES ($1, $2, $3, $4) returning id",
            id,
            record.account,
            record.password,
            record.origin
        )
        .fetch_one(&*POOL)
        .await
        .map_err(|e| anyhow!("Failed to register user: {}", e))
    }

    pub async fn update(&self, record: &UpdateUserRequest) -> Result<PgQueryResult, sqlx::Error> {
        query!(
            "UPDATE sys_user SET password = $1 WHERE id = $2",
            record.password,
            record.id,
        )
        .execute(&*POOL)
        .await
    }

    pub async fn delete_by_id(&self, id: String) -> Result<PgQueryResult, sqlx::Error> {
        query!("DELETE FROM sys_user WHERE id = $1", id,).execute(&*POOL).await
    }

    pub async fn fetch_page(&self, limit: i64, offset: i64) -> Result<Vec<UserDto>, sqlx::Error> {
        query_as!(
            UserDto,
            "select id , origin, account, nick_name, count(*) over() as total from sys_user limit $1 offset $2",
            limit,
            offset
        )
        .fetch_all(&*POOL)
        .await
    }
}
