//! Serving the SvelteKit app from this binary.
//!
//! In release builds the contents of `frontend/build` are embedded at compile
//! time and served directly. In debug builds nothing is embedded; requests for
//! the app are proxied to the Vite dev server instead, so `cargo run` gives
//! hot module reloading without a separate frontend server.

use axum::Router;
use axum_vite::{DevServerHandle, ViteConfig, frameworks::Framework};

use crate::config::Config;

/// Everything the app needs to serve (or proxy) the frontend.
pub struct Frontend {
    vite: ViteConfig,
}

impl Frontend {
    pub fn new(config: &Config) -> Self {
        let vite = ViteConfig {
            dev_port: config.vite_port,
            frontend_root: Some(config.frontend_dir.clone()),
            dev_command: "npm run dev".to_owned(),
            auto_start: config.vite_auto_start,
            framework: Framework::Svelte,
            // SvelteKit does not use a Vite `base`; its assets live under
            // `/_app/`, so everything is served from the root.
            prefix: "/".to_owned(),
            dir: axum_vite::embedded_dir!("$CARGO_MANIFEST_DIR/frontend/build"),
            ..ViteConfig::default()
        };
        Self { vite }
    }

    /// Start the Vite dev server as a child process when configured to.
    /// Always `None` in release builds. Keep the handle alive: dropping it
    /// stops the dev server.
    pub fn spawn_dev_server(&self) -> Option<DevServerHandle> {
        self.vite.maybe_spawn_dev_server()
    }

    /// Router serving the SPA shell, its assets, and the client-side routes.
    /// Merge it *after* the API routes so those take precedence.
    pub fn router<S>(self) -> Router<S>
    where
        S: Clone + Send + Sync + 'static,
    {
        axum_vite::spa_router(self.vite)
    }
}
