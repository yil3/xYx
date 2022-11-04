use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;


#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientResponse {
    pub id: String,
    pub secret: String,
    pub name: String,
    pub redirect_uri: String,
    pub scope: String,
    pub owner: Option<String>,
    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde_as(as = "Option<Rfc3339>")]
    pub updated_at: Option<OffsetDateTime>,
    pub total: Option<i64>,
}
