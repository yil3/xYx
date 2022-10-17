use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;

#[serde_as]
#[derive(FromRow, Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClientEntity {
    pub id: String,
    pub secret: String,
    pub name: String,
    pub redirect_uri: String,
    pub scope: String,
    pub owner: Option<String>,
}
