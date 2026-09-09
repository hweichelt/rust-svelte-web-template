//! Password hashing with Argon2id. Hashing is CPU-bound, so it runs on the
//! blocking thread pool to keep the async runtime responsive.

use std::sync::LazyLock;

use anyhow::{Context, anyhow};
use argon2::{
    Argon2,
    password_hash::{self, PasswordHasher, PasswordVerifier, phc::PasswordHash},
};

/// A valid hash of a throwaway password. Verifying against it when a login
/// names an unknown email keeps the response time the same as for a known one.
pub static DUMMY_HASH: LazyLock<String> = LazyLock::new(|| {
    Argon2::default()
        .hash_password(b"myapp-dummy-password")
        .expect("hashing a constant password cannot fail")
        .to_string()
});

pub async fn hash(password: String) -> anyhow::Result<String> {
    tokio::task::spawn_blocking(move || {
        Argon2::default()
            .hash_password(password.as_bytes())
            .map(|hash| hash.to_string())
            .map_err(|err| anyhow!("failed to hash password: {err}"))
    })
    .await
    .context("password hashing task panicked")?
}

/// Returns `Ok(true)` when `password` matches `hash`, `Ok(false)` when it does
/// not, and an error only if the stored hash is unusable.
pub async fn verify(password: String, hash: String) -> anyhow::Result<bool> {
    tokio::task::spawn_blocking(move || {
        let parsed = PasswordHash::new(&hash)
            .map_err(|err| anyhow!("stored password hash is malformed: {err}"))?;
        match Argon2::default().verify_password(password.as_bytes(), &parsed) {
            Ok(()) => Ok(true),
            Err(password_hash::Error::PasswordInvalid) => Ok(false),
            Err(err) => Err(anyhow!("password verification failed: {err}")),
        }
    })
    .await
    .context("password verification task panicked")?
}
