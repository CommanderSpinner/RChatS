use sqlx::{PgPool, Error};

//use crate::DB_tables::*;
use crate::DB_tables::user::User;
use crate::DB_tables::message::Message;
use crate::DB_tables::chat::Chat;

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

    pub async fn create_user(&self, u: &User) {

    }

    pub async fn create_chat(&self, c: &Chat) {

    }

    pub async fn create_message(&self, m: &Message) {

    }
}
