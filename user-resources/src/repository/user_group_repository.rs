use anyhow::Result;
use sqlx::query_as;
use x_common::model::page::PageParam;
use x_core::application::PG_POOL;

use crate::{
    dto::{user_dto::UserProfileDto, user_gourp_dto::UserGroupDto},
    vo::user_group_vo::UserGroupParam,
};

/**
* @Author xYx
* @Date 2022-11-16 14:57:16
*/
pub struct UserGroupRepository;

impl UserGroupRepository {
    pub async fn fetch_users_by_group_id(&self, group_id: &str, param: &PageParam) -> Result<Vec<UserProfileDto>> {
        query_as!(
            UserProfileDto,
            r#"
                select u.id, ui.nickname, count(*) over() as total
                from user_user_group uug 
                left join sys_user u on u.id = uug.user_id
                left join user_info ui on ui.owner = u.id
                where uug.user_group_id = $1
                limit $2 offset $3
            "#,
            group_id,
            param.limit(),
            param.offset()
        )
        .fetch_all(&*PG_POOL)
        .await
        .map_err(Into::into)
    }

    pub async fn insert(&self, param: &UserGroupParam, user_id: &str) -> Result<UserGroupDto, sqlx::Error> {
        query_as("INSERT INTO user_group (owner, name, description, created_by) VALUES ($1, $2, $3, $4) returning *")
            .bind(&param.owner)
            .bind(&param.name)
            .bind(&param.description)
            .bind(user_id)
            .fetch_one(&*PG_POOL)
            .await
    }

    pub async fn update(&self, param: &UserGroupParam, user_id: &str) -> Result<UserGroupDto, sqlx::Error> {
        query_as(
            r#"update user_group set 
            name = coalesce($1, name), description = coalesce($2, description), 
            status = coalesce($3, status), updated_by = coalesce($4, updated_by)
            where id = $5 
            returning *"#,
        )
        .bind(&param.name)
        .bind(&param.description)
        .bind(&param.status)
        .bind(user_id)
        .bind(&param.id)
        .fetch_one(&*PG_POOL)
        .await
    }

    pub async fn fetch_page(&self, param: &PageParam) -> Result<Vec<UserGroupDto>> {
        let mut sql = String::from("select *, count(*) over() as total from user_group where 1 = 1");
        sql.push_str(" order by created_at desc");
        sql.push_str(&format!(" limit {} offset {}", param.limit(), param.offset()));
        let page = query_as(&sql).fetch_all(&*PG_POOL).await?;
        Ok(page)
    }

    pub async fn delete(&self, id: &str) -> Result<u64> {
        let count = sqlx::query("delete from user_group where id = $1")
            .bind(id)
            .execute(&*PG_POOL)
            .await?
            .rows_affected();
        Ok(count)
    }
}
