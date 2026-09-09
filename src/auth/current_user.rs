use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use axum_extra::extract::CookieJar;

use crate::{
    auth::session::{self, COOKIE_NAME},
    entities::users,
    error::AppError,
    state::AppState,
};

/// Extractor for the authenticated user. Reads the session token from the
/// `myapp_session` cookie or, failing that, an `Authorization: Bearer` header.
/// Rejects the request with `401` when no valid session is present.
pub struct CurrentUser {
    pub user: users::Model,
    /// The raw session token the request authenticated with.
    pub token: String,
}

impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, AppError> {
        let token = token_from_parts(parts).ok_or(AppError::Unauthorized)?;
        let user = session::find_user(&state.db, &token)
            .await?
            .ok_or(AppError::Unauthorized)?;
        Ok(Self { user, token })
    }
}

fn token_from_parts(parts: &Parts) -> Option<String> {
    if let Some(cookie) = CookieJar::from_headers(&parts.headers).get(COOKIE_NAME) {
        return Some(cookie.value().to_owned());
    }
    parts
        .headers
        .get(AUTHORIZATION)?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
        .map(|token| token.trim().to_owned())
}
