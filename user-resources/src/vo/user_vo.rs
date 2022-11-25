use serde::Deserialize;
use validator::Validate;

/**
* @Author xYx
* @Date 2022-11-22 15:45:37
*/

#[derive(Deserialize, Validate)]
pub struct RegisterUserParam {
    #[validate(required, length(min = 1), email(message = "email is invalid"))]
    pub email: Option<String>,
    #[validate(required, length(min = 1))]
    pub password: Option<String>,
    pub account: String,
    pub nickname: Option<String>,
    pub origin: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct LoginUserParam {
    #[validate(required, length(min = 1))]
    pub username: Option<String>,
    #[validate(required, length(min = 1))]
    pub password: Option<String>,
    pub category: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateUserParam {
    pub id: String,
    pub password: Option<String>,
}
