use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Result<T> {
    pub msg: Option<String>,
    pub success: bool,
    pub data: Option<T>,
}

impl<T> Result<T>
where
    T: Serialize,
{
    pub fn ok(data: T) -> Self {
        Self {
            msg: Some(String::from("操作成功")),
            success: true,
            data: Some(data),
        }
    }

    pub fn ok_msg(data: T, msg: String) -> Self {
        Self {
            msg: Some(String::from(msg)),
            success: true,
            data: Some(data),
        }
    }

    pub fn msg(msg: String) -> Self {
        Self {
            msg: Some(String::from(msg)),
            success: true,
            data: None,
        }
    }

    pub fn set_msg(mut self, msg: String) -> Self {
        self.msg = Some(msg);
        self
    }

    pub fn set_success(mut self, success: bool) -> Self {
        self.success = success;
        self
    }

    pub fn set_data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
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
