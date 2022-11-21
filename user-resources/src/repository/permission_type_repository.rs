use x_core::application::PG_POOL;

use crate::entity::permission::PermissionTypeEntity;

/**
* @Author xYx
* @Date 2022-11-17 10:56:53
*/

pub struct PermissionTypeRepository;

impl PermissionTypeRepository {
    pub async fn insert(&self, mut record: PermissionTypeEntity) -> anyhow::Result<PermissionTypeEntity> {
        let id = sqlx::query_scalar!(
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
        .await?;
        record.id = id;
        Ok(record)
    }

    pub async fn update(&self, record: PermissionTypeEntity) -> anyhow::Result<PermissionTypeEntity> {
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
        .await?;
        Ok(record)
    }

    pub async fn delete(&self, id: &str) -> Result<u64, sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM permission_type
            WHERE id = $1
            "#,
            id
        )
        .execute(&*PG_POOL)
        .await
        .map(|r| r.rows_affected())
    }

    pub async fn fetch_all(&self) -> Result<Vec<PermissionTypeEntity>, sqlx::Error> {
        sqlx::query_as!(
            PermissionTypeEntity,
            r#"
            SELECT id, owner, name, description
            FROM permission_type
            "#,
        )
        .fetch_all(&*PG_POOL)
        .await
    }
}
