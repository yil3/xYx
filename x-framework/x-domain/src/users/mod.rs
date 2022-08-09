use serde::{Deserialize, Serialize};

pub mod requests;
pub mod responses;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserDto {
    pub id: String,
    pub username: String,
    pub email: String,
    pub mobile: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub token: String,
}
