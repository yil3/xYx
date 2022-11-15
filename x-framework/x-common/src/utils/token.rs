use std::env;
use std::ops::Add;
use std::time::{Duration, SystemTime};

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use sqlx::types::time::OffsetDateTime;

use crate::errors::{XError, XResult};
use crate::model::authorize::Claims;

pub struct TokenUtils;

impl TokenUtils {
    pub fn generate_jwt_token(user_id: String, sub: &str) -> XResult<String> {
        let exp = env::var("TOKEN_EXPIRED").unwrap().parse::<u64>().unwrap_or(3600 * 24);
        let from_now = Duration::from_secs(exp);
        let expired_future_time = SystemTime::now().add(from_now);
        let exp = OffsetDateTime::from(expired_future_time);

        let claims = Claims {
            sub: String::from(sub),
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

    pub fn fetch_current_user_id(token: &str) -> XResult<String> {
        let decoded_token = decode::<Claims>(
            token,
            &DecodingKey::from_secret(env::var("TOKEN_SECRET").unwrap().as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|err| XError::InternalServerErrorWithContext(err.to_string()))?;

        Ok(decoded_token.claims.user_id)
    }

    pub fn fetch_claims(token: &str) -> XResult<Claims> {
        let decoded_token = decode::<Claims>(
            token,
            &DecodingKey::from_secret(env::var("TOKEN_SECRET").unwrap().as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|err| XError::InternalServerErrorWithContext(err.to_string()))?;
        Ok(decoded_token.claims)
    }
}
