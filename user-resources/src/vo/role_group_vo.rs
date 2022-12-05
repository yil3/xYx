use serde::Deserialize;

/**
* @Author xYx
* @Date 2022-12-03 19:15:45
*/

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleGroupParam {
    pub id: Option<String>,
    pub owner: Option<String>,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
}
