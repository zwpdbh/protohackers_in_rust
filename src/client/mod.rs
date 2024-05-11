use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use tracing::info;

use crate::ClientVersion;

pub async fn client_run(port: u32, v: ClientVersion) -> Result<(), Box<dyn Error>> {
    match v {
        ClientVersion::V1 => client_v1(port).await,
    }
}

pub async fn client_v1(port: u32) -> Result<(), Box<dyn Error>> {
    match TcpStream::connect(format!("127.0.0.1:{}", port)) {
        Ok(mut stream) => {
            println!("stream starting");
            let _ = stream.write_all(b"hello world");
            println!("stream finished");

            info!("Sent Hello, awaiting reply...");

            let mut buffer = Vec::new();
            match stream.read_to_end(&mut buffer) {
                Ok(_) => {
                    let text = from_utf8(&buffer).unwrap();
                    info!("reply: {}", text);
                }
                Err(e) => {
                    info!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            info!("Failed to connect: {}", e);
        }
    }
    info!("Terminated.");
    Ok(())
}
