use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct create_user {
    pub username: String,
    pub plain_password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct create_chat {
    pub chatname: String,
    pub username: [i64; 2],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct create_message {
    pub username: String,
    pub plain_password: String,
    pub to: String,
    pub content: String,
}

// macros for debugging

// for printing something only when debuging but not in prod 
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]{
            print!("\x1b[31m"); // 31 = red
            print!("Debug msg: ");
            println!($($arg)*);

            // Reset color to default
            print!("\x1b[0m");
        }

        #[cfg(not(debug_assertions))]
        {} // do nothing in release
    };
}