use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct R<T> {
    pub message: Option<String>,
    pub success: bool,
    pub data: Option<T>,
}

impl R<()> {
    pub fn message(msg: &str) -> Self {
        Self {
            message: Some(msg.into()),
            success: true,
            data: None,
        }
    }

    pub fn error(msg: &str) -> Self {
        Self {
            message: Some(msg.into()),
            success: false,
            data: None,
        }
    }

}

impl<T> R<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        Self {
            message: Default::default(),
            success: true,
            data: Some(data),
        }
    }

    pub fn fail(msg: &str) -> Self {
        Self {
            message: Some(msg.into()),
            success: false,
            data: None,
        }
    }

    pub fn success_msg(data: T, msg: &str) -> Self {
        Self {
            message: Some(msg.into()),
            success: true,
            data: Some(data),
        }
    }

    pub fn fail_msg(data: T, msg: &str) -> Self {
        Self {
            message: Some(msg.into()),
            success: false,
            data: Some(data),
        }
    }

}

