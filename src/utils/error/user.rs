use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("用户ID {id} 不存在")]
    UserNotExist { id: String },
    #[error("密码校验不匹配")]
    PasswordMismatch,
}
