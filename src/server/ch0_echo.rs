use futures::StreamExt;
use std::net::SocketAddr;
use std::time;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio_util::codec::{BytesCodec, Framed};
use tracing::info;

use crate::ServerVersion;

pub async fn server_run(port: u32, v: ServerVersion) {
    let addr = format!("127.0.0.1:{}", port);

    let listener = TcpListener::bind(&addr).await.unwrap();
    info!("Listening on: {}", addr);

    while let Ok((stream, peer)) = listener.accept().await {
        info!("Incoming connection from: {}", peer);

        tokio::spawn(async move {
            match v {
                ServerVersion::V1 => {
                    let () = handle_connection_v1(stream, peer).await;
                }
                ServerVersion::V2 => {
                    let () = handle_connection_v2(stream, peer).await;
                }
            }
        });
    }
}

async fn handle_connection_v1(mut stream: TcpStream, peer: SocketAddr) {
    let begin = time::Instant::now();
    info!("thread starting {} starting", peer);

    // splitting stream into reader and writer, and then creating our buffer reader form the reader
    let (reader, mut writer) = stream.split();
    let mut buf_reader = BufReader::new(reader);
    let mut buf = vec![];

    loop {
        // continuously read one line at a time
        match buf_reader.read_until(b'\n', &mut buf).await {
            Ok(n) => {
                // telling the reader to stop reading once it hits the EOF condition.
                if n == 0 {
                    info!("EOF received");
                    break;
                }
                let buf_string = String::from_utf8_lossy(&buf);
                info!("Received message: {:?}", buf_string);

                // Echo back to the client
                let _ = writer.write_all(buf_string.as_bytes()).await;
                let _ = writer.flush().await;

                buf.clear();
            }
            Err(e) => {
                info!("Error receiving message: {}", e)
            }
        }
    }

    let end = begin.elapsed();
    info!("thread {} finising {}", peer, end.as_secs_f32());
}

async fn handle_connection_v2(stream: TcpStream, peer: SocketAddr) {
    let begin = time::Instant::now();
    info!("thread starting {} starting", peer);

    let (write, read) = Framed::new(stream, BytesCodec::new()).split();
    let _ = read.forward(write).await;

    let end = begin.elapsed();
    info!("thread {} finising {}", peer, end.as_secs_f32());
}
