use argon2::Config;
use std::env;

use crate::errors::XError;

pub struct SucurityUtils;

impl SucurityUtils {
    pub fn hash_password(raw_password: &str) -> crate::errors::XResult<String> {
        let password_bytes = raw_password.as_bytes();
        let hashed_password = argon2::hash_encoded(
            password_bytes,
            env::var("ARGON_SALT").unwrap().as_bytes(),
            &Config::default(),
        )
        .unwrap();
        Ok(hashed_password)
    }

    pub fn verify_password(stored_password: &str, attempted_password: String) -> crate::errors::XResult<bool> {
        let hashes_match = argon2::verify_encoded(stored_password, attempted_password.as_bytes())
            .map_err(|err| XError::InternalServerErrorWithContext(err.to_string()))?;
        Ok(hashes_match)
    }
}
