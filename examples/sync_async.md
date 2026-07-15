# Rust 同步代码 ↔ 异步代码互通完整方案

分四大场景：

1. **同步函数里调用异步代码**（阻塞同步线程跑 async）
2. **异步函数里调用同步阻塞代码**（避免阻塞 Runtime 线程）
3. **同步回调 / 第三方库调用异步逻辑**
4. **跨线程同步等待异步结果**

底层核心：Tokio Runtime、`tokio::runtime::Runtime`、`tokio::task::spawn_blocking`、`block_on`、通道同步。

## 前置依赖 Cargo.toml

```
[dependencies]
tokio = { version = "1.0", features = ["full"] }
futures = "0.3"
```

## 一、同步代码中调用异步代码（主线程 / 普通 fn 同步环境）

### 1. 短生命周期：`Runtime::block_on`（最常用）

创建局部 runtime，阻塞当前同步线程执行异步逻辑，适合工具函数、main 同步入口。

```rust
use tokio::runtime::Runtime;

// 同步函数
fn sync_call_async() -> u32 {
    // 新建 runtime
    let rt = Runtime::new().unwrap();
    // block_on 阻塞同步线程，执行异步任务
    let res = rt.block_on(async {
        async_logic(10).await
    });
    res
}

// 异步逻辑
async fn async_logic(x: u32) -> u32 {
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    x * 2
}

fn main() {
    // 同步main
    let val = sync_call_async();
    println!("{}", val);
}
```

### 2. 全局复用 Runtime（性能更好，避免反复创建）

多位置同步调用异步，复用同一个 runtime：

```rust
use std::sync::OnceLock;
use tokio::runtime::Runtime;

static RT: OnceLock<Runtime> = OnceLock::new();

fn get_runtime() -> &'static Runtime {
    RT.get_or_init(|| Runtime::new().unwrap())
}

fn sync_func() {
    let rt = get_runtime();
    let num = rt.block_on(async_logic(5));
    println!("{}", num);
}
```

### 3. 带 #[tokio::main] 异步主函数内嵌套同步调用异步

不要在 async 里用 `block_on`，会死锁！

```rust
// 错误！async 内部 block_on 会死锁
#[tokio::main]
async fn bad() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {}) // 阻塞当前worker线程，runtime卡死
}
```

## 二、异步代码中调用同步阻塞代码（重点避坑）

Tokio 工作线程不允许长时间阻塞，否则整个 Runtime 吞吐量暴跌。

### 正确方案：`spawn_blocking` 扔到阻塞专用线程池

```rust
// 同步阻塞函数（文件IO、数据库同步驱动、CPU密集计算）
fn heavy_sync_task(n: u64) -> u64 {
    std::thread::sleep(std::time::Duration::from_millis(500));
    n * n
}

#[tokio::main]
async fn main() {
    // 将同步阻塞任务提交到阻塞线程池
    let join_handle = tokio::task::spawn_blocking(move || {
        heavy_sync_task(100)
    });
    // 等待同步任务完成
    let result = join_handle.await.unwrap();
    println!("同步计算结果：{}", result);
}
```

### 区分两种场景

1. **短同步计算（微秒级，无阻塞 IO）**：可直接调用，不用 spawn\_blocking
2. **长耗时 / IO 阻塞同步函数**：必须 spawn\_blocking，否则 runtime 卡死

## 三、同步回调里执行异步（如第三方同步库回调）

场景：第三方库注册同步回调，回调内部需要跑 async。
思路：Runtime 全局单例 + block\_on

```rust
use std::sync::OnceLock;
use tokio::runtime::Runtime;

static RT: OnceLock<Runtime> = OnceLock::new();
fn rt() -> &'static Runtime {
    RT.get_or_init(|| Runtime::new().unwrap())
}

// 模拟第三方同步回调
fn sync_callback() {
    // 回调内部调用异步
    let res = rt().block_on(async {
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        "async done"
    });
    println!("回调：{}", res);
}

// 模拟第三方库注册回调
fn register_cb(cb: fn()) {
    cb();
}

fn main() {
    register_cb(sync_callback);
}
```

## 四、异步等待同步线程结果（跨线程通信）

同步线程运行计算，异步任务等待结果，用 `tokio::sync::oneshot`

```rust
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    // 新建纯同步线程
    std::thread::spawn(move || {
        let val = 999;
        tx.send(val).unwrap();
    });

    // 异步等待同步线程的数据
    let data = rx.await.unwrap();
    println!("同步线程返回：{}", data);
}
```

## 五、双向互通完整示例（Pub/Sub 场景结合你之前的 Redis 代码）

同步函数触发发布，异步订阅消费；同时异步里调用同步工具函数

```rust
use mini_redis::{client, Result};
use futures::StreamExt;
use std::sync::OnceLock;
use tokio::runtime::Runtime;

static RT: OnceLock<Runtime> = OnceLock::new();
fn rt() -> &'static Runtime {
    RT.get_or_init(|| Runtime::new().unwrap())
}

// 同步对外接口，外部同步代码可直接调用发消息
pub fn sync_publish(num: u32) {
    rt().block_on(async move {
        let mut cli = client::connect("127.0.0.1:6379").await.unwrap();
        cli.publish("numbers", num.to_string()).await.unwrap();
    });
}

// 同步阻塞工具函数，在异步中调用
fn sync_log(s: &str) {
    std::thread::sleep(std::time::Duration::from_millis(10));
    println!("[同步日志] {}", s);
}

async fn subscribe_loop() -> Result<()> {
    let cli = client::connect("127.0.0.1:6379").await?;
    let sub = cli.subscribe(vec!["numbers".into()]).await?;
    let mut stream = sub.into_stream();
    tokio::pin!(stream);

    while let Some(msg) = stream.next().await {
        let data = msg?.into_string()?;
        // 异步中调用同步阻塞函数，丢进spawn_blocking
        tokio::task::spawn_blocking(move || {
            sync_log(&format!("收到消息：{}", data));
        }).await.unwrap();
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // 启动订阅协程
    tokio::spawn(async { subscribe_loop().await });
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // 同步函数发送消息
    sync_publish(1);
    sync_publish(2);
    sync_publish(3);

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    Ok(())
}
```

## 六、高频踩坑总结

1. **不要在 async 函数内使用 Runtime::block\_on**
   会阻塞当前 ```tokio worker``` 线程，造成死锁 / 性能暴跌。
2. **同步阻塞代码必须 spawn\_blocking**
   文件读写、同步数据库、sleep、CPU 密集计算全部走阻塞线程池。
3. **Runtime 不要频繁创建销毁**
   用 OnceLock 全局单例复用，减少系统开销。
4. **Redis Pub/Sub 时序问题**
   同步发消息前确保订阅已建立，可用 Barrier / 延时同步。
5. **多线程 Runtime 共享**
   Runtime 实现 Send + Sync，可跨线程作为静态变量使用。

## 七、简易记忆口诀

- 同步调异步：`Runtime.block_on`
- 异步调同步阻塞：`spawn_blocking`
- 异步内部禁止 block\_on
- 全局复用 Runtime 用 OnceLock