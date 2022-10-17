use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientResponse {
    pub id: String,
    pub secret: String,
    pub name: String,
    pub redirect_uri: String,
    pub scope: String,
    pub owner: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
