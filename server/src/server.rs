use std::fs;
use toml::Value;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::connection::Connection;

pub struct Server {
    conn: Connection,
    listener: TcpListener,
}

impl Server {
    pub async fn new() -> Result<Self, anyhow::Error> {

        println!("Starting server");

        common::debug_println!("process id: {}", std::process::id());

        let conn_string = Self::read_conn_string();
        let conn = Connection::new(conn_string).await?;

        let listener: TcpListener = Self::start_tcp_listener().await?;

        Ok(Self { conn, listener})
    }

    fn read_conn_string() -> String {

        let config_file = fs::read_to_string("files/server_data/config/db.toml")
            .expect("Failed to read DB!");

        let conn_string_value: Value = toml::from_str(&config_file)
            .expect("Failed to parse TOML config");

        let connection_string = conn_string_value["database"]["url"]
            .as_str()
            .unwrap();

        println!("config for db:\n {}", config_file);
        println!("conn string: {}", connection_string);
        
        connection_string.to_string()
    }

    async fn start_tcp_listener() -> anyhow::Result<TcpListener> {
        let config_file_rchats = fs::read_to_string("files/server_data/config/rchats.toml")
            .expect("Failed to read config file for rchats!");

        let config_string_value: Value = toml::from_str(&config_file_rchats)
            .expect("Failed to parse TOML config for rchats");

        // Get the port as u16 (or as string)
        let port = config_string_value["config"]["port"]
            .as_integer()
            .expect("Port must be an integer") as u16;

        // Build the address string
        let addr = format!("0.0.0.0:{}", port);

        // Bind TCP listener asynchronously
        let listener = TcpListener::bind(&addr).await
            .map_err(|e| anyhow::anyhow!("Failed to bind TCP listener: {}", e))?;


        println!("Server listening on {}", addr);

        Ok(listener)
    }

    pub async fn handle_request(&self) -> anyhow::Result<()> {
        loop {
        let (socket, addr) = self.listener.accept().await?;
        println!("Client connected: {}", addr);

        let mut conn = self.conn.clone();

        tokio::spawn(async move {
            if let Err(e) = Self::handle_client(socket, conn).await {
                eprintln!("client {} error: {:?}", addr, e);
            }
            });
        }
        
    }

    // needs behavior for text and files
    async fn handle_client( mut socket: TcpStream, mut db: Connection) -> anyhow::Result<()> {
        let mut buf = [0u8; 1024]; // buffe mgiht not be adequat

        loop {
            let n = socket.read(&mut buf).await?;
            if n == 0 {
                return Ok(()); // disconnected
            }

            let msg = String::from_utf8_lossy(&buf[..n]);
            
            common::debug_println!("Client says: {}", msg);

            socket.write_all(b"ok\n").await?;
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        println!("stopping server");
    }
}