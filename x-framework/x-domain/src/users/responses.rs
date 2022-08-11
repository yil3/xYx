use serde::{Deserialize, Serialize};

use crate::users::UserDto;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserAuthenicationResponse {
    pub user: UserDto,
}

impl UserAuthenicationResponse {
    pub fn new(
        id: String,
        account: String,
        email: String,
        // unfortunately, while our implementation returns thes optional fields as empty strings,
        // the realworld demo API enables nullable serializing by default, so we have to wrap these
        // strings as `Option` option values for now
        mobile: String,
        bio: Option<String>,
        image: Option<String>,
        token: String,
    ) -> Self {
        UserAuthenicationResponse {
            user: UserDto {
                id,
                account,
                email,
                mobile,
                bio,
                image,
                token,
            },
        }
    }
}
