use std::fs;
use toml::Value;

use crate::connection::Connection;

pub struct Server {
    conn: Connection,
}

impl Server {
    pub async fn new() -> Result<Self, sqlx::Error> {

        println!("Starting server");

        let conn_string = Self::read_conn_string();
        let conn = Connection::new(conn_string).await?;
        Ok(Self { conn })
    }

    fn read_conn_string() -> String {

        let config_file = fs::read_to_string("files/post_install/config/db.toml")
            .expect("Failed to read DB!");

        let conn_string_value: Value = toml::from_str(&config_file)
            .expect("Failed to parse TOML config");

        let connection_string = conn_string_value["database"]["url"]
            .as_str()
            .unwrap();

        println!("config for db:\n {}", config_file);
        println!("conn string: {}", connection_string);
        
        return connection_string.to_string();
    }
}
