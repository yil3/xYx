use x_common::model::page::PageParam;

use crate::{
    dto::{user_dto::UserProfileDto, user_gourp_dto::UserGroupDto},
    repository::{user_group_repository::UserGroupRepository, user_user_group_repository::UserUserGroupRepository},
    vo::user_group_vo::UserGroupParam,
};

/**
* @Author xYx
* @Date 2022-11-22 09:23:44
*/
pub struct UserGroupService;

impl UserGroupService {
    pub async fn save(param: &UserGroupParam) -> anyhow::Result<UserGroupDto> {
        Ok(if param.id.is_none() {
            UserGroupRepository.insert(&param).await?
        } else {
            UserGroupRepository.update(&param).await?
        })
    }

    pub async fn page(param: &PageParam) -> anyhow::Result<Vec<UserGroupDto>> {
        Ok(UserGroupRepository.fetch_page(&param).await?)
    }

    pub async fn delete(id: &str) -> anyhow::Result<u64> {
        Ok(UserGroupRepository.delete(id).await?)
    }

    pub async fn get_users_by_group_id(param: &PageParam) -> anyhow::Result<Vec<UserProfileDto>> {
        let group_id = param.query.as_ref().expect("group_id is required");
        Ok(UserGroupRepository.fetch_users_by_group_id(group_id, param).await?)
    }

    pub async fn insert_users_by_group_id(group_id: &str, user_ids: &Vec<String>) -> anyhow::Result<u64> {
        Ok(UserUserGroupRepository
            .insert_users_by_usergroup(group_id, user_ids)
            .await?)
    }

    pub async fn remove_users_by_group_id(group_id: &str, user_ids: &Vec<String>) -> anyhow::Result<u64> {
        Ok(UserUserGroupRepository
            .remove_users_by_usergroup(group_id, user_ids)
            .await?)
    }
}
