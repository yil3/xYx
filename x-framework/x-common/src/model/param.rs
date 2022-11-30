use serde::Deserialize;

/**
* @Author xYx
* @Date 2022-11-30 09:45:31
*/

#[derive(Deserialize)]
pub struct Id(pub String);

impl Id {
    pub fn get(&self) -> &String {
        &self.0
    }
}
