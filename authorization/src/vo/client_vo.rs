use serde::Deserialize;
use time::OffsetDateTime;

use crate::entity::client::ClientEntity;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientParam {
    pub id: Option<String>,
    pub secret: String,
    pub name: String,
    pub redirect_uri: String,
    pub scope: String,
    pub owner: Option<String>,
}

impl ClientParam {
    pub fn into_entity(&self) -> ClientEntity {
        ClientEntity {
            id: self.id.to_owned().unwrap_or_default(),
            secret: self.secret.to_owned(),
            name: self.name.to_owned(),
            redirect_uri: self.redirect_uri.to_owned(),
            scope: self.scope.to_owned(),
            status: true,
            owner: self.owner.to_owned(),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}
