mod connection;
mod server;

use crate::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new().await?;
    server.handle_request().await;
    Ok(())
}
