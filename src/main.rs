use anyhow::Context;
use migration::{Migrator, MigratorTrait};
use myapp_server::{config::Config, frontend::Frontend, routes, state::AppState};
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let config = Config::from_env()?;
    let db = myapp_server::connect_db(&config).await?;
    Migrator::up(&db, None)
        .await
        .context("failed to apply database migrations")?;

    myapp_server::bindings::export_in_debug(&config.frontend_dir);
    let frontend = Frontend::new(&config);
    // Debug builds only: runs `npm run dev` until this handle is dropped.
    let _dev_server = frontend.spawn_dev_server();

    let state = AppState::new(db, config.clone());
    let app = routes::router(state).merge(frontend.router());

    let listener = TcpListener::bind(config.bind_addr)
        .await
        .with_context(|| format!("failed to bind {}", config.bind_addr))?;
    tracing::info!("listening on http://{}", config.bind_addr);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("server error")
}

async fn shutdown_signal() {
    if let Err(err) = tokio::signal::ctrl_c().await {
        tracing::error!(?err, "failed to listen for shutdown signal");
    }
    tracing::info!("shutting down");
}
