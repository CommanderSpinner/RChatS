use sqlx::{PgPool, Error};

pub struct Connection {
    pool: PgPool,
    conn_string: String,
}

impl Connection {
    pub async fn new(connection_string: String) -> Result<Connection, Error> {
        let pool = PgPool::connect(&connection_string).await?;
        Ok(Connection {
            pool,
            conn_string: connection_string,
        })
    }
}
