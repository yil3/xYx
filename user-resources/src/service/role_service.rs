use anyhow::Result;

use crate::{
    po::role::Role, repository::role_repository::RoleRepository,
    vo::role_vo::RoleParam,
};

/**
* @Author xYx
* @Date 2022-11-23 14:17:22
*/

pub struct RoleService;

impl RoleService {
    pub async fn save(&self, param: &mut RoleParam) -> Result<Role> {
        Ok(if param.id.is_none() {
            RoleRepository.insert(param).await?
        } else {
            RoleRepository.update(param).await?
        })
    }
}
