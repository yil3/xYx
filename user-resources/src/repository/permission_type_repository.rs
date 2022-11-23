use x_core::application::PG_POOL;

use crate::po::permission::PermissionType;

/**
* @Author xYx
* @Date 2022-11-17 10:56:53
*/

pub struct PermissionTypeRepository;

impl PermissionTypeRepository {
    pub async fn insert(&self, record: PermissionType) -> Result<PermissionType, sqlx::Error> {
        sqlx::query_as!(
            PermissionType,
            r#"
            INSERT INTO permission_type (owner, name, description)
            VALUES ($1, $2, $3)
            returning *
            "#,
            record.owner,
            record.name,
            record.description
        )
        .fetch_one(&*PG_POOL)
        .await
    }

    pub async fn update(&self, record: PermissionType) -> Result<PermissionType, sqlx::Error> {
        sqlx::query_as!(
            PermissionType,
            r#"
            UPDATE permission_type
            SET name = coalesce($1, name), description = coalesce($2, description)
            WHERE id = $3
            returning *
            "#,
            &record.name,
            &record.description,
            &record.id
        )
        .fetch_one(&*PG_POOL)
        .await
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

    pub async fn fetch_all(&self) -> Result<Vec<PermissionType>, sqlx::Error> {
        sqlx::query_as!(
            PermissionType,
            r#"
            SELECT id, owner, name, description
            FROM permission_type
            "#,
        )
        .fetch_all(&*PG_POOL)
        .await
    }
}
