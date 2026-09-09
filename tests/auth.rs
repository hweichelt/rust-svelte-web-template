//! End-to-end tests for registration, login, session cookies, and logout.
//! They run against the database in `DATABASE_URL` (loaded from `.env`).

use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode, header},
};
use http_body_util::BodyExt;
use migration::{Migrator, MigratorTrait};
use myapp_server::{config::Config, routes, state::AppState};
use serde_json::{Value, json};
use tower::ServiceExt;
use uuid::Uuid;

async fn app() -> Router {
    dotenvy::dotenv().ok();
    let config = Config::from_env().expect("config");
    let db = myapp_server::connect_db(&config).await.expect("db");
    Migrator::up(&db, None).await.expect("migrations");
    routes::router(AppState::new(db, config)).expect("router")
}

async fn send(app: &Router, req: Request<Body>) -> (StatusCode, Option<String>, Value) {
    let res = app.clone().oneshot(req).await.expect("request");
    let status = res.status();
    let set_cookie = res
        .headers()
        .get(header::SET_COOKIE)
        .map(|v| v.to_str().unwrap().to_owned());
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let body = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).expect("json body")
    };
    (status, set_cookie, body)
}

fn post_json(uri: &str, body: Value, cookie: Option<&str>) -> Request<Body> {
    let mut req = Request::post(uri).header(header::CONTENT_TYPE, "application/json");
    if let Some(cookie) = cookie {
        req = req.header(header::COOKIE, cookie);
    }
    req.body(Body::from(body.to_string())).unwrap()
}

fn get(uri: &str, cookie: Option<&str>) -> Request<Body> {
    let mut req = Request::get(uri);
    if let Some(cookie) = cookie {
        req = req.header(header::COOKIE, cookie);
    }
    req.body(Body::empty()).unwrap()
}

/// Reduce a `Set-Cookie` header to the `name=value` pair for the `Cookie` request header.
fn cookie_pair(set_cookie: &str) -> String {
    set_cookie.split(';').next().unwrap().to_owned()
}

#[tokio::test]
async fn register_login_logout_flow() {
    let app = app().await;
    let email = format!("test-{}@example.com", Uuid::new_v4());

    // Register: 201, returns the user, sets an HttpOnly session cookie.
    let (status, set_cookie, body) = send(
        &app,
        post_json(
            "/api/auth/register",
            json!({ "email": email.to_uppercase(), "display_name": "  Test User ", "password": "correct horse" }),
            None,
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{body}");
    assert_eq!(body["email"], email, "email is normalized to lowercase");
    assert_eq!(body["display_name"], "Test User");
    assert!(
        body.get("password_hash").is_none(),
        "hash must never be exposed"
    );
    let set_cookie = set_cookie.expect("session cookie set on register");
    assert!(set_cookie.contains("HttpOnly"), "{set_cookie}");
    assert!(set_cookie.contains("SameSite=Lax"), "{set_cookie}");
    let cookie = cookie_pair(&set_cookie);

    // Same email again: 409.
    let (status, _, body) = send(
        &app,
        post_json(
            "/api/auth/register",
            json!({ "email": email, "display_name": "Other", "password": "correct horse" }),
            None,
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT, "{body}");
    assert_eq!(body["error"]["code"], "conflict");

    // The registration cookie authenticates /me.
    let (status, _, body) = send(&app, get("/api/auth/me", Some(&cookie))).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["email"], email);

    // Wrong password: 401 with no cookie.
    let (status, set_cookie, _) = send(
        &app,
        post_json(
            "/api/auth/login",
            json!({ "email": email, "password": "wrong" }),
            None,
        ),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert!(set_cookie.is_none());

    // Unknown email: also 401.
    let (status, _, _) = send(
        &app,
        post_json(
            "/api/auth/login",
            json!({ "email": "nobody@example.com", "password": "correct horse" }),
            None,
        ),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    // Correct password: 200 with a fresh cookie that also works as a bearer token.
    let (status, set_cookie, body) = send(
        &app,
        post_json(
            "/api/auth/login",
            json!({ "email": email, "password": "correct horse" }),
            None,
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let login_cookie = cookie_pair(&set_cookie.expect("cookie set on login"));
    let token = login_cookie.split_once('=').unwrap().1;
    let (status, _, _) = send(
        &app,
        Request::get("/api/auth/me")
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    // Logout revokes only the session used; the other one stays valid.
    let (status, set_cookie, _) = send(
        &app,
        post_json("/api/auth/logout", json!({}), Some(&login_cookie)),
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    assert!(set_cookie.expect("removal cookie").contains("Max-Age=0"));
    let (status, _, _) = send(&app, get("/api/auth/me", Some(&login_cookie))).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    let (status, _, _) = send(&app, get("/api/auth/me", Some(&cookie))).await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn rejects_invalid_input() {
    let app = app().await;

    let cases = [
        (
            json!({ "email": "not-an-email", "display_name": "A", "password": "correct horse" }),
            "email",
        ),
        (
            json!({ "email": "a@b.co", "display_name": "   ", "password": "correct horse" }),
            "display_name",
        ),
        (
            json!({ "email": "a@b.co", "display_name": "A", "password": "short" }),
            "password",
        ),
    ];
    for (body, field) in cases {
        let (status, _, res) = send(&app, post_json("/api/auth/register", body, None)).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{field}: {res}");
        assert_eq!(res["error"]["code"], "validation_error");
        assert!(res["error"]["message"].as_str().unwrap().contains(field));
    }

    // Malformed JSON is a validation error too, not a plain-text 4xx.
    let req = Request::post("/api/auth/register")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from("{not json"))
        .unwrap();
    let (status, _, res) = send(&app, req).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(res["error"]["code"], "validation_error");

    // No session at all.
    let (status, _, res) = send(&app, get("/api/auth/me", None)).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(res["error"]["code"], "unauthorized");
}

#[tokio::test]
async fn health_reports_ok() {
    let app = app().await;
    let (status, _, body) = send(&app, get("/health", None)).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "ok");
}
