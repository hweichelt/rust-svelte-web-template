use axum::{
    Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use axum_extra::extract::CookieJar;
use chrono::{DateTime, FixedOffset, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, SqlErr};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::{
    auth::{CurrentUser, password, session},
    entities::{prelude::*, users},
    error::{AppError, AppResult},
    extract::Json,
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/me", get(me))
}

#[derive(Debug, Deserialize, TS)]
#[ts(export_to = "api.ts")]
pub struct RegisterRequest {
    pub email: String,
    pub display_name: String,
    pub password: String,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export_to = "api.ts")]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Public view of a user. Never expose `users::Model` directly: it carries the
/// password hash.
#[derive(Debug, Serialize, TS)]
#[ts(export_to = "api.ts")]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub created_at: DateTime<FixedOffset>,
}

impl From<users::Model> for UserResponse {
    fn from(user: users::Model) -> Self {
        Self {
            id: user.id,
            email: user.email,
            display_name: user.display_name,
            created_at: user.created_at,
        }
    }
}

async fn register(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(req): Json<RegisterRequest>,
) -> AppResult<(StatusCode, CookieJar, axum::Json<UserResponse>)> {
    let email = normalize_email(&req.email)?;
    let display_name = validate_display_name(&req.display_name)?;
    validate_password(&req.password)?;

    let password_hash = password::hash(req.password).await?;
    let now = Utc::now();
    let user = users::ActiveModel {
        id: Set(Uuid::now_v7()),
        email: Set(email),
        display_name: Set(display_name),
        password_hash: Set(password_hash),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
    }
    .insert(&state.db)
    .await
    .map_err(|err| match err.sql_err() {
        Some(SqlErr::UniqueConstraintViolation(_)) => {
            AppError::Conflict("an account with this email already exists".to_owned())
        }
        _ => err.into(),
    })?;

    let jar = start_session(&state, jar, user.id).await?;
    Ok((StatusCode::CREATED, jar, axum::Json(user.into())))
}

async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(req): Json<LoginRequest>,
) -> AppResult<(CookieJar, axum::Json<UserResponse>)> {
    let email = normalize_email(&req.email)?;
    let user = Users::find()
        .filter(users::Column::Email.eq(email))
        .one(&state.db)
        .await?;

    // Always run the (slow) verification so that an unknown email takes as
    // long as a wrong password.
    let stored_hash = user
        .as_ref()
        .map_or_else(|| password::DUMMY_HASH.clone(), |u| u.password_hash.clone());
    let ok = password::verify(req.password, stored_hash).await?;

    let Some(user) = user.filter(|_| ok) else {
        return Err(AppError::Unauthorized);
    };

    let jar = start_session(&state, jar, user.id).await?;
    Ok((jar, axum::Json(user.into())))
}

async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
    current: CurrentUser,
) -> AppResult<(StatusCode, CookieJar)> {
    session::revoke(&state.db, &current.token).await?;
    Ok((
        StatusCode::NO_CONTENT,
        jar.remove(session::removal_cookie()),
    ))
}

async fn me(current: CurrentUser) -> axum::Json<UserResponse> {
    axum::Json(current.user.into())
}

async fn start_session(state: &AppState, jar: CookieJar, user_id: Uuid) -> AppResult<CookieJar> {
    let ttl = state.config.session_ttl;
    let token = session::create(&state.db, user_id, ttl).await?;
    Ok(jar.add(session::cookie(token, ttl, state.config.cookie_secure)))
}

fn normalize_email(raw: &str) -> AppResult<String> {
    let email = raw.trim().to_lowercase();
    let valid = email.len() <= 254
        && email
            .split_once('@')
            .is_some_and(|(local, domain)| !local.is_empty() && domain.contains('.'));
    if !valid {
        return Err(AppError::Validation("email is not valid".to_owned()));
    }
    Ok(email)
}

fn validate_display_name(raw: &str) -> AppResult<String> {
    let name = raw.trim();
    if name.is_empty() || name.chars().count() > 80 {
        return Err(AppError::Validation(
            "display_name must be between 1 and 80 characters".to_owned(),
        ));
    }
    Ok(name.to_owned())
}

fn validate_password(password: &str) -> AppResult<()> {
    let len = password.chars().count();
    if !(8..=256).contains(&len) {
        return Err(AppError::Validation(
            "password must be between 8 and 256 characters".to_owned(),
        ));
    }
    Ok(())
}
