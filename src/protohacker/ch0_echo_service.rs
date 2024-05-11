use std::error::Error;
use std::net::SocketAddr;
use std::time;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tracing::{error, info};

pub async fn server_run(port: u32) {
    let addr = format!("127.0.0.1:{}", port);

    let socket = TcpListener::bind(&addr).await.unwrap();
    info!("Listening on: {}", addr);

    while let Ok((stream, peer)) = socket.accept().await {
        info!("Incoming connection from: {}", peer);
        tokio::spawn(async move {
            let () = handle_connection(stream, peer).await;
            // info!("thread starting {} starting", peer);

            // let five_seconds = time::Duration::from_secs(5);
            // let begin = time::Instant::now();
            // let _ = tokio::time::sleep(five_seconds).await;

            // let end = begin.elapsed();
            // info!("thread {} finising {}", peer, end.as_secs_f32());
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
                    info!("EOF received");
                    break;
                }
                let buf_string = String::from_utf8_lossy(&buf);

                let data: Vec<String> = buf_string
                    .split(';')
                    .map(|x| x.to_string().replace('\n', ""))
                    .collect();

                info!("Received message: {:?}", data);

                // Echo back to the client
                let response = format!("{:?}", data);
                let _ = writer.write_all(response.as_bytes()).await;
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
    info!("stream starting");
    stream.write_all(b"hello world").await?;

    info!("stream finished");
    Ok(())
}
