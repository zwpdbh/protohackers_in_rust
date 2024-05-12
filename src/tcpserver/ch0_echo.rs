use futures::StreamExt;
use std::net::SocketAddr;
use std::time;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio_util::codec::{BytesCodec, Framed};
use tracing::{info, span, Level};

use crate::ServerVersion;

pub async fn server_run(port: u32, v: ServerVersion) {
    let addr = format!("127.0.0.1:{}", port);

    let listener = TcpListener::bind(&addr).await.unwrap();
    info!("Listening on: {}", addr);

    while let Ok((stream, peer)) = listener.accept().await {
        info!("Incoming connection from: {}", peer);

        tokio::spawn(async move {
            match v {
                ServerVersion::EchoV1 => {
                    let () = handle_connection_v1(stream, peer).await;
                }
                ServerVersion::EchoV2 => {
                    let () = handle_connection_v2(stream, peer).await;
                }
            }
        });
    }
}

/// This handles the tcp frame by simply using '\n'.
async fn handle_connection_v1(mut stream: TcpStream, peer: SocketAddr) {
    let span = span!(Level::INFO, "handle_connection");
    let _enter = span.enter();

    let begin = time::Instant::now();
    info!("thread starting {} starting", peer);

    // splitting stream into reader and writer, and then creating our buffer reader form the reader
    let (reader, mut writer) = stream.split();
    let mut buf_reader = BufReader::new(reader);
    let mut buf = vec![];

    loop {
        // Continuously read one line at a time:
        // This means if the content sent from client doesn't contains `\n`, the code will block to wait.
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

/// This handles frame using existing library: tokio_util::codec::framed
async fn handle_connection_v2(stream: TcpStream, peer: SocketAddr) {
    let span = span!(Level::INFO, "handle_connection");
    let _enter = span.enter();

    let begin = time::Instant::now();
    info!("thread starting {} starting", peer);

    let (write, read) = Framed::new(stream, BytesCodec::new()).split();
    let _ = read.forward(write).await;

    let end = begin.elapsed();
    info!("thread {} finising {}", peer, end.as_secs_f32());
}
