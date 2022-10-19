use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserDto {
    pub id: String,
    pub account: String,
    pub nickname: Option<String>,
    pub origin: Option<String>,
    pub total: Option<i64>,
}

