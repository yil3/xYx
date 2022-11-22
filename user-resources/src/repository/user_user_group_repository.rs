use sqlx::{query, query_scalar};
use x_core::application::PG_POOL;

use crate::entity::user_group::UserUserGroupEntity;

/**
* @Author xYx
* @Date 2022-11-22 15:27:13
*/

pub struct UserUserGroupRepository;

impl UserUserGroupRepository {
    pub async fn insert(&self, record: &UserUserGroupEntity) -> anyhow::Result<String> {
        Ok(query_scalar!(
            r#"
            INSERT INTO user_user_group (user_id, user_group_id)
            VALUES ($1, $2)
            returning id
            "#,
            record.user_id,
            record.user_group_id,
        )
        .fetch_one(&*PG_POOL)
        .await?)
    }

    pub async fn update(&self, record: &UserUserGroupEntity) -> anyhow::Result<String> {
        Ok(query_scalar!(
            r#"
            UPDATE user_user_group
            SET user_id = coalesce($1, user_id),
            user_group_id = coalesce($2, user_group_id)
            WHERE id = $3
            returning id
            "#,
            record.user_id,
            record.user_group_id,
            record.id,
        )
        .fetch_one(&*PG_POOL)
        .await?)
    }

    pub async fn delete(&self, id: &str) -> anyhow::Result<u64> {
        Ok(query!(
            r#"
            DELETE FROM user_user_group
            WHERE id = $1
            "#,
            id,
        )
        .execute(&*PG_POOL)
        .await?
        .rows_affected())
    }
}
