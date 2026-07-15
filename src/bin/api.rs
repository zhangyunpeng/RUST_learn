use axum::{Json, Router, routing::post};
use futures::StreamExt;
use mini_redis::{Result, client};
use serde::Serialize;
use std::sync::OnceLock;
use tokio::runtime::Runtime;

const MINI_REDIS_ADDRESS: &str = "127.0.0.1:6379";
const MINI_REDIS_CHANNEL: &str = "numbers";

static RT: OnceLock<Runtime> = OnceLock::new();

fn rt() -> &'static Runtime {
    RT.get_or_init(|| Runtime::new().expect("Create runtime failed"))
}

// ==================== Redis 发布接口 ====================
/// 同步接口：纯同步代码可直接调用（如第三方同步回调、脚本逻辑）
pub fn sync_publish(num: u32) {
    rt().block_on(async {
        let mut cli = client::connect(MINI_REDIS_ADDRESS).await.unwrap();
        cli.publish(MINI_REDIS_CHANNEL, num.to_string().into())
            .await
            .unwrap();
    });
}

/// 异步接口：HTTP服务、tokio spawn 内部使用（无block\_on，无嵌套运行时报错）
pub async fn async_publish(num: u32) -> Result<()> {
    let mut cli = client::connect(MINI_REDIS_ADDRESS).await?;
    cli.publish(MINI_REDIS_CHANNEL, num.to_string().into())
        .await?;
    Ok(())
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ChannelLog {
    channel: String,
    message: String,
}

// ==================== 同步阻塞日志 ====================
fn sync_log(s: &str) {
    println!("log: {}", s);
}

// ==================== Redis 订阅循环（后台监听消息） ====================
async fn subscribe_loop() -> Result<()> {
    let cli = client::connect(MINI_REDIS_ADDRESS).await?;
    let subscribe = cli.subscribe(vec![MINI_REDIS_CHANNEL.into()]).await?;
    let message = subscribe.into_stream();
    tokio::pin!(message);
    while let Some(msg) = message.next().await {
        let data = msg?;
        let log = ChannelLog {
            channel: data.channel,
            message: String::from_utf8(data.content.to_vec())?,
        };
        let data_str = serde_json::to_string(&log)?;
        let _ = tokio::spawn(async move {
            sync_log(&data_str);
        })
        .await;
    }

    Ok(())
}

// ==================== Axum HTTP API 处理逻辑 ====================
#[derive(Serialize)]
struct Resp {
    code: u32,
    msg: String,
}

/// HTTP接口：接收数字，异步推送到Redis频道
async fn publish_api(Json(num): Json<u32>) -> Json<Resp> {
    match async_publish(num).await {
        Ok(_) => Json(Resp {
            code: 0,
            msg: format!("推送成功，num={num}"),
        }),
        Err(e) => Json(Resp {
            code: 500,
            msg: format!("推送失败: {e}"),
        }),
    }
}

/// 构建HTTP路由
fn create_router() -> Router {
    Router::new().route("/publish", post(publish_api))
}

// ==================== 异步主逻辑（服务+订阅后台任务） ====================
async fn run_server() -> Result<()> {
    // 后台启动Redis订阅循环
    tokio::spawn(async move {
        if let Err(e) = subscribe_loop().await {
            eprintln!("订阅循环异常退出: {e}");
        }
    });

    println!("HTTP API 服务启动: 127.0.0.1:8080");
    let router = create_router();
    // 绑定端口启动axum服务
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await?;
    Ok(())
}

fn main() {
    let rt = rt();
    rt.block_on(async {
        if let Err(e) = run_server().await {
            eprintln!("服务运行异常: {e}");
        }
    });
}
