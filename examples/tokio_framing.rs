use bytes::{Buf, Bytes, BytesMut};
use mini_redis::frame::Error::Incomplete;
use mini_redis::{Frame, Result};
use std::io::Cursor;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    // 建立TCP连接
    let stream = TcpStream::connect("127.0.0.1:6379").await.unwrap();
    let mut conn = Connection::new(stream);

    // 循环读取客户端指令
    while let Some(frame) = conn.read_frame().await.unwrap() {
        println!("收到客户端帧: {:?}", frame);
        // 回复 OK
        conn.write_frame(&Frame::Simple("OK".to_string()))
            .await
            .unwrap();
    }
}

/// TCP 连接封装
/// 内置缓冲读写，处理 RESP 帧粘包、分包逻辑
struct Connection {
    /// 带缓冲写流，减少 syscall
    stream: BufWriter<TcpStream>,
    /// 读缓冲区，存储未解析的 RESP 字节流
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream: BufWriter::new(stream),
            buffer: BytesMut::with_capacity(4096),
        }
    }

    /// 读取一个完整 RESP 帧
    /// - Ok(Some(frame))：成功读到完整帧
    /// - Ok(None)：对端正常关闭连接，无剩余数据
    /// - Err：协议错误 / IO 错误 / 连接异常关闭
    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            // 先尝试解析已有缓冲
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            // 缓冲不足，从 TCP 读取更多数据到 buffer
            let n = self.stream.read_buf(&mut self.buffer).await?;
            if n == 0 {
                // 读到0字节 = 对端关闭写端
                return if self.buffer.is_empty() {
                    Ok(None)
                } else {
                    // 还有残留未解析数据，异常断开
                    Err("连接提前关闭，存在不完整协议帧".into())
                };
            }
        }
    }

    /// 将 RESP Frame 写入缓冲并 flush 发送
    pub async fn write_frame(&mut self, frame: &Frame) -> io::Result<()> {
        match frame {
            Frame::Simple(val) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Error(val) => {
                self.stream.write_u8(b'-').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Integer(val) => {
                self.stream.write_u8(b':').await?;
                self.write_decimal(*val).await?;
            }
            Frame::Null => {
                self.stream.write_all(b"$-1\r\n").await?;
            }
            Frame::Bulk(val) => {
                let len = val.len() as u64;
                self.stream.write_u8(b'$').await?;
                self.write_decimal(len).await?;
                self.stream.write_all(val).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Array(_val) => unimplemented!("数组帧暂未实现"),
        }

        // 强制刷新缓冲区，数据立即发送
        self.stream.flush().await?;
        Ok(())
    }

    /// 解析缓冲区，返回完整帧（无IO，纯内存操作）
    fn parse_frame(&mut self) -> Result<Option<Frame>> {
        let mut buf = Cursor::new(&self.buffer[..]);

        // Frame::check 校验当前缓冲能否构成完整帧
        match Frame::check(&mut buf) {
            Ok(_) => {
                // check 后游标位置 = 完整帧占用字节数
                let frame_len = buf.position() as usize;

                // 重置游标到开头，正式解析帧
                buf.set_position(0);
                let frame = Frame::parse(&mut buf)?;

                // 把已解析字节从 buffer 移除，剩余数据留到下次解析
                self.buffer.advance(frame_len);

                Ok(Some(frame))
            }
            // 数据不够，无法组成完整帧，等待下次read
            Err(Incomplete) => Ok(None),
            // 协议格式非法，直接抛错断开连接
            Err(e) => Err(e.into()),
        }
    }

    /// 辅助方法：写入十进制数字 + \r\n（用于Integer、Bulk长度）
    async fn write_decimal(&mut self, mut val: u64) -> io::Result<()> {
        use std::fmt::Write;
        let mut buf = String::new();
        write!(&mut buf, "{}", val).unwrap();
        self.stream.write_all(buf.as_bytes()).await?;
        self.stream.write_all(b"\r\n").await?;
        Ok(())
    }
}
