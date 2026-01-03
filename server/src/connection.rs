use sqlx::{PgPool, Error};

#[derive(Clone)]
pub struct Connection {
    pool: PgPool,
}

impl Connection {
    pub async fn new(connection_string: String) -> Result<Connection, Error> {
        let pool = PgPool::connect(&connection_string).await?;
        Ok(Connection {
            pool,
        })
    }

    pub async fn create_user() {

    }

    pub async fn create_chat() {

    }

    pub async fn create_message() {
        
    }
}
