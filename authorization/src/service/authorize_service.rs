use crate::{
    dto::token_dto::TokenDto,
    vo::{authorize_vo::AuthorizeParam, token_vo::TokenParam},
};
use anyhow::anyhow;
use redis::Commands;
use x_common::{
    errors::{XError, XResult},
    model::response::R,
    utils,
};
use x_core::application::Application;

use super::{client_service::ClientService, token_service::TokenService};

pub struct AuthorizeService;

impl AuthorizeService {
    pub async fn authorize(&self, params: &AuthorizeParam, userid: &str) -> XResult<String> {
        let mut url = params.redirect_uri.clone();
        if params.client_id.is_empty() {
            url.push_str("?error=client_is_empty");
            return Ok(url);
        }
        if self.verify_client(&params.client_id).await.is_err() {
            url.push_str("?error=invalid_client");
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

    pub async fn token(&self, params: &TokenParam) -> XResult<TokenDto> {
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
            let user_resourc_serve = Application::config()
                .user_resources_server
                .to_owned()
                .expect("application.yml config user_resources_server url is not empty")
                .url;
            let user_id = reqwest::Client::new()
                .post(format!("{}/user/validate", &user_resourc_serve))
                .header("content-type", "application/json")
                .body(serde_json::json!({ "username": account, "password": password }).to_string())
                .send()
                .await
                .map_err(|e| XError::AnyhowError(anyhow!(e.to_string())))?
                .json::<R<String>>()
                .await
                .map_err(|e| XError::AnyhowError(anyhow!(e.to_string())))?
                .data
                .expect("user not found");
            return Ok(self.generate_token(&params.client_id, &user_id, &params.scope).await?);
            // match self.validate_user(account, password).await {
            //     Ok(userid) => return Ok(self.generate_token(&params.client_id, &userid, &params.scope).await?),
            //     Err(_) => return Err(XError::InvalidLoginAttmpt),
            // }
        }
        if params.grant_type == "client_credentials" {
            // TODO: 客户端模式
        }
        Err(XError::InternalServerError)
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> XResult<TokenDto> {
        Ok(TokenService.refresh_token(refresh_token).await?.into_dto())
    }

    pub async fn signout(&self, access_token: &str) -> XResult<()> {
        Application::redis()
            .del(access_token)
            .map_err(|e| XError::AnyhowError(anyhow!(e)))?;
        TokenService.remove_expired_token().await;
        Ok(())
    }

    async fn generate_token(&self, client_id: &str, userid: &str, scope: &Option<String>) -> XResult<TokenDto> {
        let entity = TokenService
            .generate_token(client_id, userid, scope)
            .await
            .map_err(|e| XError::AnyhowError(anyhow!(e)))?;
        Ok(entity.into_dto())
    }

    async fn generate_code(&self, userid: &str) -> XResult<String> {
        let key = utils::code::uuid();
        Application::redis()
            .set_ex(&key, userid, 600)
            .map_err(|e| XError::AnyhowError(anyhow!(e)))?;
        Ok(key)
    }

    async fn verify_client(&self, client_id: &str) -> XResult<()> {
        match ClientService.find_by_id(client_id).await {
            Ok(_client) => Ok(()),
            Err(_) => Err(XError::AnyhowError(anyhow!("client is invalid"))),
        }
    }

    // async fn validate_user(&self, account: &str, password: &str) -> XResult<String> {
    //     let row = TokenRepository
    //         .fetch_user_by_account(account)
    //         .await
    //         .map_err(|e| XError::AnyhowError(anyhow!(e)))?;
    //     if let Ok(stored_password) = row.try_get::<String, &str>("password") {
    //         if SucurityUtils::verify_password(&stored_password, password)? {
    //             Ok(row.get::<String, &str>("id"))
    //         } else {
    //             Err(XError::InvalidLoginAttmpt)
    //         }
    //     } else {
    //         Err(XError::InvalidLoginAttmpt)
    //     }
    // }

    fn validate_code(&self, code: &str) -> Option<String> {
        match Application::redis().get(code) {
            Ok(userid) => Some(userid),
            _ => None,
        }
    }
}
