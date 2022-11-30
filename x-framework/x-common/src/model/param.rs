use serde::Deserialize;

/**
* @Author xYx
* @Date 2022-11-30 09:45:31
*/

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Param {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub query: Option<String>,
}

impl Param {
    pub fn query_to<T>(&self) -> anyhow::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        Ok(serde_json::from_str(&self.query.as_ref().expect("query is required"))?)
    }
}

