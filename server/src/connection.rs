use sqlx::{PgPool, Error, Row};

//use crate::DB_tables::*;
use crate::DB_tables::user::User;
use crate::DB_tables::message::Message;
use crate::DB_tables::chat::Chat;
//use crate::common::*;

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

    // create_user gets parameters directly as vars because hashed password is not in request and gets hashed on server
    pub async fn create_user(&self, username: String, hashed_password: String) -> Result<(), Error> {
        sqlx::query("INSERT INTO \"user\" (user_name, hashed_password) VALUES ($1, $2)")
            .bind(username)
            .bind(hashed_password)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn create_chat(&self, c: &common::create_chat) -> Result<(), Error> {
        sqlx::query("INSERT INTO chat(chat_name, user_ids) VALUES ($1, ARRAY[$2, $3])")
            .bind(c.chatname.clone())
            .bind(c.userids[0])
            .bind(c.userids[1])
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn create_message(&self, m: &common::create_message) -> Result<(), Error> {

        let url: String = "".to_string();


        sqlx::query("INSERT INTO message(cid, uid, url, content) VALUES ($1, $2, $3, $4)")
            .bind(self.get_cid(&m.username).await?)
            .bind(self.get_uid(&m.username, &m.to).await)
            .bind(url)
            .bind(m.content.clone())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // read out matching cid from db
    async fn get_cid(&self, username: &str) -> Result<i64, sqlx::Error> {
        let chat = sqlx::query("SELECT * FROM chat WHERE = $1")
            .bind(username)
            .fetch_one(&self.pool)
            .await?;
        let cid: i64 = chat.get("cid");

        Ok(cid)
    }

    // read out matching uid from db
    async fn get_uid(&self, username: &str, to: &str) -> i64 {
        -1
    }
}
