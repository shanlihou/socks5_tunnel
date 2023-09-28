use socks5_tunnel::server::tunnel_server;
use socks5_tunnel::common::log::AppLog;

async fn _main() {
    _ = tunnel_server::run().await;
}

fn main() {
    AppLog::trivial_conf(1);
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(_main());
}

