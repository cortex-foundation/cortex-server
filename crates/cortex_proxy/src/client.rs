use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use anyhow::Result;
use crate::backend;

pub async fn handle_connection(mut socket: TcpStream, target_addr: String) -> Result<()> {
    let mut buf = [0; 1024];

    loop {
        let n = match socket.read(&mut buf).await {
            Ok(n) if n == 0 => return Ok(()),
            Ok(n) => n,
            Err(e) => return Err(e.into()),
        };

        let user_msg = String::from_utf8_lossy(&buf[0..n]).to_string();
        let clean_msg = user_msg.trim();

        if clean_msg.is_empty() {
            continue;
        }

        match backend::forward_to_brain(&target_addr, clean_msg).await {
            Ok(response) => {
                let reply = format!("Proxy Received: {}", response);
                socket.write_all(reply.as_bytes()).await?;
            }
            Err(e) => {
                eprintln!("Backend error: {}", e);
                let _ = socket.write_all(b"Error: Brain unavailable.\n").await;
            }
        }
    }
}
