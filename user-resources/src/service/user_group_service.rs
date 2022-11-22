use x_common::model::page::PageParam;

use crate::{
    domain::user_group_domain::UserGroupParam, dto::user_gourp_dto::UserGroupDto,
    repository::user_group_repository::UserGroupRepository,
};

/**
* @Author xYx
* @Date 2022-11-22 09:23:44
*/
pub struct UserGroupService;

impl UserGroupService {
    pub async fn save(param: &UserGroupParam) -> anyhow::Result<UserGroupDto> {
        let record = param.into_entity();
        Ok(if record.id.is_empty() {
            UserGroupRepository.insert(&record).await?
        } else {
            UserGroupRepository.update(&record).await?
        })
    }

    pub async fn page(param: PageParam) -> anyhow::Result<Vec<UserGroupDto>> {
        Ok(UserGroupRepository.fetch_page(&param).await?)
    }

    pub async fn delete(id: &str) -> anyhow::Result<u64> {
        Ok(UserGroupRepository.delete(id).await?)
    }
}
