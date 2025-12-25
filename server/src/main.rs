mod connection;
mod appserver;
mod ws;
mod upload;

use crate::appserver::AppServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = AppServer::new().await?;
    server.handle_request().await;
    Ok(())
}
