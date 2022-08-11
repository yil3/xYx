
use serde::{Deserialize, Serialize};

pub mod responses;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ProfileDto {
    pub account: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}
