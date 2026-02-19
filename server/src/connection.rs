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

        let from_uid = self.get_uid(&m.username).await?;
        let to_uid = self.get_uid(&m.to).await?;
        let cid = self.get_cid(from_uid, to_uid).await?;

        let url: String = "files/chat_data/".to_string() + &m.username.to_string();


        sqlx::query("INSERT INTO message(cid, uid, url, content) VALUES ($1, $2, $3, $4)")
            .bind(cid)
            .bind(from_uid)
            .bind(url)
            .bind(m.content.clone())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // read out matching uid from db
    pub async fn get_uid(&self, username: &str) -> Result<i64, sqlx::Error> {
        let user = sqlx::query("SELECT * FROM \"user\" WHERE username= $1")
            .bind(username)
            .fetch_one(&self.pool)
            .await?;
        let uid: i64 = user.get("uid");

        Ok(uid)
    }

    // read out matching cid from db
    pub async fn get_cid(&self, from: i64, to: i64) -> Result<i64, sqlx::Error> {
        let cid = sqlx::query_scalar("SELECT cid FROM chat WHERE user_ids = ARRAY[$1, $2] OR user_ids = ARRAY[$2, $1]")
            .bind(from)
            .bind(to)
            .fetch_one(&self.pool)
            .await?;

        Ok(cid)
    }

    // returns messages. count defines how many messages should be read
    pub async fn read_messages_from_chat(&self, cid: i64, count: i64) -> Result<Vec<Message>, sqlx::Error> {
        let m: Vec<Message> = sqlx::query_as::<_, Message>("SELECT * FROM message WHERE cid = $1 ORDER BY created_at LIMIT $2;")
            .bind(cid)
            .bind(count)
            .fetch_all(&self.pool)
            .await?;
        Ok(m)
    }
}
