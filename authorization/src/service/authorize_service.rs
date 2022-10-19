use anyhow::anyhow;
use anyhow::Result;
use axum::extract::Query;
use x_common::{
    errors::{XError, XResult},
    utils,
};

use crate::dto::{
    request::{authorize_requests::AuthorizeRequest, token_requests::TokenRequest},
    response::token_responses::TokenResponses,
};
use crate::entity::token::TokenEntity;

use super::token_service::TokenService;
use super::user_service::UserService;

pub struct AuthorizeService;

impl AuthorizeService {
    pub fn authorize(&self, request: Query<AuthorizeRequest>) -> String {
        let mut url = request.redirect_uri.clone();
        if request.client_id.is_empty() {
            url.push_str("?error=invalid_request");
            return url;
        }
        if request.response_type == "code" {
            let code = self.generate_code();
            url.push_str(format!("?code={code}").as_str());
        }
        if request.response_type == "token" {
            let token = self.generate_token();
            url.push_str(format!("?token={token}").as_str());
        }
        if request.state.is_some() {
            url.push_str(format!("&state={}", request.state.as_ref().unwrap()).as_str());
        }
        url
    }
    pub fn generate_token(&self) -> String {
        // TODO: 生成token，写入数据库
        utils::code::uuid()
    }
    pub fn generate_code(&self) -> String {
        // TODO: 生成code，缓存数据设置过期时间
        utils::code::uuid()
    }
    pub fn verify_client(&self, _client_id: &str) {
        // TODO: 验证client_id
    }

    pub async fn token(&self, request: &TokenRequest) -> XResult<TokenResponses> {
        let mut token = TokenResponses::default();
        if request.grant_type == "authorization_code" {
            let code = request.code.as_ref().unwrap();
            if self.validate_code(code) {
                token.access_token = self.generate_token();
            }
        }
        if request.grant_type == "password" {
            // TODO: 验证用户名密码
            let account = request.username.as_ref().unwrap();
            let password = request.password.as_ref().unwrap();
            match UserService.validate_user(account, password).await {
                Ok(_) => {
                    let entity = TokenService
                        .generate_token(&request.client_id, &account, &request.scope)
                        .await
                        .map_err(|_| XError::AnyhowError(anyhow!("insert token error")))?;
                    return Ok(entity.into_dto());
                },
                Err(_) => return Err(XError::AnyhowError(anyhow!("用户名或密码错误"))),
            }
        }
        if request.grant_type == "client_credentials" {
            // TODO: 客户端模式
        }
        Ok(token)
    }

    pub fn refresh_token(&self) {}

    pub fn validate_token(&self) {}

    pub fn validate_code(&self, _code: &str) -> bool {
        true
    }
}
