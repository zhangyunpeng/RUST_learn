use super::error::base::GlobalAppError;
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response<T> {
    code: u32,
    msg: String,
    data: Option<T>,
}

impl<T> Response<T> {
    pub fn success(code: u32, data: Option<T>) -> Self {
        Self {
            code,
            msg: "success".into(),
            data,
        }
    }

    pub fn fail(code: u32, msg: String) -> Self {
        Self {
            code,
            msg,
            data: None,
        }
    }
}

impl IntoResponse for GlobalAppError {
    fn into_response(self) -> axum::response::Response {
        let (http_status, biz_code, msg) = (self.http_status(), self.biz_code(), self.to_string());
        let resp = Response::<()>::fail(biz_code, msg);
        (http_status, Json(resp)).into_response()
    }
}
pub type ApiResult<T> = Result<T, GlobalAppError>;
