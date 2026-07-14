use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    // let _ = demo1().await;
    // let _ = demo2().await;
    let _ = demo3().await;
}

#[allow(unused)]
async fn demo1() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        let (mut rd, mut wr) = io::split(socket);
        if io::copy(&mut rd, &mut wr).await.is_err() {
            eprintln!("failed to copy data from socket");
        }
    }
}

#[allow(unused)]
async fn demo2() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        let (mut rd, mut wr) = TcpStream::split(&mut socket);
        if io::copy(&mut rd, &mut wr).await.is_err() {
            eprintln!("failed to copy data from socket");
        }
    }
}

async fn demo3() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            match socket.read(&mut buf).await {
                Ok(0) => return,
                Ok(n) => {
                    if socket.write_all(&buf[..n]).await.is_err() {
                        eprintln!("failed to write data");
                        return;
                    }
                }
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
