use anyhow::Result;
use sqlx::query_as;
use x_common::model::page::PageParam;
use x_core::application::PG_POOL;

use crate::{dto::user_gourp_dto::UserGroupDto, entity::user_group::UserGroupEntity};

/**
* @Author xYx
* @Date 2022-11-16 14:57:16
*/
pub struct UserGroupRepository;

impl UserGroupRepository {
    pub async fn insert(&self, record: &UserGroupEntity) -> Result<UserGroupDto> {
        let record = query_as(
            "INSERT INTO user_group (owner, name, description, created_by) VALUES ($1, $2, $3, $4) returning *",
        )
        .bind(&record.owner)
        .bind(&record.name)
        .bind(&record.description)
        .bind(&record.created_by)
        .fetch_one(&*PG_POOL)
        .await?;
        Ok(record)
    }

    pub async fn update(&self, record: &UserGroupEntity) -> Result<UserGroupDto> {
        let record = query_as(
            r#"update user_group set 
            name = coalesce($1, name), description = coalesce($2, description), 
            status = coalesce($3, status), updated_by = coalesce($4, updated_by)
            where id = $5 
            returning *"#,
        )
        .bind(&record.name)
        .bind(&record.description)
        .bind(&record.status)
        .bind(&record.updated_by)
        .bind(&record.id)
        .fetch_one(&*PG_POOL)
        .await?;
        Ok(record)
    }

    pub async fn fetch_page(&self, param: &PageParam) -> Result<Vec<UserGroupDto>> {
        let mut sql = String::from("select *, count(*) over() as total from user_group where 1 = 1");
        sql.push_str(&format!(" limit {} offset {}", param.limit(), param.offset()));
        sql.push_str(" order by created_at desc");
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