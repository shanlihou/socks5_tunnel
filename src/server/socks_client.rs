use tokio::net::TcpSocket;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use std::net::SocketAddrV4;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

use crate::common::config::CONFIG;
use crate::types::alias::CommonRet;

pub async fn connect() -> CommonRet<(OwnedReadHalf, OwnedWriteHalf)> {
    let addr = CONFIG.socks_host.parse()?;
    let socket = TcpSocket::new_v4()?;
    let mut stream = socket.connect(addr).await?;

    let pack = vec![0x05, 0x01, 0x00];
    stream.write(&pack).await?;
    let buf = &mut [0; 2];
    stream.read(buf).await?;

    let dst_addr: SocketAddrV4 = CONFIG.remote_host.parse()?;
    let mut pack = vec![0x05, 0x01, 0x00, 0x01];
    pack.extend_from_slice(&dst_addr.ip().octets());
    pack.extend_from_slice(&dst_addr.port().to_be_bytes());

    stream.write(&pack).await?;
    let buf = &mut [0; 10];
    stream.read(buf).await?;

    Ok(stream.into_split())
}
