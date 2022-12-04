use x_common::model::page::{Page, PageParam};

use crate::{
    dto::role_permission_group_dto::RolePermissionGroupPageDto,
    repository::role_permission_group_repository::RolePermissionGroupRepository,
    vo::{role_permission_vo::RolePermissionGroupParam, role_vo::RoleParam},
};

use super::role_service::RoleService;

/**
* @Author xYx
* @Date 2022-12-04 22:33:03
*/

pub struct RolePermissionGroupService;

impl RolePermissionGroupService {
    pub async fn save(&self, record: &RolePermissionGroupParam, user_id: &str) -> anyhow::Result<String> {
        if record.id.is_none() {
            let id = RolePermissionGroupRepository.insert(record, user_id).await?;
            let role = RoleParam {
                id: None,
                owner: record.owner.clone(),
                description: None,
                status: Some(true),
                name: Some("管理员".into()),
                code: Some("admin".into()),
                gourop_id: Some(id.to_owned()),
                parent_id: Some("0".into()),
            };
            RoleService.save(&role, user_id).await?;
            Ok(id)
        } else {
            Ok(RolePermissionGroupRepository.update(record, user_id).await?)
        }
    }

    pub async fn delete(&self, id: &str) -> Result<u64, sqlx::Error> {
        RolePermissionGroupRepository.delete(id).await
    }

    pub async fn get_page(&self, param: &PageParam) -> Result<Page<RolePermissionGroupPageDto>, sqlx::Error> {
        Ok(Page::build(
            param.page,
            param.size,
            RolePermissionGroupRepository.fetch_page(param).await?,
        ))
    }
}
