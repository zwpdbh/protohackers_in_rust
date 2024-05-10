use std::error::Error;
use std::time;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tracing::info;

pub async fn server_run(port: u32) {
    let addr = format!("127.0.0.1:{}", port);

    let socket = TcpListener::bind(&addr).await.unwrap();
    info!("Listening on: {}", addr);

    while let Ok((mut _stream, peer)) = socket.accept().await {
        info!("Incoming connection from: {}", peer);
        tokio::spawn(async move {
            info!("thread starting {} starting", peer);

            let five_seconds = time::Duration::from_secs(5);
            let begin = time::Instant::now();
            let _ = tokio::time::sleep(five_seconds).await;

            let end = begin.elapsed();
            info!("thread {} finising {}", peer, end.as_secs_f32());
        });
    }
}

#[allow(unused)]
pub async fn client_run(port: u32) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await?;
    info!("stream starting");
    stream.write_all(b"hello world").await?;
    info!("stream finished");

    Ok(())
}
