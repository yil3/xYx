
#[derive(Debug, Default, serde::Serialize)]
pub struct Page<T> {
    pub page: u32,
    pub size: u32,
    pub pages: u32,
    pub list: Vec<T>,
    pub total: i64,
    pub has_next: bool,
}

impl<T> Page<T> {
    pub fn build(list: Vec<T>, total: i64, limit: u32, offset: u32) -> Self {
        let page = offset + 1;
        let pages = total as u32 / limit + 1;
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
