use sqlx::{PgPool, Error};

pub struct Connection {
    pool: PgPool,
}

impl Connection {
    // Create a new connection pool asynchronously
    pub async fn new(connection_string: &str) -> Result<Connection, Error> {
        let pool = PgPool::connect(connection_string).await?;
        Ok(Connection { pool })
    }    
}