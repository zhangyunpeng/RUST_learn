// use axum::{extract::Path, http::StatusCode, response::Json, routing::get, Router};
// use serde::Serialize;
// use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
// use std::time::Duration;
// use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

// 数据库返回结构体
// #[derive(Serialize, sqlx::FromRow)]
// struct User {
//     id: u64,
//     username: String,
// }
//
// // 全局数据库连接池
// async fn create_pool() -> MySqlPool {
//     MySqlPoolOptions::new()
//         .max_connections(10)
//         .connect("mysql://root:123456@127.0.0.1:3306/test_db")
//         .await
//         .unwrap()
// }
//
// // 用户查询接口
// async fn get_user(Path(user_id): Path<u64>, pool: axum::extract::State<MySqlPool>) -> Result<Json<User>, StatusCode> {
//     // sqlx预编译，杜绝SQL注入
//     let user = sqlx::query_as!(
//         User,
//         "SELECT id, username FROM user WHERE id = ?",
//         user_id
//     )
//         .fetch_one(&*pool)
//         .await
//         .map_err(|_| StatusCode::NOT_FOUND)?;
//
//     Ok(Json(user))
// }

#[tokio::main]
async fn main() {
    // // 初始化DB池
    // let pool = create_pool().await;
    //
    // // 限流中间件：单IP每秒最多10次请求
    // let governor_conf = GovernorConfigBuilder::default()
    //     .per_second(10)
    //     .burst_size(5)
    //     .finish()
    //     .unwrap();
    // let governor_layer = GovernorLayer::new(governor_conf);
    //
    // // 路由注册 + 全局中间件
    // let app = Router::new()
    //     .route("/user/:id", get(get_user))
    //     .with_state(pool)
    //     .layer(governor_layer) // 限流
    //     .layer(tower_http::logging::LoggerLayer::new()); // 请求日志
    //
    // println!("服务启动 0.0.0.0:8080");
    // axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
    //     .serve(app.into_make_svc())
    //     .await
    //     .unwrap();
}
