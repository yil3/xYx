use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize)]
pub struct Page<T> {
    pub page: i64,
    pub size: i64,
    pub pages: i64,
    pub list: Vec<T>,
    pub total: i64,
    pub has_next: bool,
}

pub trait Pageable {
    fn total(&self) -> i64;
}

impl<T> Page<T>
where
    T: serde::Serialize,
    T: Pageable,
{
    pub fn build(list: Vec<T>, limit: i64, offset: i64) -> Self {
        let total = if list.len() > 0 {
            list.get(0).unwrap().total()
        } else {
            0
        };
        let page = offset + 1;
        let pages = total / limit;
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CommonPageRequest {
    pub page: i64,
    pub size: i64,
    pub query: Option<String>,
}

impl CommonPageRequest {
    pub fn offset(&self) -> i64 {
        self.size * (self.page - 1)
    }
    pub fn limit(&self) -> i64 {
        self.size
    }
}

impl Default for CommonPageRequest {
    fn default() -> Self {
        Self {
            page: 1,
            size: 10,
            query: None,
        }
    }
}
