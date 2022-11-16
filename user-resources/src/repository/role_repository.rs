use sqlx::query_scalar;
use x_core::application::PG_POOL;

use crate::entity::role::RoleEntity;

/**
* @Author xYx
* @Date 2022-11-16 16:47:06
*/

pub struct RoleRepository;

impl RoleRepository {
    pub async fn insert(&self, record: &RoleEntity) -> anyhow::Result<String> {
        let id = query_scalar!(
            "INSERT INTO sys_role 
            (owner, name, code, description, parent_id, created_by, updated_by) 
            VALUES 
            ($1, $2, $3, $4, $5, $6, $7) 
            RETURNING id",
            record.owner,
            record.name,
            record.code,
            record.description,
            record.parent_id,
            record.created_by,
            record.updated_by
        )
        .fetch_one(&*PG_POOL)
        .await?;
        Ok(id)
    }

    pub async fn update(&self, record: &RoleEntity) -> anyhow::Result<String> {
        let id = query_scalar!(
            "UPDATE sys_role SET 
            name = coalesce($1, name),
            code = coalesce($2, code),
            description = coalesce($3, description),
            parent_id = coalesce($4, parent_id),
            updated_by = $5 
            WHERE id = $6 
            RETURNING id",
            record.name,
            record.code,
            record.description,
            record.parent_id,
            record.updated_by,
            record.id
        )
        .fetch_one(&*PG_POOL)
        .await?;
        Ok(id)
    }
}
