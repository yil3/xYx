use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct AuthorizeParam {
    pub client_id: String,
    pub redirect_uri: String,
    pub response_type: String,
    pub scope: Option<String>,
    pub state: Option<String>,
}
 
