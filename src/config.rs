use std::{net::SocketAddr, path::PathBuf};

use anyhow::Context;

/// Runtime configuration, read from environment variables (and `.env` in development).
#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub bind_addr: SocketAddr,
    pub session_ttl: chrono::Duration,
    /// Set the `Secure` flag on the session cookie. Must be `true` behind HTTPS.
    pub cookie_secure: bool,
    /// Debug builds only: port of the Vite dev server that page requests are proxied to.
    pub vite_port: u16,
    /// Debug builds only: start `npm run dev` in `frontend_dir` alongside the server.
    pub vite_auto_start: bool,
    /// Debug builds only: the SvelteKit project directory.
    pub frontend_dir: PathBuf,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?;

        let bind_addr = std::env::var("BIND_ADDR")
            .unwrap_or_else(|_| "127.0.0.1:3000".to_owned())
            .parse()
            .context("BIND_ADDR is not a valid socket address")?;

        let session_ttl_days: i64 = match std::env::var("SESSION_TTL_DAYS") {
            Ok(raw) => raw.parse().context("SESSION_TTL_DAYS must be an integer")?,
            Err(_) => 30,
        };
        anyhow::ensure!(session_ttl_days > 0, "SESSION_TTL_DAYS must be positive");

        let cookie_secure = env_flag("COOKIE_SECURE", false);

        let vite_port = match std::env::var("VITE_PORT") {
            Ok(raw) => raw.parse().context("VITE_PORT must be a port number")?,
            Err(_) => 5173,
        };
        let vite_auto_start = env_flag("VITE_AUTO_START", true);
        let frontend_dir = std::env::var("FRONTEND_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("frontend"));

        Ok(Self {
            database_url,
            bind_addr,
            session_ttl: chrono::Duration::days(session_ttl_days),
            cookie_secure,
            vite_port,
            vite_auto_start,
            frontend_dir,
        })
    }
}

/// Read a boolean variable; `1`, `true` and `yes` are true, anything else false.
fn env_flag(name: &str, default: bool) -> bool {
    std::env::var(name)
        .map(|v| matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes"))
        .unwrap_or(default)
}
