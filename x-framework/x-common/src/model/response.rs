use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
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
            msg: Default::default(),
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

    pub fn ok_msg(data: T, msg: &str) -> Self {
        Self {
            msg: Some(String::from(msg)),
            success: true,
            data: Some(data),
        }
    }

    pub fn fail() -> Self {
        Self {
            msg: Default::default(),
            success: false,
            data: None,
        }
    }

    pub fn error(msg: &str) -> Self {
        Self {
            msg: Some(String::from(msg)),
            success: false,
            data: None,
        }
    }
}

