//! myapp-server: API backend with database-backed session authentication,
//! serving the SvelteKit frontend from the same binary.

pub mod auth;
pub mod config;
pub mod entities;
pub mod error;
pub mod extract;
pub mod frontend;
pub mod routes;
pub mod state;

use anyhow::Context;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::config::Config;

/// Open the database connection pool described by `config`.
pub async fn connect_db(config: &Config) -> anyhow::Result<DatabaseConnection> {
    let mut options = ConnectOptions::new(config.database_url.clone());
    options.sqlx_logging(false);
    Database::connect(options)
        .await
        .context("failed to connect to the database")
}
