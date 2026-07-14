use tokio::io::{self, AsyncReadExt, AsyncWriteExt, Result};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let _ = demo_copy().await;
}

async fn demo_copy() -> Result<()> {
    let socket = TcpStream::connect("127.0.0.1:8080").await?;
    let (mut rd, mut wr) = io::split(socket);

    tokio::spawn(async move {
        wr.write_all(b"Hello\n\r").await?;
        wr.write_all(b"World\n\r").await?;
        wr.shutdown().await?;
        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 128];

    loop {
        let n = rd.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        println!("收到数据{:?}", String::from_utf8_lossy(&buf[..n]));
    }

    Ok(())
}
