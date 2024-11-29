use crate::error::AppError;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub auth: AuthConfig,
}

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    port: u16,
    db_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthConfig {
    ek: String,
    dk: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, AppError> {
        let rdr = File::open("backend.yaml")?;
        let ret = serde_yaml::from_reader(rdr)?;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_should_work() -> Result<()> {
        let config = AppConfig::new()?;

        assert_eq!(config.server.port, 6688);
        assert_eq!(
            config.server.db_url,
            "postgres://postgres:postgres@localhost:5432/backend"
        );
        assert_eq!(config.auth.ek, "-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEIO86NLYAOor1kUohceuaT9susMROxY973ceRUg+LQx97\n-----END PRIVATE KEY-----\n");
        assert_eq!(config.auth.dk, "-----BEGIN PUBLIC KEY-----\nMCowBQYDK2VwAyEAlCHtaGQUJ64HH7fP2rxuqkhoOl6mEYbNJbPuvAdao6I=\n-----END PUBLIC KEY-----\n");

        Ok(())
    }
}
