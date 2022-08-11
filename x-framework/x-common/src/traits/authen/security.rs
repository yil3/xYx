use std::sync::Arc;

// use mockall::automock;

use crate::errors::XResult;

/// A security service for handling JWT authentication.
pub type DynSecurity = Arc<dyn ISecurityUtils + Send + Sync>;

// #[automock]
pub trait ISecurityUtils {
    fn hash_password(&self, raw_password: &str) -> XResult<String>;
    fn verify_password(&self, stored_password: &str, attempted_password: String) -> XResult<bool>;
}
