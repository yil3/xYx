use anyhow::Result;
use sqlx::{postgres::PgQueryResult, query, query_as, query_scalar};
use x_common::utils::code;
use x_core::application::PG_POOL;

use crate::{dto::{
    request::users_requests::{RegisterUserRequest, UpdateUserRequest},
    response::users_responses::UserDto,
}, entity::user::UserEntity};

pub struct UserRepository;

impl UserRepository {
    pub async fn fetch_user_by_account(&self, account: &str) -> Result<UserEntity> {
        let user = query_as!(
            UserEntity,
            r#"
            SELECT * FROM sys_user WHERE account = $1
            "#,
            account
        )
        .fetch_one(&*PG_POOL)
        .await?;
        Ok(user)
    }
    pub async fn insert(&self, record: &RegisterUserRequest) -> Result<String> {
        let id = code::unique_id();
        let mut trans = PG_POOL.begin().await?;
        query_scalar!(
            "INSERT INTO sys_user (id, account, password, origin) VALUES ($1, $2, $3, $4) returning id",
            id,
            record.account,
            record.password,
            record.origin
        )
        .fetch_one(&mut trans)
        .await?;

        query!(
            "INSERT INTO sys_user_info (id, owner, nickname) VALUES ($1, $2, $3)",
            code::unique_id(),
            id,
            record.nickname
        )
        .execute(&mut trans)
        .await?;
        trans.commit().await?;
        Ok(id)
    }

    pub async fn update(&self, record: &UpdateUserRequest) -> Result<PgQueryResult> {
        let query = query!(
            "UPDATE sys_user SET password = $1 WHERE id = $2",
            record.password,
            record.id,
        )
        .execute(&*PG_POOL)
        .await?;
        Ok(query)
    }

    pub async fn delete_by_id(&self, id: String) -> Result<PgQueryResult> {
        Ok(query!("DELETE FROM sys_user WHERE id = $1", id,)
            .execute(&*PG_POOL)
            .await?)
    }

    pub async fn fetch_page(&self, limit: i64, offset: i64) -> Result<Vec<UserDto>> {
        let list = query_as!(
            UserDto,
            r#"
            select u.id , u.origin, u.account, ui.nickname, count(*) over() as total 
            from sys_user u
            left join sys_user_info ui on ui.owner = u.id
            left join sys_token st on st.owner = u.id
            limit $1 offset $2
            "#,
            limit,
            limit * (offset - 1)
        )
        .fetch_all(&*PG_POOL)
        .await?;
        Ok(list)
    }
}
