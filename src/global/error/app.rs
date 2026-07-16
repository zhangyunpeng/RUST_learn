use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Tokio Runtime Error")]
    TokioRuntimeError,
    #[error("服务启动错误{0}")]
    AppStartError(String),
    #[error("数据库异常： {0}")]
    MysqlError(#[from] sqlx::Error),
}

impl AppError {
    pub fn code(&self) -> u32 {
        match self {
            AppError::TokioRuntimeError => 10001,
            AppError::AppStartError(_) => 10002,
            AppError::MysqlError(_) => 10004,
        }
    }
}
