use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserDto {
    pub id: String,
    pub email: String,
    pub mobile: String,
    pub token: String,
}

