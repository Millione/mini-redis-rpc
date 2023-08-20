use std::net::SocketAddr;

use clap::{Parser, Subcommand};
use mini_redis_rpc::{
    gen::volo_gen::redis::{RedisServiceClientBuilder, SetReq},
    DEFAULT_PORT,
};
use pilota::FastStr;
use volo_thrift::ResponseError;

#[derive(Parser, Debug)]
#[clap(
    name = "mini-redis-rpc-cli",
    version,
    author,
    about = "Issue Redis commands"
)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    #[clap(name = "hostname", long, default_value = "127.0.0.1")]
    host: String,

    #[clap(long, default_value_t = DEFAULT_PORT)]
    port: u16,
}

#[derive(Subcommand, Debug)]
enum Command {
    Get {
        key: FastStr,
    },
    Set {
        key: FastStr,

        value: FastStr,

        #[arg(value_parser = clap::value_parser!(i64).range(1..))]
        expires: Option<i64>,
    },
    Del {
        key: Vec<FastStr>,
    },
    Ping {
        msg: Option<FastStr>,
    },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), ResponseError<std::convert::Infallible>> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn");
    }

    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    let addr: SocketAddr = format!("{}:{}", cli.host, cli.port).parse().unwrap();

    let client = RedisServiceClientBuilder::new("redis")
        .address(addr)
        .build();

    // Process the requested command
    match cli.command {
        Command::Get { key } => {
            if let Some(value) = client.get(key).await?.value {
                println!("{}", value);
            } else {
                println!("(nil)");
            }
        }
        Command::Set {
            key,
            value,
            expires,
        } => {
            client
                .set(SetReq {
                    key,
                    value,
                    expires,
                })
                .await?;
            println!("OK");
        }
        Command::Del { key } => println!("(integer) {}", client.del(key).await?),

        Command::Ping { msg } => {
            client.ping().await?;
            match msg {
                Some(msg) => println!("{}", msg),
                None => println!("PONG"),
            }
        }
    }

    Ok(())
}
