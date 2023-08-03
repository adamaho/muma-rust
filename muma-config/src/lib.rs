use dotenv::dotenv;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    /// Loads config from environment variables
    ///
    /// Example:
    /// ```
    /// let config = Config::from_env();
    /// ```
    pub fn from_env() -> anyhow::Result<Self, config::ConfigError> {
        dotenv().ok();

        let config = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        config.try_deserialize::<Config>()
    }
}
