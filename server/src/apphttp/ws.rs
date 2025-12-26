
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use std::sync::Arc;

use crate::connection::Connection;

pub async fn WsHandler() -> impl IntoResponse {

}