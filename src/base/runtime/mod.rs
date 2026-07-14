mod demo;
pub mod mini_tokio;
pub mod mini_tokio_v2;
pub mod self_futures;

async fn task1() {
    println!("任务1开始");
    demo::sleep(3).await;
    println!("任务1完成");
}

async fn task2() {
    println!("任务2开始");
    demo::sleep(2).await;
    println!("任务2完成");
}

pub fn run() {
    demo::block_on(async {
        // 并发执行两个异步任务
        let t1 = task1();
        let t2 = task2();
        futures_util::join!(t1, t2);
    });
}
