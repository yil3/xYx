use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct R<T> {
    pub msg: Option<String>,
    pub success: bool,
    pub data: Option<T>,
}

impl<T> R<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        Self {
            msg: Some(String::from("操作成功")),
            success: true,
            data: Some(data),
        }
    }

    pub fn ok(msg: &str) -> Self {
        Self {
            msg: Some(String::from(msg)),
            success: true,
            data: None,
        }
    }

    pub fn fail(msg: &str) -> Self {
        Self {
            msg: Some(String::from(msg)),
            success: false,
            data: None,
        }
    }

    pub fn err() -> Self {
        Self {
            msg: Some(String::from("操作失败")),
            success: false,
            data: None,
        }
    }
}

