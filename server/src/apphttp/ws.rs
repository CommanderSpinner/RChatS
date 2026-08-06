
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use axum::extract::ws::Message as AxumMessage;
use common::ClientMessage;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

use crate::connection::Connection;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(conn): State<Arc<Connection>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, conn))
}
// it should be added that the json request ids get stored so if the get send twice they dont get executed twice

async fn handle_socket(
    mut socket: WebSocket,
    conn: Arc<Connection>,
) {
    common::debug_println!("WebSocket connected");

    while let Some(Ok(AxumMessage::Text(text))) = socket.recv().await {
        match serde_json::from_str::<ClientMessage>(&text) {
            Ok(client_msg) => {
                // print the whole message
                common::debug_println!("Received request: {:#?}", client_msg);

                // match on the variants to handle specific logic
                match client_msg {
                    ClientMessage::CreateUser(data) => {
                        common::debug_println!("Creating user: {}", data.username);
                    }
                    ClientMessage::CreateChat(data) => {
                        common::debug_println!("Creating chat: {}", data.chatname);
                        match conn.create_chat(&data).await {
                            Ok(_) => {
                                common::debug_println!("Chat created successfully");
                            }
                            Err(e) => {
                                eprintln!("Failed to create chat: {}", e);
                            }
                        }
                    }
                    ClientMessage::CreateMessage(data) => {
                        common::debug_println!("Message from {} to {}", data.username, data.to);
                    }
                    ClientMessage::GetContacts(data) => {
                                common::debug_println!("getting contacts for: {}", data.userid);

                    }
                }
            }
            Err(e) => {
                println!("Error deserializing: {}\n. Raw text: {}", e, text);
            }
        }
    }

    common::debug_println!("WebSocket disconnected");
}