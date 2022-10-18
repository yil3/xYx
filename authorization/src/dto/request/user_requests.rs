use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct RegisterUserRequest {
    #[validate(required, length(min = 1), email(message = "email is invalid"))]
    pub email: Option<String>,
    #[validate(required, length(min = 1))]
    pub password: Option<String>,
    pub account: Option<String>,
    pub origin: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct LoginUserRequest {
    #[validate(required, length(min = 1), email(message = "email is invalid"))]
    pub email: Option<String>,
    #[validate(required, length(min = 1))]
    pub password: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UpdateUserRequest {
    pub id: String,
    pub password: Option<String>,
}
