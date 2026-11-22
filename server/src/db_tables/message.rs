use sqlx::FromRow;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Message {
    pub mid: i64,
    pub cid: i64,
    pub uid: i64,
    pub url: String,
    pub content: String,
    pub created_at: Option<NaiveDateTime>,
}