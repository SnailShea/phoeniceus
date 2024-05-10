# Phoeniceus
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](code_of_conduct.md) ![GitHub License](https://img.shields.io/github/license/EastonVelocity/phoeniceus) ![Crates.io Version](https://img.shields.io/crates/v/phoeniceus) 

A modern implementation of RFC868 inspired by [timed-rs](https://github.com/yanorei32/timed-rs) and the death of xinetd in RHEL (and alike) systems. 
This implementation will listen for TCP, UDP, or a mix of both connection types.

The name of the project is based on the scientific name of the Red-winged Blackbird. :)

# Configuration
The application requires a TOML-based config file including the following options:
```
mode (string): One of "tcp", "udp" or "both"
host (string): address to bind to e.g., "127.0.0.1", "::"
port (integer): port to bind to, normally 37
```

### Examples
Listens on all available v4 and v6 addresses for TCP and UDP connections via port 37
```toml
mode = "both"
host = "::"
port = 37
```

Listens on v4 localhost for TCP connections via port 1037
```toml
mode = "tcp"
host = "127.0.0.1"
port = 1037
```

# Usage
```
Usage: phoeniceus --config <CONFIG>

Options:
  -c, --config <CONFIG>  Path to config file for phoeniceus daemon
  -h, --help             Print help
```

### Example
```
[tramage@tramage-desktop phoeniceus]$ cat tod.toml 
mode = "both"
host = "::"
port = 37

[tramage@tramage-desktop phoeniceus]$ phoeniceus -c ./tod.toml 
2024-05-09T17:05:07.507007Z  INFO phoeniceus: Listening for TCP and UDP connections on :::37

[tramage@tramage-desktop phoeniceus]$ rdate localhost -p
rdate: [localhost]      Thu May  9 13:07:19 2024

# and in the application logs...
2024-05-09T17:07:19.347900Z  INFO phoeniceus::srv: Received TCP connection from [::1]:48300
2024-05-09T17:07:19.347947Z  INFO phoeniceus::srv: Sent -370704057 as bytes to [::1]:48300
```
