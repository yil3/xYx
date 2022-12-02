use serde::{de::DeserializeOwned, Deserialize, Serialize};

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

impl<T> Page<T> {
    pub fn build_with_total(total: i64, page: i64, size: i64, list: Vec<T>) -> Self {
        let pages = if total % size == 0 {
            total / size
        } else {
            total / size + 1
        };
        Self {
            page,
            size,
            list,
            pages,
            total,
            has_next: page < pages,
        }
    }
}

impl<T> Page<T>
where
    T: Serialize,
    T: Pageable,
{
    pub fn build(page: i64, size: i64, list: Vec<T>) -> Self {
        let total = if list.len() > 0 {
            list.get(0).unwrap().total()
        } else {
            0
        };
        let pages = (total as f64 / size as f64).ceil() as i64;
        Self {
            page,
            size,
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
    pub fn query_to<T>(&self) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        Ok(serde_json::from_str(&self.query.as_ref().expect("query is required"))?)
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
