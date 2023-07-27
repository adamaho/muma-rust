use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct User {
    user_id: i32,
    username: String,
    created_at: DateTime<Utc>,
}

/// Selects a user from the database by the provided id
pub async fn select_by_id(pool: &MySqlPool, user_id: i32) -> anyhow::Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
SELECT user_id, username, created_at FROM users
WHERE user_id = ?
            "#,
        user_id
    )
    .fetch_one(pool)
    .await
}
