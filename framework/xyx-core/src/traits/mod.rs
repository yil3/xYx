pub mod users;
pub mod oauth2;

use crate::errors::XResult;
use async_trait::async_trait;

#[async_trait]
pub trait IBaseService<T> {
    async fn find_by_id(&self, id: &str) -> XResult<T>;
    async fn delete_by_id(&self, id: &str) -> XResult<T>;
    async fn update(&self, data: &T) -> XResult<T>;
    async fn create(&self, data: &T) -> XResult<T>;
}
