use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("用户ID {id} 不存在")]
    UserNotExist { id: String },
    #[error("密码校验不匹配")]
    PasswordMismatch,
    #[error("用户age {age} 错误")]
    UserAgeError { age: i8 },
    #[error("用户默认错误{0}")]
    UserDefault(String),
    #[error("数据库异常： {0}")]
    MysqlError(#[from] sqlx::Error),
}

impl UserError {
    pub fn code(&self) -> u32 {
        match self {
            UserError::UserDefault(_) => 30000,
            UserError::UserNotExist { id: _ } => 30001,
            UserError::PasswordMismatch => 30002,
            UserError::UserAgeError { age: _ } => 30003,
            UserError::MysqlError(_) => 30004,
        }
    }

    pub fn default(msg: &str) -> UserError {
        UserError::UserDefault(msg.to_string())
    }
}
