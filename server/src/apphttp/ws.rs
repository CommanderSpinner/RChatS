
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use common::debug_println;
use std::sync::Arc;

use crate::connection::Connection;

pub async fn ws_handler() -> impl IntoResponse {
    debug_println!("WebSocket connected");
    

}