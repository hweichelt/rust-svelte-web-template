//! Database-backed sessions. The client holds an opaque random token; only its
//! SHA-256 hash is stored, so a leaked database does not leak usable sessions.

use axum_extra::extract::cookie::{Cookie, SameSite};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{Duration, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, QueryFilter,
    Set,
};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::entities::{prelude::*, sessions, users};

pub const COOKIE_NAME: &str = "myapp_session";
const TOKEN_BYTES: usize = 32;

fn generate_token() -> String {
    let mut bytes = [0u8; TOKEN_BYTES];
    rand::fill(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

fn hash_token(token: &str) -> String {
    URL_SAFE_NO_PAD.encode(Sha256::digest(token.as_bytes()))
}

/// Create a session for `user_id` and return the raw token to hand to the client.
pub async fn create(
    db: &DatabaseConnection,
    user_id: Uuid,
    ttl: Duration,
) -> Result<String, DbErr> {
    let token = generate_token();
    let now = Utc::now();
    sessions::ActiveModel {
        id: Set(Uuid::now_v7()),
        user_id: Set(user_id),
        token_hash: Set(hash_token(&token)),
        created_at: Set(now.into()),
        expires_at: Set((now + ttl).into()),
    }
    .insert(db)
    .await?;
    Ok(token)
}

/// Resolve a raw token to its user. Expired sessions are deleted on sight.
pub async fn find_user(
    db: &DatabaseConnection,
    token: &str,
) -> Result<Option<users::Model>, DbErr> {
    let Some((session, user)) = Sessions::find()
        .filter(sessions::Column::TokenHash.eq(hash_token(token)))
        .find_also_related(Users)
        .one(db)
        .await?
    else {
        return Ok(None);
    };

    if session.expires_at < Utc::now() {
        session.delete(db).await?;
        return Ok(None);
    }
    Ok(user)
}

/// Delete the session behind `token`, if any.
pub async fn revoke(db: &DatabaseConnection, token: &str) -> Result<(), DbErr> {
    Sessions::delete_many()
        .filter(sessions::Column::TokenHash.eq(hash_token(token)))
        .exec(db)
        .await?;
    Ok(())
}

pub fn cookie(token: String, ttl: Duration, secure: bool) -> Cookie<'static> {
    Cookie::build((COOKIE_NAME, token))
        .path("/")
        .http_only(true)
        .secure(secure)
        .same_site(SameSite::Lax)
        .max_age(time::Duration::seconds(ttl.num_seconds()))
        .build()
}

/// A cookie that, when passed to `CookieJar::remove`, clears the session cookie.
pub fn removal_cookie() -> Cookie<'static> {
    Cookie::build((COOKIE_NAME, "")).path("/").build()
}
