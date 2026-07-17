use crate::global::error::user::UserError;
use crate::instance;
use crate::model::user;
use crate::model::user::User;

pub async fn user_info(user_id: &str) -> Result<User, UserError> {
    let res = user::user_by_user_id(instance::mysql_pool(), user_id).await?;
    if let Some(user) = res {
        return Ok(user);
    }
    Err(UserError::UserNotExist {
        id: user_id.to_string(),
    })
}
