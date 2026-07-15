use mini_redis::{Result, client};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio::sync::Barrier;
use tokio_stream::{Stream, StreamExt};
// #[tokio::main]
// async fn main() {
//     let mut stream = tokio_stream::iter(vec![1, 2, 3]);
//     while let Some(i) = stream.next().await {
//         println!("{:?}", i);
//     }
// }

#[tokio::main]
async fn main() -> Result<()> {
    let barrier = Arc::new(Barrier::new(2));

    let sub_barrier = barrier.clone();
    let sub_handle = tokio::spawn(async move {
        let _ = subscribe(sub_barrier).await;
    });

    let pub_barrier = barrier.clone();
    let pub_handle = tokio::spawn(async move {
        let _ = publish(pub_barrier).await;
    });

    pub_handle.await?;
    sub_handle.await?;

    Ok(())
}

async fn publish(barrier: Arc<Barrier>) -> Result<()> {
    barrier.wait().await;
    let mut client = client::connect("127.0.0.1:6379").await?;
    let channel = "numbers";
    client.publish(channel, "1".into()).await?;
    client.publish(channel, "22".into()).await?;
    client.publish(channel, "333".into()).await?;
    client.publish(channel, "4444".into()).await?;
    client.publish(channel, "55555".into()).await?;
    client.publish(channel, "666666".into()).await?;
    client.publish(channel, "7777777".into()).await?;
    client.publish(channel, "88888888".into()).await?;
    Ok(())
}

async fn subscribe(barrier: Arc<Barrier>) -> mini_redis::Result<()> {
    let client = client::connect("127.0.0.1:6379").await?;
    let subscribe = client.subscribe(vec!["numbers".into()]).await?;
    // 将订阅器转为消息流
    let messages = subscribe
        .into_stream()
        .filter(|msg| match msg {
            Ok(msg) if msg.content.len() > 3 => true,
            _ => false,
        })
        .map(|msg| msg.unwrap().content)
        .take(3);
    barrier.wait().await;
    // 将流固定到栈上
    tokio::pin!(messages);
    while let Some(msg) = messages.next().await {
        println!("收到消息: {:?}", msg);
    }
    Ok(())
}

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            Poll::Ready(())
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

struct Interval {
    rem: usize,
    delay: Delay,
}

impl Interval {
    fn new() -> Self {
        Self {
            rem: 3,
            delay: Delay {
                when: Instant::now(),
            },
        }
    }
}

impl Stream for Interval {
    type Item = ();
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.rem == 0 {
            return Poll::Ready(None);
        }

        match Pin::new(&mut self).poll_next(cx) {
            Poll::Ready(_) => {
                let when = self.delay.when + Duration::from_millis(10);
                self.delay = Delay { when };
                self.rem -= 1;
                Poll::Ready(Some(()))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
