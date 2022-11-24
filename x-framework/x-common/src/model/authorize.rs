use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub user_id: String,
    pub exp: usize,
}


/**
* @Author xYx
* @Date 2022-11-24 22:03:57
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInfo {
    pub user_id: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub jwt_token: String,
}
