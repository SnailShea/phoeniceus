use clap::Parser;
use std::fs::read_to_string;
use std::net::IpAddr;
use std::process::exit;
use toml::{from_str, Table};
use tracing::error;

const VALID_MODES: [&str; 3] = ["TCP", "UDP", "BOTH"];

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(
        short = 'c',
        long = "config",
        help = "Path to config file for phoeniceus daemon"
    )]
    pub config: String,
}

pub struct TimeServerConfig {
    pub mode: String,
    pub bind: String,
}

impl TimeServerConfig {
    pub fn new(config_path: String) -> TimeServerConfig {
        let _content = read_to_string(&config_path);
        if _content.is_err() {
            error!("Unable to load config file at '{config_path}' for reading");
            exit(1)
        } else {
            let content =_content.unwrap();
            let config: Table = from_str(content.as_str()).expect("Unable to read contents of file, check syntax.");
            let _mode = config
                .get("mode")
                .expect("Missing required key 'mode'")
                .as_str();
            if _mode.is_none() {
                error!("Value of mode must be a string");
                exit(1)
            }
            let _host = config
                .get("host")
                .expect("Missing required key 'host'")
                .as_str();
            if _host.is_none() {
                error!("Value of host must be a string");
                exit(1);
            }
            let _port = config
                .get("port")
                .expect("Missing required key 'port'")
                .as_integer();
            if _port.is_none() {
                error!("Value of port must be integer");
                exit(1);
            }
            let mode = _mode.unwrap().to_uppercase();
            let host_string = _host.unwrap().to_string();
            let host = host_string.parse::<IpAddr>();
            let port = _port.unwrap();
            if host.is_err() {
                error!("Refusing to bind to invalid host '{host_string}'");
                exit(1)
            }
            if !VALID_MODES.contains(&mode.as_str()) {
                let given_mode = _mode.unwrap();
                error!("Invalid operating mode '{given_mode}'");
                exit(1)
            }
            let bind = format!("{}:{}", host_string, port);

            TimeServerConfig { mode, bind }
        }
    }
}