use std::sync::OnceLock;
use std::time::Duration;
use tokio::runtime::Runtime;

static RT: OnceLock<Runtime> = OnceLock::new();
fn main() {
    // Runtime block_on 调用异步函数
    println!("{}", sync_call_async());

    let rt = get_runtime();
    let num = rt.block_on(async_logic(10));
    println!("num: {}", num);

    register_cb(sync_callback);
}

fn get_runtime() -> &'static Runtime {
    RT.get_or_init(|| Runtime::new().unwrap())
}

fn sync_call_async() -> u32 {
    let rt = Runtime::new().unwrap();
    rt.block_on(async { async_logic(10).await })
}

async fn async_logic(u: u32) -> u32 {
    tokio::time::sleep(Duration::from_millis(100)).await;
    u * u
}

fn register_cb(cb: fn()) {
    cb();
}

fn sync_callback() {
    let res = get_runtime().block_on(async {
        tokio::time::sleep(Duration::from_millis(100)).await;
        "done".to_string()
    });
    println!("res: {}", res);
}
