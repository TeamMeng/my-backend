use anyhow::Result;
use dotenv::var;

pub struct AppConfig {
    pub port: u16,
    pub db_url: String,
    pub ek: String,
    pub dk: String,
}

impl AppConfig {
    pub fn new() -> Result<Self> {
        let db_url = var("DATABASE_URL")?;
        let port = var("PORT")?.parse::<u16>()?;
        let ek = include_str!("../fixtures/encoding.pem");
        let dk = include_str!("../fixtures/decoding.pem");
        Ok(Self {
            port,
            db_url,
            ek: ek.to_string(),
            dk: dk.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_should_work() -> Result<()> {
        let config = AppConfig::new()?;

        assert_eq!(config.port, 6688);
        assert_eq!(
            config.db_url,
            "postgres://postgres:postgres@localhost:5432/backend"
        );
        assert_eq!(config.ek, "-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEIO86NLYAOor1kUohceuaT9susMROxY973ceRUg+LQx97\n-----END PRIVATE KEY-----\n");
        assert_eq!(config.dk, "-----BEGIN PUBLIC KEY-----\nMCowBQYDK2VwAyEAlCHtaGQUJ64HH7fP2rxuqkhoOl6mEYbNJbPuvAdao6I=\n-----END PUBLIC KEY-----\n");

        Ok(())
    }
}
