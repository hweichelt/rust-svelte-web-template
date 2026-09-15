//! TypeScript bindings for the API's wire types, generated with ts-rs.
//!
//! The roots listed in [`export`] and everything reachable from them are
//! written to `frontend/src/lib/bindings/api.ts`. The same function runs from
//! `cargo test` and, in debug builds, on every server start, so the committed
//! file never drifts from the Rust types while you develop.

use std::path::Path;

use ts_rs::{Config, ExportError, TS};

use crate::{
    error::ErrorBody,
    routes::auth::{LoginRequest, RegisterRequest, UserResponse},
};

/// Output directory, relative to the frontend project.
pub const OUTPUT_DIR: &str = "src/lib/bindings";

/// Write the bindings under `frontend_dir`. Add a type here only if it is not
/// already reachable from one of the existing roots.
pub fn export(frontend_dir: &Path) -> Result<(), ExportError> {
    let cfg = Config::new().with_out_dir(frontend_dir.join(OUTPUT_DIR));
    ErrorBody::export_all(&cfg)?;
    UserResponse::export_all(&cfg)?;
    RegisterRequest::export_all(&cfg)?;
    LoginRequest::export_all(&cfg)?;
    Ok(())
}

/// Debug builds only: regenerate the bindings, logging instead of failing when
/// the frontend directory is not where the server was started from.
pub fn export_in_debug(frontend_dir: &Path) {
    if !cfg!(debug_assertions) {
        return;
    }
    match export(frontend_dir) {
        Ok(()) => tracing::debug!(
            "wrote TypeScript bindings to {}",
            frontend_dir.join(OUTPUT_DIR).display()
        ),
        Err(err) => tracing::warn!(%err, "failed to write TypeScript bindings"),
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    /// `cargo test` regenerates the committed bindings; commit the result.
    #[test]
    fn export_bindings() {
        super::export(Path::new("frontend")).expect("export bindings");
    }
}
