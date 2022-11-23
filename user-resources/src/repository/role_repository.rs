use sqlx::{query, query_as};
use x_common::model::page::PageParam;
use x_core::application::PG_POOL;

use crate::{dto::role_dto::RoleDto, entity::role::RoleEntity, vo::role_vo::RoleParam};

/**
* @Author xYx
* @Date 2022-11-16 16:47:06
*/

pub struct RoleRepository;

impl RoleRepository {
    pub async fn insert(&self, record: &RoleParam) -> Result<RoleEntity, sqlx::Error> {
        query_as!(
            RoleEntity,
            "INSERT INTO sys_role 
                (owner, name, code, description, parent_id, created_by, updated_by) 
                VALUES 
                ($1, $2, $3, $4, $5, $6, $7) 
                RETURNING *",
            record.owner,
            record.name,
            record.code,
            record.description,
            record.parent_id,
            record.created_by,
            record.updated_by
        )
        .fetch_one(&*PG_POOL)
        .await
    }

    pub async fn update(&self, record: &RoleParam) -> Result<RoleEntity, sqlx::Error> {
        query_as!(
            RoleEntity,
            "UPDATE sys_role SET 
                name = coalesce($1, name),
                code = coalesce($2, code),
                description = coalesce($3, description),
                parent_id = coalesce($4, parent_id),
                updated_by = $5 
                WHERE id = $6 
                RETURNING *",
            record.name,
            record.code,
            record.description,
            record.parent_id,
            record.updated_by,
            record.id
        )
        .fetch_one(&*PG_POOL)
        .await
    }

    pub async fn delete(&self, id: &str) -> Result<u64, sqlx::Error> {
        query!("DELETE FROM sys_role WHERE id = $1", id)
            .execute(&*PG_POOL)
            .await
            .map(|r| r.rows_affected())
    }

    pub async fn fetch_page(&self, param: PageParam) -> Result<Vec<RoleDto>, sqlx::Error> {
        let mut sql = "SELECT * FROM sys_role WHERE 1 = 1".to_string();
        if let Some(query) = &param.query {
            if !query.is_empty() {
                sql.push_str(&format!(" AND name LIKE '%{}%'", query));
            }
        }
        sql.push_str(" ORDER BY created_at DESC");
        sql.push_str(&format!(" LIMIT {} OFFSET {}", param.limit(), param.offset()));
        query_as(&sql).fetch_all(&*PG_POOL).await
    }

    pub async fn fetch_by_parent_id(&self, parent_id: &str) -> Result<Vec<RoleDto>, sqlx::Error> {
        query_as("SELECT * FROM sys_role WHERE parent_id = $1")
            .bind(parent_id)
            .fetch_all(&*PG_POOL)
            .await
    }

    pub async fn fetch_all(&self) -> Result<Vec<RoleDto>, sqlx::Error> {
        query_as("SELECT * FROM sys_role").fetch_all(&*PG_POOL).await
    }
}
