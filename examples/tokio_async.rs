use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    // let a = demo1();
    // a.await;

    let when = Instant::now() + Duration::from_secs(10);
    let future = Delay { when };
    let out = future.await;
    assert_eq!(out, "done");
}

#[allow(unused)]
async fn demo1() {
    println!("来自异步函数的打印");
    let _socket = TcpStream::connect("127.0.0.1:6379").await;
    println!("TCP 异步操作完成");
}

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("hello world");
            Poll::Ready("done")
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
