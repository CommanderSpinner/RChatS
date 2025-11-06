mod connection;
mod server;

use crate::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _s = Server::new().await?;
    Ok(())
}
