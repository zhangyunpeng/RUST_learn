use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response<T> {
    code: u32,
    msg: String,
    data: Option<T>,
}

impl<T> Response<T> {
    pub fn success(code: u32, data: Option<T>) -> Self {
        Self {code, msg: "success".into(), data}
    }

    pub fn fail(code: u32, msg: String) -> Self {
        Self {code, msg, data: None}
    }
}