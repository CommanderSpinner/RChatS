use serde::{Serialize, Deserialize};
use colored::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ClientMessage {
    CreateUser(create_user),
    CreateChat(create_chat),
    CreateMessage(create_message),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct create_user {
    pub username: String,
    pub plain_password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct create_chat {
    pub chatname: String,
    pub userids: [i64; 2],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct create_message {
    pub username: String,
    pub plain_password: String,
    pub to: String, // to is to which username it will be send 
    pub content: String,
}

// macros for debugging

// for printing something only when debuging but not in prod 
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]{
            
            use colored::Colorize;

            print!("{}", "Debug msg: ".blue());
            println!("{}", format!($($arg)*).blue());
        }
        #[cfg(not(debug_assertions))]
        {} // do nothing in release
    };
}