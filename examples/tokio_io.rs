use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let _ = demo_read().await;
    let _ = demo_read_to_end().await;
    let _ = demo_write().await;
    let _ = demo_write_all().await.unwrap();
    let _ = demo_copy().await;
}

async fn demo_copy() -> io::Result<()> {
    let mut reader: &[u8] = b"hello world";
    let mut file = File::create("./files/demo_copy.txt").await?;
    io::copy(&mut reader, &mut file).await?;
    Ok(())
}

async fn demo_read() -> io::Result<()> {
    let mut f = File::open("./files/demo1.txt").await?;
    let mut buffer = [0; 10];
    let n = f.read(&mut buffer[..]).await?;
    println!("read {} bytes", n);
    Ok(())
}

async fn demo_read_to_end() -> io::Result<()> {
    let mut f = File::open("./files/demo1.txt").await?;
    let mut buffer = vec![];
    f.read_to_end(&mut buffer).await?;
    println!("read {} bytes", buffer.len());
    Ok(())
}

async fn demo_write() -> io::Result<()> {
    let mut f = File::create("./files/demo_write.txt").await?;
    let n = f.write(b"Hello, World!").await?;
    println!("wrote {} bytes", n);
    Ok(())
}

async fn demo_write_all() -> io::Result<()> {
    let mut f = File::create("./files/demo_write.txt").await?;
    f.write_all(b"Hello, World!").await?;
    Ok(())
}
