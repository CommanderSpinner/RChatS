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
