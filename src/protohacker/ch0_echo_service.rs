use std::error::Error;
use std::time;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

pub async fn server_run(port: u32) {
    let addr = format!("127.0.0.1:{}", port);

    let socket = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on: {}", addr);

    while let Ok((mut _stream, peer)) = socket.accept().await {
        println!("Incoming connection from: {}", peer);
        tokio::spawn(async move {
            println!("thread starting {} starting", peer);

            let five_seconds = time::Duration::from_secs(5);
            let begin = time::Instant::now();
            let _ = tokio::time::sleep(five_seconds).await;

            let end = begin.elapsed();
            println!("thread {} finising {}", peer, end.as_secs_f32());
        });
    }
}

#[allow(unused)]
pub async fn client_run(port: u32) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await?;
    println!("stream starting");
    stream.write_all(b"hello world").await?;
    println!("stream finished");

    Ok(())
}
