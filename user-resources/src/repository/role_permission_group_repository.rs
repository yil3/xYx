use sqlx::{query, query_as, query_scalar};
use x_common::model::page::PageParam;
use x_core::application::PG_POOL;

use crate::{
    dto::role_permission_group_dto::RolePermissionGroupPageDto, vo::role_permission_vo::RolePermissionGroupParam,
};

/**
* @Author xYx
* @Date 2022-12-03 17:26:33
*/

pub struct RolePermissionGroupRepository;

impl RolePermissionGroupRepository {
    pub async fn fetch_page(&self, param: &PageParam) -> Result<Vec<RolePermissionGroupPageDto>, sqlx::Error> {
        query_as!(
            RolePermissionGroupPageDto,
            r#"
            SELECT
            rpg.*, COUNT(*) OVER() AS total, sr.id AS admin_role_id
            FROM role_permission_group rpg
            left join sys_role sr on sr.group_id = rpg.id and sr.parent_id = '0'
            where 
                case when $1::varchar is not null then 
                    rpg.name like concat('%', $1, '%') or rpg.code like '%||$1||%' or rpg.description like '%||$1||%'
                else 
                    1=1 
            end
            ORDER BY rpg.created_at
            LIMIT $2
            OFFSET $3
            "#,
            param.query,
            param.limit(),
            param.offset(),
        )
        .fetch_all(&*PG_POOL)
        .await
    }

    pub async fn insert(&self, record: &RolePermissionGroupParam, user_id: &str) -> Result<String, sqlx::Error> {
        query_scalar!(
            "INSERT INTO role_permission_group 
                (owner, name, code, description, created_by, updated_by) 
                VALUES 
                ($1, $2, $3, $4, $5, $5) 
                RETURNING id",
            record.owner,
            record.name,
            record.code,
            record.description,
            user_id,
        )
        .fetch_one(&*PG_POOL)
        .await
    }

    pub async fn update(&self, record: &RolePermissionGroupParam, user_id: &str) -> Result<String, sqlx::Error> {
        query_scalar!(
            "UPDATE role_permission_group 
                SET 
                    name = $1, 
                    code = $2, 
                    description = $3, 
                    updated_by = $4, 
                    updated_at = now() 
                WHERE 
                    id = $5 
                RETURNING id",
            record.name,
            record.code,
            record.description,
            user_id,
            record.id,
        )
        .fetch_one(&*PG_POOL)
        .await
    }

    pub async fn delete(&self, id: &str) -> Result<u64, sqlx::Error> {
        query!(
            "DELETE FROM role_permission_group 
                WHERE 
            id = $1",
            id,
        )
        .execute(&*PG_POOL)
        .await
        .map(|r| r.rows_affected())
    }
}
