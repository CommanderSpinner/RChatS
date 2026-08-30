use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub uid: i64,
    pub user_name: String,
    pub hashed_password: String,
    pub created_at: chrono::NaiveDateTime,
}