use futures_util::StreamExt;
use mini_redis::{Result, client};
use std::sync::OnceLock;
use std::time::Duration;
use tokio::runtime::Runtime;

#[tokio::main]
async fn main() {
    // spawn_blocking 将任务投入阻塞线程
    let handle = tokio::task::spawn_blocking(|| logic(100));
    let val = handle.await.unwrap();
    println!("val: {}", val);

    let (tx, rx) = tokio::sync::oneshot::channel();
    std::thread::spawn(move || {
        tx.send(1).unwrap();
    });
    let val = rx.await.unwrap();
    println!("val: {}", val);

    // 启动订阅协程
    tokio::spawn(async { subscribe_loop().await });
    tokio::time::sleep(Duration::from_millis(100)).await;

    async_publish(1).await;
    async_publish(2).await;
    async_publish(3).await;
    tokio::time::sleep(Duration::from_secs(2)).await;
}

fn logic(n: u64) -> u64 {
    std::thread::sleep(std::time::Duration::from_millis(100));
    n * n
}

// 异步推送
async fn async_publish(num: u32) {
    let mut cli = client::connect("127.0.0.1:6379").await.unwrap();
    cli.publish("numbers", num.to_string().into())
        .await
        .unwrap();
}

// 同步阻塞工具函数，在异步中调用
fn sync_log(s: &str) {
    std::thread::sleep(std::time::Duration::from_millis(100));
    println!("[sync] log: {}", s);
}

async fn subscribe_loop() -> Result<()> {
    let cli = client::connect("127.0.0.1:6379").await?;
    let sub = cli.subscribe(vec!["numbers".into()]).await?;
    let stream = sub.into_stream();
    tokio::pin!(stream);

    while let Some(msg) = stream.next().await {
        let data = msg?;
        let s = format!("channel: {:?}, content: {:?}", data.channel, data.content);
        // 异步中调用同步阻塞函数，丢进spawn_blocking
        tokio::task::spawn_blocking(move || {
            sync_log(s.as_str());
        })
        .await?;
    }
    Ok(())
}
