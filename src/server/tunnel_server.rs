use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::io::{AsyncWriteExt, AsyncReadExt};

use crate::server::socks_client;
use crate::types::alias::CommonRet;
use crate::common::config::CONFIG;


async fn process_socks_read(
    mut writer: OwnedWriteHalf,
    mut s_reader: OwnedReadHalf,
    ) -> CommonRet<()> {
    
    let mut data = [0; 2048];
    loop {
        let n = s_reader.read(&mut data).await?;
        if n == 0 {
            break;
        }
        writer.write(&data[..n]).await?;
    }

    Ok(())
}
    


async fn process_connection(
    mut reader: OwnedReadHalf,
    writer: OwnedWriteHalf,
    ) -> CommonRet<()> {

    let (s_reader, mut s_writer) = socks_client::connect().await?;

    tokio::spawn(async move {
        if let Err(e) = process_socks_read(writer, s_reader).await {
            log::error!("process_socks_read error: {:?}", e);
        }
    });

    let mut data = [0; 2048];
    loop {
        let n = reader.read(&mut data).await?;
        if n == 0 {
            break;
        }
        s_writer.write(&data[..n]).await?;
    }

    Ok(())
}

pub async fn run() 
{
    let listener = tokio::net::TcpListener::bind(&CONFIG.local_host).await.unwrap();
    log::info!("game server start: success {}", CONFIG.local_host);

    loop {
        let (tcp_reader, tcp_writer) = if let Ok((socket, _)) = listener.accept().await {
            socket.into_split()
        } else {
            log::error!("accept error");
            continue;
        };

        let addr = if let Ok(addr) = tcp_reader.peer_addr() {
            addr
        } else {
            log::error!("get peer addr error");
            continue;
        };

        let ip = addr.ip();
        let port = addr.port();

        log::info!("client connected ip:{:?}, port:{:?}", ip, port);
        tokio::spawn(process_connection(
            tcp_reader,
            tcp_writer,
        ));
    }
}

