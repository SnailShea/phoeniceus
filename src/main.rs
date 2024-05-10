use clap::Parser;
use conf::{Args, TimeServerConfig};
use srv::TimeServerSpawner;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    let path = args.config;
    let config = TimeServerConfig::new(path);
    TimeServerSpawner::spawn(&config.mode, &config.bind).await;
}

pub mod conf;
pub mod srv;