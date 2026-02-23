
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use axum::extract::ws::Message as AxumMessage;
use common::create_message;
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
        if let Ok(msg) = serde_json::from_str::<create_message>(&text) {
            common::debug_println!("{:?}", msg);
        } else {
            common::debug_println!("Failed to parse message");
        }
    }

    common::debug_println!("WebSocket disconnected");
}