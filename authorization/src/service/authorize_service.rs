use crate::{
    dto::{
        authorize_dto::AuthorizeParam,
        token_dto::{TokenParam, TokenRecord},
    },
    repository::token_repository::TokenRepository,
};
use anyhow::anyhow;
use axum::extract::Query;
use redis::Commands;
use sqlx::Row;
use x_common::{
    errors::{XError, XResult},
    utils::{self, sucurity::SucurityUtils},
};
use x_core::application::Application;

use super::{client_service::ClientService, token_service::TokenService};

pub struct AuthorizeService;

impl AuthorizeService {
    pub async fn authorize(&self, params: Query<AuthorizeParam>, userid: &str) -> XResult<String> {
        let mut url = params.redirect_uri.clone();
        if params.client_id.is_empty() {
            url.push_str("?error=invalid_request");
            return Ok(url);
        }
        if params.response_type == "code" {
            let code = self.generate_code(&userid).await?;
            url.push_str(format!("?code={code}").as_str());
        }
        if params.response_type == "token" {
            let token = self
                .generate_token(&params.client_id, userid, &params.scope)
                .await?
                .access_token;
            url.push_str(format!("?token={token}").as_str());
        }
        if params.state.is_some() {
            url.push_str(format!("&state={}", params.state.as_ref().unwrap()).as_str());
        }
        Ok(url)
    }

    pub async fn generate_token(
        &self,
        client_id: &str,
        userid: &str,
        scope: &Option<String>,
    ) -> XResult<TokenRecord> {
        let entity = TokenService
            .generate_token(client_id, userid, scope)
            .await
            .map_err(|e| XError::AnyhowError(anyhow!(e)))?;
        Ok(entity.into_dto())
    }

    pub async fn generate_code(&self, userid: &str) -> XResult<String> {
        let key = utils::code::uuid();
        Application::redis()
            .set_ex(&key, userid, 600)
            .map_err(|e| XError::AnyhowError(anyhow!(e)))?;
        Ok(key)
    }

    pub async fn verify_client(&self, client_id: &str) -> XResult<()> {
        match ClientService.find_by_id(client_id).await {
            Ok(_client) => Ok(()),
            Err(_) => Err(XError::AnyhowError(anyhow!("client is invalid"))),
        }
    }

    pub async fn token(&self, params: &TokenParam) -> XResult<TokenRecord> {
        if params.grant_type == "authorization_code" {
            let code = params.code.as_ref().unwrap();
            match self.validate_code(code) {
                Some(userid) => return Ok(self.generate_token(&params.client_id, &userid, &params.scope).await?),
                None => return Err(XError::AnyhowError(anyhow!("invalid code"))),
            }
        }
        if params.grant_type == "password" {
            let account = params.username.as_ref().unwrap();
            let password = params.password.as_ref().unwrap();
            match self.validate_user(account, password).await {
                Ok(userid) => return Ok(self.generate_token(&params.client_id, &userid, &params.scope).await?),
                Err(_) => return Err(XError::InvalidLoginAttmpt),
            }
        }
        if params.grant_type == "client_credentials" {
            // TODO: 客户端模式
        }
        Err(XError::InternalServerError)
    }

    pub async fn validate_user(&self, account: &str, password: &str) -> XResult<String> {
        let row = TokenRepository.fetch_user_by_account(account).await?;
        if let Ok(stored_password) = row.try_get::<String, &str>("password") {
            if SucurityUtils::verify_password(&stored_password, password.to_string())? {
                Ok(row.get::<String, &str>("id"))
            } else {
                Err(XError::InvalidLoginAttmpt)
            }
        } else {
            Err(XError::InvalidLoginAttmpt)
        }
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> XResult<TokenRecord> {
        Ok(TokenService.refresh_token(refresh_token).await?.into_dto())
    }

    pub fn validate_code(&self, code: &str) -> Option<String> {
        Some(Application::redis().get(code).unwrap())
    }
}
