use std::error::Error;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::TcpStream;
use tracing::info;

#[allow(unused)]
pub async fn client_run_v1(port: u32) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await?;
    info!("stream starting");
    stream.write_all(b"hello world").await?;
    info!("stream finished");

    Ok(())
}

#[allow(unused)]
pub async fn client_run_v2(port: u32) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await?;
    info!("stream starting");
    stream.write_all(b"hello world").await?;
    info!("stream finished");

    // Flush the stream to ensure the message is sent immediately
    let _ = stream.flush();

    // Read the response from the server
    let mut reader = BufReader::new(&mut stream);
    let mut response = String::new();
    let _ = reader.read_line(&mut response);

    // Print the response from the server
    info!("Response from server: {}", response);

    Ok(())
}
