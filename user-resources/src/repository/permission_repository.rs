use x_core::application::PG_POOL;

use crate::{po::permission::Permission, vo::permission_vo::PermissionParam};

/**
* @Author xYx
* @Date 2022-11-21 10:29:46
*/
pub struct PermissionRepository;

impl PermissionRepository {
    pub async fn fetch_permission_sign_by_user(user_id: &str) -> Result<Vec<Option<String>>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                select p.name || '::' || pt.name as name from role_permission rp
                left join sys_permission p on rp.permission_id = p.id
                left join permission_type pt on pt.id = rp.permission_type_id
                where rp.role_id in (select role_id from user_role where user_id = $1)
                order by p.name
            "#,
            user_id
        )
        .fetch_all(&*PG_POOL)
        .await?
        .iter()
        .map(move |x| x.name.to_owned())
        .collect();
        Ok(result)
    }
    pub async fn insert(&self, record: &PermissionParam) -> Result<Permission, sqlx::Error> {
        sqlx::query_as!(
            Permission,
            r#"
                insert into sys_permission 
                (owner, name, code, role_id, description, created_by, updated_by) 
                values ($1, $2, $3, $4, $5, $6, $7)
                returning *
            "#,
            record.owner,
            record.name,
            record.code,
            record.role_id,
            record.description,
            record.created_by,
            record.updated_by
        )
        .fetch_one(&*PG_POOL)
        .await
    }

    pub async fn update(&self, record: &PermissionParam) -> Result<Permission, sqlx::Error> {
        sqlx::query_as!(
            Permission,
            r#"
              update sys_permission set
              owner = coalesce($1, owner),
              name = coalesce($2, name),
              code = coalesce($3, code),
              role_id = coalesce($4, role_id),
              description = coalesce($5, description),
              updated_by = coalesce($6, updated_by)
              where id = $7
              returning *
          "#,
            record.owner,
            record.name,
            record.code,
            record.role_id,
            record.description,
            record.updated_by,
            record.id
        )
        .fetch_one(&*PG_POOL)
        .await
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

    pub async fn fetch_by_role_id(&self, role_id: &str) -> Result<Vec<Permission>, sqlx::Error> {
        sqlx::query_as!(
            Permission,
            r#"
                select * from sys_permission where role_id = $1
            "#,
            role_id
        )
        .fetch_all(&*PG_POOL)
        .await
    }
}
