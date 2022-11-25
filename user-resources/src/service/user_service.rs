use anyhow::Result;
use x_common::{
    errors::{XError, XResult},
    model::page::{Page, PageParam},
    utils::sucurity::SucurityUtils,
};

use crate::{dto::user_dto::UserDto, repository::user_repository::UserRepository, vo::user_vo::RegisterUserParam};
/**
* @Author xYx
* @Date 2022-11-22 15:49:42
*/
pub struct UserService;
impl UserService {
    pub async fn get_user_page(&self, param: &PageParam) -> Result<Page<UserDto>> {
        let page = UserRepository.fetch_page(param).await?;
        Ok(Page::build(param.page, param.size, page))
    }
    pub async fn register(&self, param: &mut RegisterUserParam) -> Result<String> {
        let hashed_password = SucurityUtils::hash_password(&param.password.as_ref().unwrap())?;
        param.password = Some(hashed_password);
        UserRepository.insert(param).await
    }

    pub async fn validate_user(&self, account: &str, password: &str) -> XResult<String> {
        let user = UserRepository.fetch_user_by_account(account).await?;
        if SucurityUtils::verify_password(&user.password, password)? {
            Ok(user.id)
        } else {
            Err(XError::InvalidLoginAttmpt)
        }
    }
}
