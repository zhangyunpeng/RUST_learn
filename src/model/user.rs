use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};
// use bcrypt::{hash, DEFAULT_COST};
use crate::global::error::user::UserError;

/// 用户表 sys_user 实体映射
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")] // JSON 返回小驼峰 userID
pub struct User {
    pub id: u64,
    pub user_id: String,
    pub username: String,
    pub password: String,
    pub name: String,
    pub age: u8,
    pub phone: String,
    pub email: String,
    pub status: i8,
    // pub create_time: chrono::DateTime<chrono::Local>,
    // pub update_time: chrono::DateTime<chrono::Local>,
    pub create_time: chrono::DateTime<chrono::Utc>,
    pub update_time: chrono::DateTime<chrono::Utc>,
    pub deleted: i8,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserDTO {
    pub username: String,
    pub password: String,
    pub name: String,
    pub age: Option<u8>,
    pub phone: String,
    pub email: String,
}

pub async fn create(pool: &MySqlPool, data: CreateUserDTO) -> Result<u64, UserError> {
    let user_id = format!("u_{}", uuid::Uuid::new_v4());
    // let pwd_hash = match hash(&data.password, DEFAULT_COST) {
    //     Ok(pwd_hash) => pwd_hash,
    //     Err(e) => return Err(GlobalAppError::UserModel(UserDefault(e.to_string())))
    // };
    let result = sqlx::query(
        r#"
            INSERT INTO user (user_id, username, password, name, age, phone, email)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
    )
    .bind(user_id)
    .bind(data.username)
    .bind(data.password)
    .bind(data.name)
    .bind(data.age)
    .bind(data.phone)
    .bind(data.email)
    .execute(pool)
    .await?;

    Ok(result.last_insert_id() as u64)
}

pub async fn user_by_user_id(pool: &MySqlPool, user_id: &str) -> Result<Option<User>, UserError> {
    let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, user_id, username, password, name, age, phone, email, status, create_time, update_time, deleted
            FROM user
            WHERE user_id = ? AND deleted = 0
        "#
        )
            .bind(user_id)
            .fetch_optional(pool)
            .await?;
    Ok(user)
}
