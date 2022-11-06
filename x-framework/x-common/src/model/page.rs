use serde::{Deserialize, Serialize};

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub page: i64,
    pub size: i64,
    pub list: Vec<T>,
    pub pages: i64,
    pub total: i64,
    pub has_next: bool,
}

pub trait Pageable {
    fn total(&self) -> i64;
}

impl<T> Page<T>
where
    T: Serialize,
    T: Pageable,
{
    pub fn build(list: Vec<T>, limit: i64, offset: i64) -> Self {
        let total = if list.len() > 0 {
            list.get(0).unwrap().total()
        } else {
            0
        };
        let page = offset + 1;
        let pages = if (total / limit) == 0 { 1 } else { total / limit };
        Self {
            page,
            size: limit,
            pages,
            list,
            total,
            has_next: pages > page,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PageParam {
    pub page: i64,
    pub size: i64,
    pub query: Option<String>,
}

impl<'a> PageParam {
    pub fn offset(&self) -> i64 {
        self.size * (self.page - 1)
    }
    pub fn limit(&self) -> i64 {
        self.size
    }
    pub fn query_to<T>(&'a self) -> anyhow::Result<T>
    where
        T: Deserialize<'a>,
    {
        if let Some(query) = &self.query {
            Ok(serde_json::from_str(query)?)
        } else {
            Err(anyhow::anyhow!("deserialize query failed"))
        }
    }
}

impl Default for PageParam {
    fn default() -> Self {
        Self {
            page: 1,
            size: 10,
            query: None,
        }
    }
}
