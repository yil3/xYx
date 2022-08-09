use x_core::{entity::user::UserEntity, errors::XResult, traits::authen::repository::IAuthenRepository};

pub struct Dao;

impl IAuthenRepository for Dao {
    fn find_user_by_mobile(&self, mobile: &str) -> XResult<Option<UserEntity>> {
        todo!()
    }
}
