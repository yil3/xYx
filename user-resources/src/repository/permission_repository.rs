use x_core::application::PG_POOL;

use crate::entity::permission::PermissionEntity;

/**
* @Author xYx
* @Date 2022-11-21 10:29:46
*/
pub struct PermissionRepository;

impl PermissionRepository {
    pub async fn insert(&self, mut record: PermissionEntity) -> Result<PermissionEntity, sqlx::Error> {
        let id = sqlx::query_scalar!(
            r#"
            insert into sys_permission 
            (owner, name, code, role_id, description, created_by, updated_by) 
            values ($1, $2, $3, $4, $5, $6, $7)
            returning id
        "#,
            &record.owner,
            &record.name,
            &record.code,
            &record.role_id,
            &record.description,
            &record.created_by,
            &record.updated_by
        )
        .fetch_one(&*PG_POOL)
        .await?;
        record.id = id;
        Ok(record)
    }

    pub async fn update(&self, record: PermissionEntity) -> Result<PermissionEntity, sqlx::Error> {
        sqlx::query!(
            r#"
            update sys_permission set
            owner = coalesce($1, owner),
            name = coalesce($2, name),
            code = coalesce($3, code),
            role_id = coalesce($4, role_id),
            description = coalesce($5, description),
            updated_by = coalesce($6, updated_by)
            where id = $7
        "#,
            &record.owner,
            &record.name,
            &record.code,
            &record.role_id,
            &record.description,
            &record.updated_by,
            &record.id
        )
        .execute(&*PG_POOL)
        .await?;
        Ok(record)
    }

    pub async fn delete(&self, id: &str) -> Result<u64, sqlx::Error> {
        sqlx::query!(
            r#"
            delete from sys_permission where id = $1
        "#,
            id
        )
        .execute(&*PG_POOL)
        .await
        .map(|r| r.rows_affected())
    }

    pub async fn fetch_by_role_id(&self, role_id: &str) -> Result<Vec<PermissionEntity>, sqlx::Error> {
        sqlx::query_as!(
            PermissionEntity,
            r#"
            select * from sys_permission where role_id = $1
        "#,
            role_id
        )
        .fetch_all(&*PG_POOL)
        .await
    }
}
