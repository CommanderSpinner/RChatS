use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub username: String,
    pub password: String,
    pub to: String,
    pub content: MessageContent,
}


// macros for debugging

// for printing something only when debuging but not in prod 
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