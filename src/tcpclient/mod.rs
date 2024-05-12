use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
use tracing::info;

use crate::ClientVersion;

pub async fn client_run(port: u32, v: ClientVersion) -> Result<(), Box<dyn Error>> {
    match v {
        ClientVersion::V1 => client_v1(port).await,
        ClientVersion::V2 => client_v2(port).await,
    }
}

pub async fn client_v1(port: u32) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(format!("localhost:{}", port)).unwrap();
    info!("connecting to port: {}", port);
    // Since the server is expected to read_until `\n`, we must add `\n` in the end.
    // Otherwise, the server will block for ever to wait until the client close.
    stream.write("Hello\n".as_bytes()).unwrap();
    stream.flush().unwrap();

    let mut buffer = [0; 6];
    stream.read(&mut buffer).unwrap();

    info!(
        "got response from server: {:?}",
        str::from_utf8(&buffer).unwrap()
    );
    Ok(())
}

pub async fn client_v2(port: u32) -> Result<(), Box<dyn Error>> {
    match TcpStream::connect(format!("127.0.0.1:{}", port)) {
        Ok(mut stream) => {
            info!("stream starting");
            let _ = stream.write_all("Hello World\n".as_bytes()).unwrap();
            let _ = stream.flush();

            info!("send message finished, awaiting reply...");

            let mut buffer = Vec::new();
            match stream.read_to_end(&mut buffer) {
                Ok(_) => {
                    let text = str::from_utf8(&buffer).unwrap();
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
