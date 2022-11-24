use anyhow::Result;
use x_common::{
    errors::{XError, XResult},
    utils::sucurity::SucurityUtils,
};

use crate::{
    vo::user_vo::RegisterUserParam, po::user::User, repository::user_repository::UserRepository,
};
/**
* @Author xYx
* @Date 2022-11-22 15:49:42
*/
pub struct UserService;
impl UserService {
    pub async fn register(&self, param: &mut RegisterUserParam) -> Result<String> {
        let hashed_password = SucurityUtils::hash_password(&param.password.as_ref().unwrap())?;
        param.password = Some(hashed_password);
        UserRepository.insert(param).await
    }

    pub async fn validate_user(&self, account: &str, password: &str) -> XResult<User> {
        let user = UserRepository.fetch_user_by_account(account).await?;
        if SucurityUtils::verify_password(&user.password, password)? {
            Ok(user)
        } else {
            Err(XError::InvalidLoginAttmpt)
        }
    }
    
}
