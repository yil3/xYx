use crate::{
    dto::permission_dto::PermissionDto, repository::permission_repository::PermissionRepository,
    vo::permission_vo::PermissionParam,
};
use anyhow::Result;

/**
* @Author xYx
* @Date 2022-11-23 15:21:12
*/
pub struct PermissionService;

impl PermissionService {
    pub async fn save(&self, param: &mut PermissionParam, userid: &str) -> Result<PermissionDto> {
        param.created_by = Some(userid.into());
        param.updated_by = Some(userid.into());
        Ok(if param.id.is_none() {
            PermissionDto::from(PermissionRepository.insert(param).await?)
        } else {
            PermissionDto::from(PermissionRepository.update(param).await?)
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
}
