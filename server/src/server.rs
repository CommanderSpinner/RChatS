use std::fs;
use toml::Value;
use std::net::TcpListener;

use crate::connection::Connection;

pub struct Server {
    conn: Connection,
}

impl Server {
    pub async fn new() -> Result<Self, sqlx::Error> {

        println!("Starting server");

        let conn_string = Self::read_conn_string();
        let conn = Connection::new(conn_string).await?;

        Self::start_tcp_listener();

        Ok(Self { conn })
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

    fn start_tcp_listener() {
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

        let listener = TcpListener::bind(&addr)
            .expect("Failed to bind TCP listener");


        println!("Server listening on {}", addr);
        
    }

    pub async fn handle_request(&self) {
        
    } 
}
