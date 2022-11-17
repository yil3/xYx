use x_core::application::PG_POOL;

use crate::entity::role::UserRoleEntity;

/**
* @Author xYx
* @Date 2022-11-17 10:46:28
*/

pub struct UserRoleRepository;

impl UserRoleRepository {
    pub async fn insert(&self, record: &UserRoleEntity) -> Result<u64, sqlx::Error> {
        sqlx::query("INSERT INTO user_role (user_id, role_id) VALUES ($1, $2)")
            .bind(&record.user_id)
            .bind(&record.role_id)
            .execute(&*PG_POOL)
            .await
            .map(|r| r.rows_affected())
    }

    pub async fn delete(&self, id: &str) -> Result<u64, sqlx::Error> {
        sqlx::query("DELETE FROM user_role WHERE id = $1")
            .bind(id)
            .execute(&*PG_POOL)
            .await
            .map(|r| r.rows_affected())
    }
}
