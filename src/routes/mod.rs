pub mod auth;
pub mod health;

use anyhow::Context;
use axum::{
    Router,
    http::{HeaderValue, Method, header},
    routing::get,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::state::AppState;

pub fn router(state: AppState) -> anyhow::Result<Router> {
    let origins = state
        .config
        .cors_origins
        .iter()
        .map(|origin| {
            origin
                .parse::<HeaderValue>()
                .with_context(|| format!("invalid CORS origin {origin:?}"))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_credentials(true)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    Ok(Router::new()
        .route("/health", get(health::health))
        .nest("/api/auth", auth::router())
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state))
}
