use anyhow::Result;
use sqlx::{postgres::PgQueryResult, query, query_as, query_scalar};
use x_core::application::PG_POOL;

use crate::{
    dto::user_dto::{RegisterUserParam, UpdateUserParam, UserRecord},
    entity::user::UserEntity,
};

pub struct UserRepository;

impl UserRepository {
    pub async fn fetch_user_by_account(&self, account: &str) -> Result<UserEntity> {
        let user = query_as!(
            UserEntity,
            r#"
            SELECT su.* FROM sys_user su
            left join user_account ua on su.id = ua.owner
            WHERE ua.account = $1
            "#,
            account
        )
        .fetch_one(&*PG_POOL)
        .await?;
        Ok(user)
    }
    pub async fn insert(&self, record: &RegisterUserParam) -> Result<String> {
        let mut trans = PG_POOL.begin().await?;
        let id = query_scalar!(
            "INSERT INTO sys_user (password, origin) VALUES ($1, $2) returning id",
            record.password,
            record.origin,
        )
        .fetch_one(&mut trans)
        .await?;
        query!(
            r#"insert into user_account(account, owner, category) values ($1, $2, '0')"#,
            record.account,
            &id
        )
        .execute(&mut trans)
        .await?;
        query!(
            "INSERT INTO user_info (owner, nickname) VALUES ($1, $2)",
            &id,
            record.nickname
        )
        .execute(&mut trans)
        .await?;
        trans.commit().await?;
        Ok(id)
    }

    pub async fn update(&self, record: &UpdateUserParam) -> Result<PgQueryResult> {
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

    pub async fn fetch_page(&self, limit: i64, offset: i64) -> Result<Vec<UserRecord>> {
        let list = query_as!(
            UserRecord,
            r#"
            select u.id , u.origin, ua.account, ui.nickname, count(*) over() as total 
            from sys_user u
            left join user_account ua on u.id = ua.owner and ua.category = '0'
            left join user_info ui on ui.owner = u.id
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
