use sqlx::{query, query_scalar};
use x_core::application::PG_POOL;

use crate::po::user_group::UserUserGroup;

/**
* @Author xYx
* @Date 2022-11-22 15:27:13
*/

pub struct UserUserGroupRepository;

impl UserUserGroupRepository {

    pub async fn insert_usergroups_by_user(&self, user_id: &str, group_ids: &Vec<String>) -> Result<u64, sqlx::Error> {
        let mut tx = PG_POOL.begin().await?;
        let mut count = 0;
        for group_id in group_ids {
            count += sqlx::query!(
                r#"
                    insert into user_user_group (user_id, user_group_id) values ($1, $2)
                "#,
                user_id,
                group_id
            )
            .execute(&mut tx)
            .await?
            .rows_affected();
        }
        tx.commit().await?;
        Ok(count)
    }

    pub async fn remove_usergroups_by_user(&self, user_id: &str, group_ids: &Vec<String>) -> Result<u64, sqlx::Error> {
        let mut tx = PG_POOL.begin().await?;
        let mut count = 0;
        for group_id in group_ids {
            count += sqlx::query!(
                r#"
                    delete from user_user_group where user_id = $1 and user_group_id = $2
                "#,
                user_id,
                group_id
            )
            .execute(&mut tx)
            .await?
            .rows_affected();
        }
        tx.commit().await?;
        Ok(count)
    }

    pub async fn insert_users_by_usergroup(&self, group_id: &str, user_ids: &Vec<String>) -> Result<u64, sqlx::Error> {
        let mut tx = PG_POOL.begin().await?;
        let mut count = 0;
        for user_id in user_ids {
            count += sqlx::query!(
                r#"
                    insert into user_user_group (user_id, user_group_id) values ($1, $2)
                "#,
                user_id,
                group_id
            )
            .execute(&mut tx)
            .await?
            .rows_affected();
        }
        tx.commit().await?;
        Ok(count)
    }

    pub async fn remove_users_by_usergroup(&self, group_id: &str, user_ids: &Vec<String>) -> Result<u64, sqlx::Error> {
        let mut tx = PG_POOL.begin().await?;
        let mut count = 0;
        for user_id in user_ids {
            count += sqlx::query!(
                r#"
                    delete from user_user_group where user_id = $1 and user_group_id = $2
                "#,
                user_id,
                group_id
            )
            .execute(&mut tx)
            .await?
            .rows_affected();
        }
        tx.commit().await?;
        Ok(count)
    }

    pub async fn insert(&self, user_id: &str, user_group_id: &str) -> Result<String, sqlx::Error> {
        query_scalar!(
            r#"
            INSERT INTO user_user_group (user_id, user_group_id)
            VALUES ($1, $2)
            returning id
            "#,
            user_id,
            user_group_id,
        )
        .fetch_one(&*PG_POOL)
        .await
    }

    pub async fn update(&self, record: &UserUserGroup) -> anyhow::Result<String> {
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

    pub async fn delete(&self, user_id: &str, user_group_id: &str) -> anyhow::Result<u64> {
        Ok(query!(
            r#"
            DELETE FROM user_user_group
            WHERE user_id = $1 and user_group_id = $2
            "#,
            user_id,
            user_group_id,
        )
        .execute(&*PG_POOL)
        .await?
        .rows_affected())
    }
}
