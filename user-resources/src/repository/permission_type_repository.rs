use x_core::application::PG_POOL;

use crate::entity::permission::PermissionEntity;

/**
* @Author xYx
* @Date 2022-11-17 10:56:53
*/

pub struct PermissionTypeRepository;

impl PermissionTypeRepository {
    pub async fn insert(&self, record: &PermissionEntity) -> Result<String, sqlx::Error> {
        sqlx::query_scalar!(
            r#"
            INSERT INTO permission_type (owner, name, description)
            VALUES ($1, $2, $3)
            returning id
            "#,
            &record.owner,
            &record.name,
            &record.description
        )
        .fetch_one(&*PG_POOL)
        .await
    }
    
    pub async fn update(&self, record: &PermissionEntity) -> Result<String, sqlx::Error> {
        sqlx::query_scalar!(
            r#"
            UPDATE permission_type
            SET name = coalesce($1, name), description = coalesce($2, description)
            WHERE id = $3
            returning id
            "#,
            &record.name,
            &record.description,
            &record.id
        )
        .fetch_one(&*PG_POOL)
        .await
    }
}
