use anyhow::Result;
use x_common::utils::vector::Tree;

use crate::{dto::role_dto::RoleDto, repository::role_repository::RoleRepository, vo::role_vo::RoleParam};

/**
* @Author xYx
* @Date 2022-11-23 14:17:22
*/

pub struct RoleService;

impl RoleService {
    pub async fn save(&self, param: &mut RoleParam) -> Result<RoleDto> {
        Ok(if param.id.is_none() {
            RoleDto::from(RoleRepository.insert(param).await?)
        } else {
            RoleDto::from(RoleRepository.update(param).await?)
        })
    }

    pub async fn get_all(&self) -> Result<Vec<RoleDto>> {
        let v: Vec<RoleDto> = RoleRepository.fetch_all().await?.into_iter().map(RoleDto::from).collect();
        Ok(v.to_tree())
    }

    pub async fn delete(&self, id: &str) -> Result<u64> {
        Ok(RoleRepository.delete(id).await?)
    }
}
