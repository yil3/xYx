use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(FromRow)]
#[sqlx(rename_all = "camelCase")]
#[derive(Deserialize, Serialize)]
pub struct ClientEntity {
    pub id: String,
    pub secret: String,
    pub name: String,
    pub redirect_uri: String,
    pub scope: String,
    pub owner: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
