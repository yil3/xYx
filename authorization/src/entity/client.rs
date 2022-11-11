use serde::{Serialize, Deserialize};
use time::OffsetDateTime;

/**
* @Author xYx
* @Date 2022-09-26 10:50:23
*/

#[derive(Serialize, Deserialize)]
pub struct ClientEntity {
    pub id: String,
    pub secret: String,
    pub name: String,
    pub redirect_uri: String,
    pub scope: String,
    pub owner: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
