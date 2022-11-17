use x_core::application::PG_POOL;

use crate::entity::role::UserGroupRoleEntity;

/**
* @Author xYx
* @Date 2022-11-17 10:55:28
*/

pub struct UserGroupRoleRepository;

impl UserGroupRoleRepository {
    pub async fn insert(&self, record: &UserGroupRoleEntity) -> Result<u64, sqlx::Error> {
        sqlx::query("INSERT INTO user_group_role (user_group_id, role_id) VALUES ($1, $2)")
            .bind(&record.user_group_id)
            .bind(&record.role_id)
            .execute(&*PG_POOL)
            .await
            .map(|r| r.rows_affected())
    }

    pub async fn delete(&self, id: &str) -> Result<u64, sqlx::Error> {
        sqlx::query("DELETE FROM user_group_role WHERE id = $1")
            .bind(id)
            .execute(&*PG_POOL)
            .await
            .map(|r| r.rows_affected())
    }
}
