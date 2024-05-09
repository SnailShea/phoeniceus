use std::process::exit;
use time::{
    macros::{date, time},
    OffsetDateTime,
    UtcOffset
};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, UdpSocket}
};
use tracing::{error, info};

fn rfc868_now() -> i32 {
    let offset = UtcOffset::UTC;
    let base_time = OffsetDateTime::new_in_offset(date!(1900 - 01 - 01), time!(0:00), offset);
    let now = OffsetDateTime::now_utc();
    let seconds = (now - base_time).whole_seconds() as i32;
    seconds
}


pub struct UDPTimeServer(UdpSocket);
impl UDPTimeServer {
    pub async fn new(bind_host: &str) -> UDPTimeServer {
        let listener = UdpSocket::bind(&bind_host).await;
        if listener.is_err() {
            error!("Unable to bind to UDP socket at {bind_host}");
            exit(1);
        } else {
            UDPTimeServer { 0: listener.unwrap() }    
        }
    }

    pub async fn listen(&self) {
        loop {
            let mut buf: [u8; 0] = [];
            let received = self.0.recv_from(&mut buf).await;
            if received.is_ok() {
                let (_, addr) = received.unwrap();
                info!("Received UDP connection from {addr}");
                let seconds = rfc868_now();
                let response = self.0.send_to(&seconds.to_be_bytes(), addr).await;
                if let Err(err) = response {
                    error!("Unable to write to UDP socket for {addr}: {err}");
                } else {
                    info!("Sent {seconds} as bytes to {addr}")
                }
            }
        }
    }
}

pub struct TCPTimeServer(TcpListener);
impl TCPTimeServer {
    pub async fn new(bind_host: &str) -> TCPTimeServer {
        let listener = TcpListener::bind(bind_host).await;
        if listener.is_err() {
            error!("Unable to bind to TCP socket at {bind_host}");
            exit(1);
        } else {
            TCPTimeServer { 0: listener.unwrap() }
        }
    }

    pub async fn listen(&self) {
        loop {
            let client = self.0.accept().await;
            if client.is_ok() {
                let (mut socket, addr) = client.unwrap();
                info!("Received TCP connection from {addr}");
                let seconds = rfc868_now();
                let response = socket.write_all(&seconds.to_be_bytes()).await;
                if let Err(err) =  response {
                    error!("Unable to write to TCP socket for {addr}: {err}")
                } else {
                    info!("Sent {seconds:?} as bytes to {addr}")
                }
            }
        }
    }
}