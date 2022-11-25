use crate::{
    dto::permission_dto::PermissionDto, repository::{permission_repository::PermissionRepository, permission_type_repository::PermissionTypeRepository},
    vo::permission_vo::{PermissionParam, PermissionTypeParam}, po::permission::PermissionType,
};
use anyhow::Result;

/**
* @Author xYx
* @Date 2022-11-23 15:21:12
*/
pub struct PermissionService;

impl PermissionService {
    pub async fn save(&self, param: &mut PermissionParam, userid: &str) -> Result<PermissionDto> {
        Ok(if param.id.is_none() {
            PermissionDto::from(PermissionRepository.insert(param, userid).await?)
        } else {
            PermissionDto::from(PermissionRepository.update(param, userid).await?)
        })
    }

    pub async fn delete_by_id(&self, id: &str) -> Result<u64> {
        Ok(PermissionRepository.delete(id).await?)
    }

    pub async fn get_by_role(&self, role_id: &str) -> Result<Vec<PermissionDto>> {
        Ok(PermissionRepository
            .fetch_by_role_id(role_id)
            .await?
            .into_iter()
            .map(|it| PermissionDto::from(it))
            .collect())
    }

    pub async fn get_permission_sign_by_user(&self, user_id: &str) -> Result<Vec<Option<String>>> {
        Ok(PermissionRepository.fetch_permission_sign_by_user(user_id).await?)
    }

    pub async fn save_permission_type(&self, param: &PermissionTypeParam) -> Result<PermissionType> {
        if param.id.is_none() {
            Ok(PermissionTypeRepository.insert(param).await?)
        } else {
            Ok(PermissionTypeRepository.update(param).await?)
        }
    }

    pub async fn delete_permission_type(&self, id: &str) -> Result<u64> {
        Ok(PermissionTypeRepository.delete(id).await?)
    }

    pub async fn get_permission_type(&self) -> Result<Vec<PermissionType>> {
        Ok(PermissionTypeRepository.fetch_all().await?)
    }
}
