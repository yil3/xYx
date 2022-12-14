use std::sync::Arc;

use x_common::errors::XResult;

// use mockall::automock;

/// A security service for handling JWT authentication.
pub type DynToken = Arc<dyn ITokenUtils + Send + Sync>;

// #[automock]
pub trait ITokenUtils {
    fn new_token(&self, user_id: i64, email: &str) -> XResult<String>;
    fn get_user_id_from_token(&self, token: String) -> XResult<i64>;
}
