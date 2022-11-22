use anyhow::Result;
use x_common::{
    errors::{XError, XResult},
    utils::sucurity::SucurityUtils,
};

use crate::{
    domain::user_domain::RegisterUserParam, entity::user::UserEntity, repository::user_repository::UserRepository,
};
/**
* @Author xYx
* @Date 2022-11-22 15:49:42
*/
pub struct UserService;
impl UserService {
    pub async fn register(&self, input: &mut RegisterUserParam) -> Result<String> {
        let hashed_password = SucurityUtils::hash_password(&input.password.as_ref().unwrap())?;
        input.password = Some(hashed_password);
        UserRepository.insert(input).await
    }

    pub async fn validate_user(&self, account: &str, password: &str) -> XResult<UserEntity> {
        let user = UserRepository.fetch_user_by_account(account).await?;
        if SucurityUtils::verify_password(&user.password, password)? {
            Ok(user)
        } else {
            Err(XError::InvalidLoginAttmpt)
        }
    }
}
