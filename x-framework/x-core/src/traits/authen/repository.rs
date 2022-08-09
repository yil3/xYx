use async_trait::async_trait;

use crate::{errors::XResult, entity::user::UserEntity};

#[async_trait]
pub trait IAuthenRepository {
    fn find_user_by_mobile(&self, mobile: &str) -> XResult<Option<UserEntity>>;
}
