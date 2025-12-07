use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub username: String,
    pub password: String,
    pub to: String,
    pub content: MessageContent,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum MessageContent {
    Text(String),
    Image(String), // Base64-encoded
    Audio(String), // Base64-encoded 
}


// macros for debugging
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]{
            print!("Debug msg: ");
            println!($($arg)*);
        }

        #[cfg(not(debug_assertions))]
        {} // do nothing in release
    };
}