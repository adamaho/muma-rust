use anyhow::Context;
use std::path::Path;
use std::time::Duration;

use sea_orm::{ConnectOptions, Database as SeaOrmDatabase, DatabaseConnection};

#[derive(Debug, Default)]
pub struct Database {
    mysql_url: String,
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
            mysql_url: String::from(url),
        }
    }

    /// Creates a new database instance from a given environment file. Specifically useful for
    /// development when we need to get a database connection.
    ///
    /// Example:
    /// ```
    /// let db = Database::from_env("../path");
    /// ```
    pub fn from_env_path(path: &str) -> anyhow::Result<Database> {
        let env_items = dotenvy::from_path_iter(Path::new(path))
            .with_context(|| format!("muma-db: Failed to read file from {}", path))?;

        let mut db = Database::default();

        for item in env_items {
            let (key, value) = item?;
            if key == String::from("MYSQL_URL") {
                db.mysql_url = value;
                break;
            }
        }

        Ok(db)
    }

    /// Creates a new connection to the database with the appropriate pooling
    ///
    /// Example:
    /// ```
    /// let db = Database::new("adam", "foobar", "localhost", "5713", "muma");
    /// let conn = db.connect().await?;
    /// ```
    pub async fn connect(self) -> anyhow::Result<DatabaseConnection> {
        let mut opt = ConnectOptions::new(self.mysql_url).to_owned();

        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);

        let db = SeaOrmDatabase::connect(opt).await?;

        Ok(db)
    }
}
