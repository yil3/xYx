use std::ops::Add;
use std::time::{Duration, SystemTime};
use std::env;

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;

use crate::errors::{XResult, XError};
use crate::traits::authen::token::ITokenUtils;


/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    user_id: i64,
    exp: usize,
}

pub struct DefaultTokenUtils;

impl ITokenUtils for DefaultTokenUtils {
    fn new_token(&self, user_id: i64, email: &str) -> XResult<String> {
        let from_now = Duration::from_secs(3600);
        let expired_future_time = SystemTime::now().add(from_now);
        let exp = OffsetDateTime::from(expired_future_time);

        let claims = Claims {
            sub: String::from(email),
            exp: exp.unix_timestamp() as usize,
            user_id,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(env::var("TOKEN_SECRET").unwrap().as_bytes()),
        )
        .map_err(|err| XError::InternalServerErrorWithContext(err.to_string()))?;

        Ok(token)
    }

    fn get_user_id_from_token(&self, token: String) -> XResult<i64> {
        let decoded_token = decode::<Claims>(
            token.as_str(),
            // &DecodingKey::from_secret(self.config.token_secret.as_bytes()),
            &DecodingKey::from_secret(env::var("TOKEN_SECRET").unwrap().as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|err| XError::InternalServerErrorWithContext(err.to_string()))?;

        Ok(decoded_token.claims.user_id)
    }
}
