use crate::dto::{
    request::{authorize_requests::AuthorizeRequest, token_requests::TokenRequest},
    response::token_responses::TokenResponses,
};
use anyhow::anyhow;
use axum::extract::Query;
use x_common::{
    errors::{XError, XResult},
    utils,
};

use super::user_service::UserService;
use super::{client_service::ClientService, token_service::TokenService};

pub struct AuthorizeService;

impl AuthorizeService {
    pub async fn authorize(&self, params: Query<AuthorizeRequest>, userid: &str) -> XResult<String> {
        let mut url = params.redirect_uri.clone();
        if params.client_id.is_empty() {
            url.push_str("?error=invalid_request");
            return Ok(url);
        }
        if params.response_type == "code" {
            let code = self.generate_code(&userid).await;
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
    ) -> XResult<TokenResponses> {
        let entity = TokenService
            .generate_token(client_id, userid, scope)
            .await
            .map_err(|e| XError::AnyhowError(anyhow!(e)))?;
        Ok(entity.into_dto())
    }
    pub async fn generate_code(&self, _userid: &str) -> String {
        // TODO: 生成code，缓存数据设置过期时间
        utils::code::uuid()
    }
    pub async fn verify_client(&self, client_id: &str) -> XResult<()> {
        match ClientService.find_by_id(client_id).await {
            Ok(_client) => Ok(()),
            Err(_) => Err(XError::AnyhowError(anyhow!("client is invalid"))),
        }
    }

    pub async fn token(&self, params: &TokenRequest) -> XResult<TokenResponses> {
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
            match UserService.validate_user(account, password).await {
                Ok(user) => return Ok(self.generate_token(&params.client_id, &user.id, &params.scope).await?),
                Err(_) => return Err(XError::InvalidLoginAttmpt),
            }
        }
        if params.grant_type == "client_credentials" {
            // TODO: 客户端模式
        }
        Err(XError::InternalServerError)
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> XResult<TokenResponses> {
        Ok(TokenService.refresh_token(refresh_token).await?.into_dto())
    }

    pub fn validate_token(&self) {}

    pub fn validate_code(&self, _code: &str) -> Option<String> {
        Some("".to_string())
    }
}
