use x_core::application::PG_POOL;

/**
* @Author xYx
* @Date 2022-11-17 10:55:28
*/

pub struct UserGroupRoleRepository;

impl UserGroupRoleRepository {

    pub async fn insert_role_by_usergroup_id(&self, group_id: &str, role_ids: &Vec<String>) -> Result<u64, sqlx::Error> {
        let mut tx = PG_POOL.begin().await?;
        let mut count = 0;
        for role_id in role_ids {
            count += sqlx::query!(
                r#"
                    insert into user_group_role (user_group_id, role_id) values ($1, $2)
                "#,
                group_id,
                role_id
            )
            .execute(&mut tx)
            .await?
            .rows_affected();
        }
        tx.commit().await?;
        Ok(count)
    }

    pub async fn remove_role_by_usergroup_id(&self, group_id: &str, role_ids: &Vec<String>) -> Result<u64, sqlx::Error> {
        let mut tx = PG_POOL.begin().await?;
        let mut count = 0;
        for role_id in role_ids {
            count += sqlx::query!(
                r#"
                    delete from user_group_role where user_group_id = $1 and role_id = $2
                "#,
                group_id,
                role_id
            )
            .execute(&mut tx)
            .await?
            .rows_affected();
        }
        tx.commit().await?;
        Ok(count)
    }
}
