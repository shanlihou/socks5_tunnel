use tokio::net::TcpSocket;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use std::net::SocketAddrV4;
use tokio::io::{AsyncWriteExt, AsyncReadExt};


use crate::types::alias::CommonRet;

pub async fn connect() -> CommonRet<(OwnedReadHalf, OwnedWriteHalf)> {
    let addr = "192.168.16.134:7890".parse()?;
    let socket = TcpSocket::new_v4()?;
    let mut stream = socket.connect(addr).await?;

    let pack = vec![0x05, 0x01, 0x00];
    stream.write(&pack).await?;
    let buf = &mut [0; 2];
    stream.read(buf).await?;
    println!("buf: {:?}", buf);

    let dst_addr: SocketAddrV4 = "46.151.32.187:42014".parse()?;
    let mut pack = vec![0x05, 0x01, 0x00, 0x01];
    pack.extend_from_slice(&dst_addr.ip().octets());
    pack.extend_from_slice(&dst_addr.port().to_be_bytes());

    stream.write(&pack).await?;
    let buf = &mut [0; 10];
    stream.read(buf).await?;
    println!("buf: {:?}", buf);

    Ok(stream.into_split())
}
