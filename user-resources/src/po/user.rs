use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use x_common::utils::code;

/**
* @Author xYx
* @Date 2022-11-16 14:46:05
*/

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub origin: Option<String>,
    pub password: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

pub struct UserAccount {
    pub id: String,
    pub owner: String,
    pub account: String,
    pub open_id: Option<String>,
    pub category: AccountCategory,
    pub created_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
    pub deleted: bool,
}

pub struct UserInfo {
    pub id: String,
    pub owner: String,
    pub nickname: String,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub updated_at: OffsetDateTime,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: code::unique_id(),
            origin: Default::default(),
            password: String::from("123456"),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}

#[derive(sqlx::Type)]
#[repr(i32)]
pub enum AccountCategory {
    Account = 0,
    Phone = 1,
    Email = 2,
    WeChat = 3,
    QQ = 4,
    Weibo = 5,
}
