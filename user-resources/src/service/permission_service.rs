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
}
