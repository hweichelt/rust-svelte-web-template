pub mod auth;
pub mod health;

use axum::{
    Router,
    routing::{any, get},
};
use tower_http::trace::TraceLayer;

use crate::{error::AppError, state::AppState};

/// The JSON API. The frontend is served from the same origin, so there is no
/// CORS layer; `SameSite=Lax` on the session cookie and JSON-only request
/// bodies keep cross-site requests out.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health))
        .nest("/api/auth", auth::router())
        // Unknown API paths get a JSON 404 rather than the SPA shell.
        .route("/api/{*path}", any(not_found))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn not_found() -> AppError {
    AppError::NotFound("no such endpoint".to_owned())
}
