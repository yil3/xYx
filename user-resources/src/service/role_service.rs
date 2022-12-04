use anyhow::Result;
use x_common::{utils::vector, model::page::PageParam};

use crate::{
    dto::role_dto::{RoleDto, RoleTreeDto, RolePageDto},
    repository::{role_repository::RoleRepository, user_role_repository::UserRoleRepository},
    vo::role_vo::RoleParam,
};

/**
* @Author xYx
* @Date 2022-11-23 14:17:22
*/

pub struct RoleService;

impl RoleService {
    pub async fn save(&self, param: &RoleParam, user_id: &str) -> Result<RoleDto> {
        Ok(if param.id.is_none() {
            RoleDto::from(RoleRepository.insert(param, user_id).await?)
        } else {
            RoleDto::from(RoleRepository.update(param, user_id).await?)
        })
    }

    pub async fn get_page(&self, param: &PageParam) -> Result<Vec<RolePageDto>> {
        Ok(RoleRepository.fetch_page(param).await?)
    }

    pub async fn tree(&self) -> Result<Vec<RoleTreeDto>> {
        let v = RoleRepository.fetch_all().await?;
        Ok(vector::to_tree(&v))
    }

    pub async fn delete(&self, id: &str) -> Result<u64> {
        Ok(RoleRepository.delete(id).await?)
    }

    pub async fn get_roles_by_user_id(&self, user_id: &str) -> Result<Vec<RoleDto>> {
        let v = RoleRepository.fetch_role_by_user(user_id).await?;
        Ok(v.iter().map(|x| RoleDto::from(x.to_owned())).collect())
    }

    pub async fn get_role_sign_by_user_id(&self, user_id: &str) -> Result<Vec<String>> {
        let v = RoleRepository.fetch_role_by_user(user_id).await?;
        Ok(v.iter().map(|x| x.name.to_owned()).collect())
    }

    // pub async fn insert_roles_to_user(&self, user_id: &str, role_ids: &Vec<String>) -> Result<u64> {
    //     Ok(UserRoleRepository.insert_role_by_user_id(user_id, role_ids).await?)
    // }

    pub async fn insert_users_to_role(&self, role_id: &str, user_ids: &Vec<String>) -> Result<u64> {
        Ok(UserRoleRepository.insert_user_by_role_id(role_id, user_ids).await?)
    }

    // pub async fn remove_roles_from_user(&self, user_id: &str, role_ids: &Vec<String>) -> Result<u64> {
    //     Ok(UserRoleRepository.remove_role_by_user_id(user_id, role_ids).await?)
    // }

    pub async fn remove_users_from_role(&self, role_id: &str, user_ids: &Vec<String>) -> Result<u64> {
        Ok(UserRoleRepository.remove_user_by_role_id(role_id, user_ids).await?)
    }
}
