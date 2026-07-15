use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Tokio Runtime Error")]
    TokioRuntimeError,
    #[error("服务启动错误{0}")]
    AppStartError(String),
}
