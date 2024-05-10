use clap::Parser;
use conf::{Args, TimeConfig};
use srv::TimeServerSpawner;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    let path = args.config;
    let config = TimeConfig::new(path);
    let mode = config.mode;
    let bind = config.bind;
    let _ = TimeServerSpawner::spawn(&mode, &bind).await;
}

pub mod conf;
pub mod srv;