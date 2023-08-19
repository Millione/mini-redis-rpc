use std::net::SocketAddr;

use clap::Parser;
use mini_redis_rpc::{gen::volo_gen::redis::RedisServiceServer, Server, DEFAULT_PORT};

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn");
    }

    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let port = cli.port.unwrap_or(DEFAULT_PORT);

    let addr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();
    let addr = volo::net::Address::from(addr);

    RedisServiceServer::new(Server::new())
        .run(addr)
        .await
        .unwrap()
}

#[derive(Parser, Debug)]
#[clap(
    name = "mini-redis-rpc-server",
    version,
    author,
    about = "A Redis server"
)]
struct Cli {
    #[clap(long)]
    port: Option<u16>,
}
