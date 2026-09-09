use std::net::SocketAddr;

use anyhow::Context;

/// Runtime configuration, read from environment variables (and `.env` in development).
#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub bind_addr: SocketAddr,
    /// Origins allowed by CORS. Credentials are always allowed for these.
    pub cors_origins: Vec<String>,
    pub session_ttl: chrono::Duration,
    /// Set the `Secure` flag on the session cookie. Must be `true` behind HTTPS.
    pub cookie_secure: bool,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?;

        let bind_addr = std::env::var("BIND_ADDR")
            .unwrap_or_else(|_| "127.0.0.1:3000".to_owned())
            .parse()
            .context("BIND_ADDR is not a valid socket address")?;

        let cors_origins = std::env::var("CORS_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:5173".to_owned())
            .split(',')
            .map(|s| s.trim().to_owned())
            .filter(|s| !s.is_empty())
            .collect();

        let session_ttl_days: i64 = match std::env::var("SESSION_TTL_DAYS") {
            Ok(raw) => raw.parse().context("SESSION_TTL_DAYS must be an integer")?,
            Err(_) => 30,
        };
        anyhow::ensure!(session_ttl_days > 0, "SESSION_TTL_DAYS must be positive");

        let cookie_secure = std::env::var("COOKIE_SECURE")
            .map(|v| matches!(v.as_str(), "1" | "true" | "yes"))
            .unwrap_or(false);

        Ok(Self {
            database_url,
            bind_addr,
            cors_origins,
            session_ttl: chrono::Duration::days(session_ttl_days),
            cookie_secure,
        })
    }
}
