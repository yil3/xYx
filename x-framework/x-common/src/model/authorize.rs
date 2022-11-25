use serde::{Serialize, Deserialize};

/**
* @Author xYx
* @Date 2022-11-24 22:03:57
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub user_id: String,
    pub exp: usize,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInfo {
    pub user_id: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

