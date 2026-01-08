use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use anyhow::{Result, Context};
use crate::protocol;

pub async fn forward_to_brain(target_addr: &str, msg: &str) -> Result<String> {
    let mut stream = TcpStream::connect(target_addr).await
        .context("Failed to connect to Brain (NestJS)")?;
    
    // Frame and send
    let framed = protocol::frame_message("ping", msg, "1");
    stream.write_all(framed.as_bytes()).await?;
    
    // Read response
    let mut buf = [0; 4096];
    let n = stream.read(&mut buf).await?;
    
    let raw_response = String::from_utf8_lossy(&buf[0..n]);
    Ok(protocol::parse_response(&raw_response))
}
