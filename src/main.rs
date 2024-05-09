use clap::Parser;
use conf::TimeConfig;
use srv::{TCPTimeServer, UDPTimeServer};
use tokio::join;
use tracing::info;

#[derive(Debug, Parser)]
struct Args {
    #[arg(
        short = 'c',
        long = "config",
        help = "Path to config file for phoeniceus daemon"
    )]
    config: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    let path = args.config;
    let config = TimeConfig::new(path);
    let mode = config.mode;
    let bind = config.bind;
    if mode == "UDP".to_string() {
        let listener = UDPTimeServer::new(&bind).await;
        info!("Listening for UDP connections on {bind}");
        let _ = listener.listen().await;

    } else if mode == "TCP" {
        let listener = TCPTimeServer::new(&bind).await;
        info!("Listening for TCP connections on {bind}");
        let _ = listener.listen().await;
    } else {
        let tcp_listener = TCPTimeServer::new(&bind).await;
        let udp_listener = UDPTimeServer::new(&bind).await;
        info!("Listening for TCP and UDP connections on {bind}");
        let _ = join!(
            tcp_listener.listen(),
            udp_listener.listen()
        );
    }
}

pub mod conf;
pub mod srv;