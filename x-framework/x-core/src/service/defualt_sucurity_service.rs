use std::sync::Arc;

use argon2::Config;

use crate::{config::AppConfig, errors::XError, traits::oauth2::security::ISecurity};

pub struct DefualtSecurityService {
    pub config: Arc<AppConfig>,
}

impl DefualtSecurityService {
    pub fn new(config: Arc<AppConfig>) -> Self {
        DefualtSecurityService { config }
    }
}

impl ISecurity for DefualtSecurityService {
    fn hash_password(&self, raw_password: &str) -> crate::errors::XResult<String> {
        let password_bytes = raw_password.as_bytes();
        let hashed_password =
            argon2::hash_encoded(password_bytes, self.config.argon_salt.as_bytes(), &Config::default()).unwrap();

        Ok(hashed_password)
    }

    fn verify_password(&self, stored_password: &str, attempted_password: String) -> crate::errors::XResult<bool> {
        let hashes_match = argon2::verify_encoded(stored_password, attempted_password.as_bytes())
            .map_err(|err| XError::InternalServerErrorWithContext(err.to_string()))?;

        Ok(hashes_match)
    }
}
