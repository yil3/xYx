use x_core::application::PG_POOL;

use crate::{po::permission::Permission, vo::permission_vo::PermissionParam};

/**
* @Author xYx
* @Date 2022-11-21 10:29:46
*/
pub struct PermissionRepository;

impl PermissionRepository {
    pub async fn fetch_permission_sign_by_user(&self, user_id: &str) -> Result<Vec<Option<String>>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                select p.name || '::' || pt.name as name 
                from sys_permission p
                left join role_permission rp on rp.permission_id = p.id
                left join permission_type pt on pt.id = rp.permission_type_id

                left join sys_role r on r.id = rp.role_id
                left join user_role ur on ur.role_id = r.id

                left join sys_role r1 on r1.parent_id = r.id

                left join user_user_group uug on uug.user_id = ur.user_id
                left join user_group_role ugr on ugr.user_group_id = uug.user_group_id
                where ur.user_id = $1 and uug.user_id = $1
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
    pub async fn insert(&self, record: &PermissionParam, user_id: &str) -> Result<Permission, sqlx::Error> {
        sqlx::query_as!(
            Permission,
            r#"
                insert into sys_permission 
                (owner, name, code, description, group_id, created_by, updated_by) 
                values ($1, $2, $3, $4, $5, $6, $6)
                returning *
            "#,
            record.owner,
            record.name,
            record.code,
            record.description,
            record.group_id,
            user_id,
        )
        .fetch_one(&*PG_POOL)
        .await
    }

    pub async fn update(&self, record: &PermissionParam, user_id: &str) -> Result<Permission, sqlx::Error> {
        sqlx::query_as!(
            Permission,
            r#"
              update sys_permission set
              owner = coalesce($1, owner),
              name = coalesce($2, name),
              code = coalesce($3, code),
              description = coalesce($4, description),
              updated_by = $5
              where id = $5
              returning *
          "#,
            record.owner,
            record.name,
            record.code,
            record.description,
            user_id,
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
                select p.* from sys_permission p 
                left join role_permission rp on rp.permission_id = p.id 
                where rp.role_id = $1
            "#,
            role_id
        )
        .fetch_all(&*PG_POOL)
        .await
    }
}
