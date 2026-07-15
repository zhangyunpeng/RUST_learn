use super::app::AppError;
use super::user::UserError;
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
