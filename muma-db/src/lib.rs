use std::time::Duration;

use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

pub mod user;

#[derive(Debug, Default)]
pub struct Database {
    database_url: String,
}

impl Database {
    /// Creates a new instance of the database.
    ///
    /// Note: This should be a singleton
    ///
    /// Example:
    /// ```
    /// let db = Database::new("mysql://...");
    /// ```
    pub fn new(url: &str) -> Self {
        Database {
            database_url: String::from(url),
        }
    }

    /// Creates a new connection to the database with the appropriate pooling
    ///
    /// Example:
    /// ```
    /// let db = Database::new("mysql://...");
    /// let conn = db.connect().await?;
    /// ```
    pub async fn connect(self) -> anyhow::Result<MySqlPool> {
        let pool = MySqlPoolOptions::new()
            .max_connections(10)
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .connect(self.database_url.as_str())
            .await?;

        sqlx::migrate!().run(&pool).await?;

        Ok(pool)
    }
}
