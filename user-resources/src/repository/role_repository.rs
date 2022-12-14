use sqlx::{query, query_as, query_scalar};
use x_common::model::page::PageParam;
use x_core::application::PG_POOL;

use crate::{
    dto::role_dto::{RolePageDto, RoleTreeDto},
    po::role::Role,
    vo::role_vo::RoleParam,
};

/**
* @Author xYx
* @Date 2022-11-16 16:47:06
*/

pub struct RoleRepository;

impl RoleRepository {
    pub async fn fetch_role_by_user(&self, user_id: &str) -> Result<Vec<Role>, sqlx::Error> {
        query_as!(
            Role,
            r#"
            SELECT
                r.*
            FROM
                sys_role r
            LEFT JOIN user_role ur ON r.id = ur.role_id
            WHERE
                ur.user_id = $1
            "#,
            user_id
        )
        .fetch_all(&*PG_POOL)
        .await
    }

    pub async fn insert(&self, record: &RoleParam, user_id: &str) -> Result<Role, sqlx::Error> {
        query_as!(
            Role,
            "INSERT INTO sys_role 
                (owner, name, code, description, parent_id, role_group_id, created_by, updated_by) 
                VALUES 
                ($1, $2, $3, $4, $5, $6, $7, $7) 
                RETURNING *",
            record.owner,
            record.name,
            record.code,
            record.description,
            record.parent_id,
            record.role_gourop_id,
            user_id,
        )
        .fetch_one(&*PG_POOL)
        .await
    }

    pub async fn is_exists(&self, role_group_id: &str) -> Result<bool, sqlx::Error> {
        query_scalar!(
            "SELECT 
                count(*) 
            FROM 
                sys_role 
            WHERE 
                role_group_id = $1",
            role_group_id
        )
        .fetch_one(&*PG_POOL)
        .await
        .map(|x| x.unwrap_or(0) > 0)
    }

    pub async fn update(&self, record: &RoleParam, user_id: &str) -> Result<Role, sqlx::Error> {
        query_as!(
            Role,
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
            user_id,
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

    pub async fn fetch_page(&self, param: &PageParam) -> Result<Vec<RolePageDto>, sqlx::Error> {
        let mut sql = "SELECT *, count(*) over() total FROM sys_role WHERE 1 = 1".to_string();
        if let Some(query) = &param.query {
            if !query.is_empty() {
                sql.push_str(&format!(" AND name LIKE '%{}%'", query));
            }
        }
        sql.push_str(" ORDER BY created_at DESC");
        sql.push_str(&format!(" LIMIT {} OFFSET {}", param.limit(), param.offset()));
        query_as(&sql).fetch_all(&*PG_POOL).await
    }

    pub async fn fetch_by_parent_id(&self, parent_id: &str) -> Result<Vec<Role>, sqlx::Error> {
        query_as("SELECT * FROM sys_role WHERE parent_id = $1")
            .bind(parent_id)
            .fetch_all(&*PG_POOL)
            .await
    }

    pub async fn fetch_all(&self) -> Result<Vec<RoleTreeDto>, sqlx::Error> {
        query_as("SELECT * FROM sys_role where status = '1'")
            .fetch_all(&*PG_POOL)
            .await
    }
}
