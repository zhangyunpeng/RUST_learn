use super::app::AppError;
use super::user::UserError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GlobalAppError {
    #[error("用户业务异常：{0}")]
    UserModel(#[from] UserError),
    #[error("服务基础组件错误：{0}")]
    AppError(#[from] AppError),
}

impl From<std::io::Error> for GlobalAppError {
    fn from(e: std::io::Error) -> Self {
        Self::AppError(AppError::AppStartError(e.to_string()))
    }
}

impl GlobalAppError {
    pub fn biz_code(&self) -> u32 {
        match self {
            GlobalAppError::UserModel(e) => e.code(),
            GlobalAppError::AppError(e) => e.code(),
        }
    }

    pub fn http_status(&self) -> StatusCode {
        match self {
            GlobalAppError::AppError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::OK,
        }
    }
}
