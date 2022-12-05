use x_common::model::page::{Page, PageParam};

use crate::{
    dto::role_permission_group_dto::RoleGroupPageDto,
    repository::role_group_repository::RoleGroupRepository,
    vo::{role_group_vo::RoleGroupParam, role_vo::RoleParam},
};

use super::role_service::RoleService;

/**
* @Author xYx
* @Date 2022-12-04 22:33:03
*/

pub struct RoleGroupService;

impl RoleGroupService {
    pub async fn save(&self, record: &RoleGroupParam, user_id: &str) -> anyhow::Result<String> {
        if record.id.is_none() {
            let id = RoleGroupRepository.insert(record, user_id).await?;
            if !RoleService.is_exists(&id).await? {
                let role = RoleParam {
                    id: None,
                    owner: record.owner.clone(),
                    description: None,
                    status: Some(true),
                    name: "管理员".into(),
                    code: format!("{}_admin", record.code),
                    role_gourop_id: id.to_owned(),
                    parent_id: "0".into(),
                };
                RoleService.save(&role, user_id).await?;
            }
            Ok(id)
        } else {
            Ok(RoleGroupRepository.update(record, user_id).await?)
        }
    }

    pub async fn delete(&self, id: &str) -> Result<u64, sqlx::Error> {
        RoleGroupRepository.delete(id).await
    }

    pub async fn get_page(&self, param: &PageParam) -> Result<Page<RoleGroupPageDto>, sqlx::Error> {
        Ok(Page::build(
            param.page,
            param.size,
            RoleGroupRepository.fetch_page(param).await?,
        ))
    }
}
