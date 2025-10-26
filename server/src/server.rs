use crate::connection::Connection;

pub struct Server {
    conn: Connection,
}

impl Server {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let conn_string = Self::read_conn_string();
        let conn = Connection::new(conn_string).await?;
        Ok(Self { conn })
    }

    fn read_conn_string() -> String {
        "postgres://user:password@localhost/dbname".to_string()
    }
}
