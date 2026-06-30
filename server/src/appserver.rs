use std::fs;
use toml::Value;

use crate::{apphttp::ws::ws_handler, connection::Connection, apphttp::upload::upload_file, apphttp::page::page};

use hyper::Server;

use axum::{
    Router,
    routing::{get, post, get_service},
};
use std::{net::SocketAddr, sync::Arc};
use tower_http::services::ServeDir;


pub struct AppServer {
    conn: Connection,
}

impl AppServer {
    pub async fn new() -> Result<Self, anyhow::Error> {

        println!("Starting server");

        common::debug_println!("process id: {}", std::process::id());

        let conn_string = Self::read_conn_string();
        let conn = Connection::new(conn_string).await?;

        Ok(Self { conn})
    }

    fn read_conn_string() -> String {
        let config_file = fs::read_to_string("data/config.toml")
            .expect("Failed to read DB!");

        let config: toml::Value = toml::from_str(&config_file)
            .expect("Failed to parse TOML config");

        // Convert the "database" section safely into a Table
        let db_config = config
            .get("database")
            .and_then(|v| v.as_table())
            .expect("Missing [database] section in config");

        // Extract individual fields cleanly from the table
        let user = db_config.get("user").and_then(|v| v.as_str()).unwrap_or("admin");
        let password = db_config.get("password").and_then(|v| v.as_str()).unwrap_or("");
        let host = db_config.get("host").and_then(|v| v.as_str()).unwrap_or("localhost");
        
        // TOML numbers are i64, so we map it to an integer
        let port = db_config.get("port").and_then(|v| v.as_integer()).unwrap_or(5432);
        let dbname = db_config.get("dbname").and_then(|v| v.as_str()).unwrap_or("");

        // Build the PostgreSQL connection URL dynamically
        let connection_string = format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, dbname);

        common::debug_println!("config for db:\n {}", config_file);
        common::debug_println!("conn string: {}", connection_string);
        
        connection_string
    }

    fn read_port() -> u16 {
        let config_file = std::fs::read_to_string("data/config.toml")
            .expect("Failed to read rchats.toml");

        let value: toml::Value = toml::from_str(&config_file)
            .expect("Failed to parse TOML");

        value["rchats"]["port"]
            .as_integer()
            .expect("Port must be an integer") as u16
    }

    pub async fn handle_request(&self) -> anyhow::Result<()> {
        // Wrap DB connection in Arc for shared state
        let state = Arc::new(self.conn.clone());
            
        let app = Router::new()
            .route("/ws", get(ws_handler))
            .route("/upload/:filetype", post(upload_file)) // dynamic filetype
            .route("/", get(page))
            .route("/", post(page))
            .with_state(state);

        // Read port from config
        let port = Self::read_port();
        let addr = SocketAddr::from(([0, 0, 0, 0], port));

        println!("Server listening on {}", addr);

        Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}

impl Drop for AppServer {
    fn drop(&mut self) {
        println!("stopping server");
    }
}