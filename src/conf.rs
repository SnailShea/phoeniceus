use std::fs::read_to_string;
use std::net::IpAddr;
use std::process::exit;
use toml::{from_str, Table};
use tracing::error;

const VALID_MODES: [&str; 3] = ["TCP", "UDP", "BOTH"];

pub struct TimeConfig {
    pub mode: String,
    pub bind: String,
}

impl TimeConfig {
    pub fn new(config_path: String) -> TimeConfig {
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
                .as_str()
                .unwrap();
            let _host = config
                .get("host")
                .expect("Missing required key 'host'")
                .as_str()
                .unwrap();
            let _port = config
                .get("port")
                .expect("Missing required key 'port'")
                .as_integer()
                .unwrap();
            let mode = _mode.to_uppercase();
            let port = _port;
            let host_string = _host.to_string();
            let host = host_string.parse::<IpAddr>();
            if host.is_err() {
                error!("Refusing to bind to invalid host '{host_string}'");
                exit(1)
            }
            if !VALID_MODES.contains(&mode.as_str()) {
                error!("Invalid operating mode '{_mode}'");
                exit(1)
            }
            let bind = format!("{}:{}", _host, port);

            TimeConfig { mode, bind }
        }
    }
}