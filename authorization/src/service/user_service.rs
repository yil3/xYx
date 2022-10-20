use anyhow::Result;
use x_common::{utils::sucurity::SucurityUtils, errors::{XResult, XError}};

use crate::{dto::request::user_requests::RegisterUserRequest, repository::user_repository::UserRepository, entity::user::UserEntity};

pub struct UserService;
impl UserService {
    pub async fn register(&self, input: &RegisterUserRequest) -> Result<String> {
        let mut input = input.to_owned();
        let hashed_password = SucurityUtils::hash_password(&input.password.unwrap())?;
        input.password = Some(hashed_password);
        UserRepository.insert(&input).await
    }

    pub async fn validate_user(&self, account: &str, password: &str) -> XResult<UserEntity> {
        let user = UserRepository.fetch_user_by_account(account).await?;
        if SucurityUtils::verify_password(&user.password, password.to_string())? {
            Ok(user)
        } else {
            Err(XError::InvalidLoginAttmpt)
        }
    }
}
