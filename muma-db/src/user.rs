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
///
/// Example:
///
/// ```
/// let user = user::select_by_id(&pool, user_id).await?;
/// ```
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

#[derive(Debug, FromRow, Serialize)]
pub struct UserForm {
    pub username: String,
}

/// Inserts a new UserForm into the database
///
/// Example:
///
/// ```
/// let user = UserForm {
///     username: String::from("helloworld")
/// };
///
/// let user_id = user::insert_user(&pool, user).await?;
/// ```
pub async fn insert_user(pool: &MySqlPool, user: UserForm) -> anyhow::Result<u64, sqlx::Error> {
    let new_user_id = sqlx::query!(
        r#"
INSERT INTO users ( username )
VALUES ( ? )
        "#,
        &user.username
    )
    .execute(pool)
    .await?
    .last_insert_id();

    Ok(new_user_id)
}
