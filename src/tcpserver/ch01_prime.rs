use crate::ServerVersion;
use std::net::SocketAddr;
use std::time;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tracing::{info, span, Level};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    method: String,
    number: serde_json::Number,
}

#[derive(Serialize)]
struct Response {
    method: String,
    prime: bool,
}

#[derive(Serialize)]
struct MalformedResponse {
    method: String,
    error: String,
}

pub async fn server_run(port: u32, v: ServerVersion) {
    let addr = format!("127.0.0.1:{}", port);

    let listener = TcpListener::bind(&addr).await.unwrap();
    info!("Listening on: {}", addr);

    while let Ok((stream, peer)) = listener.accept().await {
        info!("Incoming connection from: {}", peer);

        tokio::spawn(async move {
            match v {
                ServerVersion::EchoV1 => {
                    let () = handle_connection(stream, peer).await;
                }
                ServerVersion::EchoV2 => {
                    todo!("not implemented");
                }
            }
        });
    }
}

async fn handle_connection(mut stream: TcpStream, peer: SocketAddr) {
    let span = span!(Level::INFO, "handle_connection");
    let _enter = span.enter();

    let begin = time::Instant::now();
    info!("thread starting {} starting", peer);

    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut buffer = String::new();

    loop {
        buffer.clear();

        match reader.read_line(&mut buffer).await {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    info!("EOF received");
                    break;
                }

                let response: String = match serde_json::from_str::<Request>(&buffer) {
                    Ok(request) if request.method == "isPrime" => {
                        todo!("valid json message");
                    }
                    _ => {
                        let malformed =
                            serde_json::json!({"method": "isPrime", "error": "malformed request"});
                        serde_json::to_string(&malformed).unwrap() + "\n"
                    }
                };
                if writer.write_all(response.as_bytes()).await.is_err() {
                    break;
                }

                if response.contains("error") {
                    break;
                }
            }
            Err(_) => break,
        }
    }

    let end = begin.elapsed();
    info!("thread {} finising {}", peer, end.as_secs_f32());
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while (i as f64) <= (n as f64).sqrt() {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}
