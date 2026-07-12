use tokio::sync::mpsc;
/*
mpsc：多生产者单消费者通道，可发送多条消息；
oneshot：单生产者单消费者通道，仅能传递一条消息；
broadcast：多生产者多消费者，每条消息所有接收者都能收到；
watch：多生产者多消费者，仅保留最新一条消息，无历史缓存。
*/
#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(10);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("first").await.unwrap();
    });

    tokio::spawn(async move {
        tx2.send("second").await.unwrap();
    });

    while let Some(msg) = rx.recv().await {
        println!("get message = {}", msg);
    }
}
