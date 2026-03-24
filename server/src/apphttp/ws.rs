
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
    State(_conn): State<Arc<Connection>>,
) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    common::debug_println!("WebSocket connected");

    while let Some(Ok(AxumMessage::Text(text))) = socket.recv().await {
        match serde_json::from_str::<ClientMessage>(&text) {
            Ok(client_msg) => {
                // 1. Print the whole message here (outside the inner match)
                common::debug_println!("Received request: {:#?}", client_msg);

                // 2. Now match on the variants to handle specific logic
                match client_msg {
                    ClientMessage::CreateUser(data) => {
                        common::debug_println!("Creating user: {}", data.username);
                    }
                    ClientMessage::CreateChat(data) => {
                        common::debug_println!("Creating chat: {}", data.chatname);
                    }
                    ClientMessage::CreateMessage(data) => {
                        common::debug_println!("Message from {} to {}", data.username, data.to);
                    }
                }
            }
            Err(e) => {
                println!("Error deserializing: {}. Raw text: {}", e, text);
            }
        }
    }

    common::debug_println!("WebSocket disconnected");
}