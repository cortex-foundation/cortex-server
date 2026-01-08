use tokio::net::TcpListener;
use anyhow::Result;
use crate::config::Config;
use crate::client;

pub async fn run() -> Result<()> {
    let config = Config::new();
    let listener = TcpListener::bind(&config.bind_addr).await?;
    
    println!("Cortex Proxy listening on {}", config.bind_addr);
    println!("Forwarding to {}", config.target_addr);

    loop {
        let (socket, _) = listener.accept().await?;
        let target = config.target_addr.clone();
        
        tokio::spawn(async move {
            if let Err(e) = client::handle_connection(socket, target).await {
                eprintln!("Connection error: {}", e);
            }
        });
    }
}
