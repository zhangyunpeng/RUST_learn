use bytes::Bytes;
use mini_redis::{Result, client};
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp_tx: Responser<Option<Bytes>>,
    },
    Set {
        key: String,
        value: Bytes,
        resp_tx: Responser<()>,
    },
}

type Responser<T> = oneshot::Sender<Result<T>>;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(10);
    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp_tx } => {
                    let resp = client.get(&key).await;
                    let _ = resp_tx.send(resp);
                }
                Command::Set {
                    key,
                    value,
                    resp_tx,
                } => {
                    client.set(key.as_str(), value).await.unwrap();
                    let _ = resp_tx.send(Ok(()));
                }
            }
        }
    });
    let tx2 = tx.clone();
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "key1".into(),
            value: "value1".into(),
            resp_tx,
        };
        tx.send(cmd).await.unwrap();
        let resp = resp_rx.await.unwrap();
        println!("set response: {:?}", resp);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "key1".into(),
            resp_tx,
        };
        tx2.send(cmd).await.unwrap();
        let resp = resp_rx.await.unwrap();
        println!("get response: {:?}", resp);
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
