// use crate::global::error::{base::GlobalAppError};
use crate::global::response::{ApiResult, Response};
use axum::Json;
use serde::Serialize;
use crate::{instance, model};

#[derive(Serialize)]
pub struct UserInfo {
    user_id: String,
    name: String,
    age: u8,
}

pub async fn user_info() -> ApiResult<Json<Response<model::user::User>>> {
    // let age = 0;
    // if age == 0 {
    //     return Err(GlobalAppError::UserError(UserError::default(
    //         "自定义错误内容",
    //     )));
    // }

    let user_id = "u_10001".to_string();
    let data = model::user::user_by_user_id(instance::mysql_pool(), &user_id).await?;

    // let result = UserInfo {
    //     user_id: "111".to_string(),
    //     name: "sunshine".to_string(),
    //     age: 18,
    // };
    Ok(Json(Response::success(0, data)))
}
