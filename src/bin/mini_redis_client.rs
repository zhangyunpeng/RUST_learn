use mini_redis::{Result, client};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("key1", "val1".into()).await?;
    client.set("key2", "val2".into()).await?;
    let result = client.get("key1").await?;
    println!("从服务端读取到的值；result={:?}", result);
    let result = client.get("key2").await?;
    println!("从服务端读取到的值；result={:?}", result);
    let result = client.get("key3").await?;
    println!("从服务端读取到的值；result={:?}", result);
    Ok(())
}
