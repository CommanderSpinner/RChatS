use sqlx::{PgPool, Error, Row};

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};


//use crate::DB_tables::*;
use crate::db_tables::user::User;
use crate::db_tables::message::Message;
use crate::db_tables::chat::Chat;
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
    pub async fn create_user(&self, username: String, plain_password: String) -> Result<(), Error> {
        sqlx::query("INSERT INTO \"user\" (user_name, hashed_password) VALUES ($1, $2)")
            .bind(username)
            .bind(Self::hash_password(&plain_password).expect("Somethign went wrong hashing"))
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

    pub async fn validate_login(&self, username: &str, plain_password: &str) -> Result<bool, sqlx::Error> {
        // 1. Fetch the user record by username only
        let row: Option<(String,)> = sqlx::query_as("SELECT hashed_password FROM \"users\" WHERE user_name = $1")
            .bind(username)
            .fetch_optional(&self.pool)
            .await?;

        // 2. If user doesn't exist, return false
        let stored_hash_str = match row {
            Some(r) => r.0,
            None => return Ok(false),
        };

        // 3. Parse the stored hash string into a PasswordHash type
        let parsed_hash = PasswordHash::new(&stored_hash_str)
            .map_err(|_| sqlx::Error::Decode("Failed to parse password hash".into()))?;

        // 4. Verify the password attempt against the stored hash
        // Argon2::default() handles extracting the salt and parameters from the PHC string automatically
        let is_valid = Argon2::default()
            .verify_password(plain_password.as_bytes(), &parsed_hash)
            .is_ok();

        Ok(is_valid)
    }

    // Hashes a password using Argon2id and a random salt.
    // Returns the full PHC string (e.g., "$argon2id$v=19$m=4096...")
    fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        // Hash the password and convert to string format for DB storage
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;

        common::debug_println!("{}", password_hash.to_string());
        
        Ok(password_hash.to_string())
    }
}


