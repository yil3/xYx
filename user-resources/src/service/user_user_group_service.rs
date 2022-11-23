use crate::{repository::user_user_group_repository::UserUserGroupRepository, vo::user_group_vo::UserUserGroupBody};

/**
* @Author xYx
* @Date 2022-11-22 15:24:52
*/

pub struct UserUserGroupService;

impl UserUserGroupService {
    pub async fn save(param: &UserUserGroupBody) -> anyhow::Result<String> {
        let entity = param.into_entity();
        Ok(if entity.id.is_empty() {
            UserUserGroupRepository.insert(&entity).await?
        } else {
            UserUserGroupRepository.update(&entity).await?
        })
    }

    pub async fn delete(id: &str) -> anyhow::Result<u64> {
        Ok(UserUserGroupRepository.delete(id).await?)
    }
}
