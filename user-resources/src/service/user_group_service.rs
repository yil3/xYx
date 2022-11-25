use anyhow::Result;
use x_common::model::page::{Page, PageParam};

use crate::{
    dto::{user_dto::UserProfileDto, user_gourp_dto::UserGroupDto},
    repository::{
        user_group_repository::UserGroupRepository, user_group_role_repository::UserGroupRoleRepository,
        user_user_group_repository::UserUserGroupRepository,
    },
    vo::user_group_vo::UserGroupParam,
};

/**
* @Author xYx
* @Date 2022-11-22 09:23:44
*/
pub struct UserGroupService;

impl UserGroupService {
    pub async fn save(&self, param: &UserGroupParam, user_id: &str) -> Result<UserGroupDto> {
        Ok(if param.id.is_none() {
            UserGroupRepository.insert(&param, user_id).await?
        } else {
            UserGroupRepository.update(&param, user_id).await?
        })
    }

    pub async fn get_user_group_page(&self, param: &PageParam) -> Result<Page<UserGroupDto>> {
        Ok(Page::build(
            param.page,
            param.size,
            UserGroupRepository.fetch_page(param).await?,
        ))
    }

    pub async fn delete(&self, id: &str) -> anyhow::Result<u64> {
        Ok(UserGroupRepository.delete(id).await?)
    }

    pub async fn get_user_page_by_group_id(&self, param: &PageParam) -> Result<Page<UserProfileDto>> {
        let group_id = param.query.as_ref().expect("group_id is required");
        let list = UserGroupRepository.fetch_users_by_group_id(group_id, param).await?;
        Ok(Page::build(param.page, param.size, list))
    }

    pub async fn insert_users_by_group_id(&self, group_id: &str, user_ids: &Vec<String>) -> Result<u64> {
        Ok(UserUserGroupRepository
            .insert_users_by_usergroup(group_id, user_ids)
            .await?)
    }

    pub async fn remove_users_by_group_id(&self, group_id: &str, user_ids: &Vec<String>) -> Result<u64> {
        Ok(UserUserGroupRepository
            .remove_users_by_usergroup(group_id, user_ids)
            .await?)
    }

    pub async fn insert_role_by_user_group(&self, user_group_id: &str, role_ids: &Vec<String>) -> Result<u64> {
        Ok(UserGroupRoleRepository
            .insert_role_by_usergroup_id(user_group_id, role_ids)
            .await?)
    }

    pub async fn remove_role_from_user_group(&self, user_group_id: &str, role_ids: &Vec<String>) -> Result<u64> {
        Ok(UserGroupRoleRepository
            .remove_role_by_usergroup_id(user_group_id, role_ids)
            .await?)
    }
}
