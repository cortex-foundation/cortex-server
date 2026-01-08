mod config;
mod protocol;
mod backend;
mod client;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server::run().await
}
