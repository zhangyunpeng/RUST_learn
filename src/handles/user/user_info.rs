use crate::global::response::{ApiResult, Response};
use crate::service::user::user_info as userInfoService;
use axum::Json;
use axum::extract::Query;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct UserInfo {
    user_id: String,
    name: String,
    age: u8,
}

#[derive(Deserialize)]
pub struct UserInfoReq {
    pub user_id: String,
}

#[derive(Serialize)]
pub struct UserInfoResponse {
    id: u64,
    user_id: String,
    username: String,
    name: String,
    age: u8,
    phone: String,
    email: String,
    status: i8,
    deleted: i8,
}

pub async fn user_info(
    Query(req): Query<UserInfoReq>,
) -> ApiResult<Json<Response<UserInfoResponse>>> {
    let user_id = req.user_id;
    let data = userInfoService::user_info(&user_id).await?;
    let resp = UserInfoResponse {
        id: data.id,
        user_id: data.user_id,
        username: data.username,
        name: data.name,
        age: data.age,
        phone: data.phone,
        email: data.email,
        status: data.status,
        deleted: data.deleted,
    };
    Ok(Json(Response::success(0, Some(resp))))
}
