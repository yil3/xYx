use std::sync::Arc;

use mockall::automock;

use crate::errors::XResult;

/// A security service for handling JWT authentication.
pub type DynToken = Arc<dyn IToken + Send + Sync>;

#[automock]
pub trait IToken {
    fn new_token(&self, user_id: i64, email: &str) -> XResult<String>;
    fn get_user_id_from_token(&self, token: String) -> XResult<i64>;
}
