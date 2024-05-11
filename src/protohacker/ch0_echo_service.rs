use std::error::Error;
use std::net::SocketAddr;
use std::time;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tracing::debug;
use tracing::{error, info, warn};

pub async fn server_run(port: u32) {
    let addr = format!("127.0.0.1:{}", port);

    let listener = TcpListener::bind(&addr).await.unwrap();
    info!("Listening on: {}", addr);

    while let Ok((stream, peer)) = listener.accept().await {
        info!("Incoming connection from: {}", peer);
        tokio::spawn(async move {
            let () = handle_connection(stream, peer).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream, peer: SocketAddr) {
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
                    warn!("EOF received");
                    break;
                }
                let buf_string = String::from_utf8_lossy(&buf);
                info!("Received message: {:?}", buf_string);

                // Echo back to the client
                let _ = writer.write_all(buf_string.as_bytes()).await;
                let _ = writer.flush().await;

                buf.clear()
            }
            Err(e) => {
                error!("Error receiving message: {}", e)
            }
        }
    }

    let end = begin.elapsed();
    info!("thread {} finising {}", peer, end.as_secs_f32());
}

pub async fn client_run(port: u32) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await?;
    let (reader, mut writer) = stream.split();

    info!("stream starting");

    let _ = writer.write_all(b"hello world").await?;
    let _ = writer.flush().await; // Never forget to do this

    let mut buf_reader = BufReader::new(reader);
    let mut buf = vec![];

    loop {
        debug!("why this doesn't work");
        match buf_reader.read_until(b'\n', &mut buf).await {
            Ok(n) => {
                // telling the reader to stop reading once it hits the EOF condition.
                if n == 0 {
                    info!("EOF received");
                    break;
                }
                let buf_string = String::from_utf8_lossy(&buf);

                let data: Vec<String> = buf_string
                    .split(';')
                    .map(|x| x.to_string().replace('\n', ""))
                    .collect();

                info!("Received message: {:?}", data);

                buf.clear();
            }
            Err(e) => {
                error!("Error receiving message: {}", e)
            }
        }
    }

    info!("stream finished");
    Ok(())
}
