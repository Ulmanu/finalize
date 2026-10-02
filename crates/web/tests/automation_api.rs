#![cfg(feature = "ssr")]

//! Automation API: scoped bearer tokens, versioned REST routes, typed errors, and settings.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::api_tokens::{ApiScope, ApiTokenService};
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::AccountId;
use momentum_testkit::api_tokens::MemoryApiTokenStore;
use momentum_testkit::auth::{
    DeterministicHasher, DeterministicTokenGenerator, MemoryAuthStore, TestClock,
};
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::todos::MemoryTodoReadStore;
use momentum_web::auth_routes::AuthContext;
use momentum_web::server::{self, AppState};
use momentum_web::session::CookieSessionResolver;
use tower::ServiceExt;

struct Harness {
    router: axum::Router,
    tokens: Arc<ApiTokenService>,
    todos: Arc<TodoService>,
    account_id: AccountId,
    cookie: String,
}

async fn harness() -> Harness {
    let store = Arc::new(MemoryAuthStore::new());
    let clock = Arc::new(TestClock::new(
        Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap(),
    ));
    let auth = Arc::new(AuthService::new(
        store.clone(),
        store.clone(),
        store.clone(),
        Arc::new(DeterministicHasher),
        Arc::new(DeterministicTokenGenerator::new()),
        clock.clone(),
    ));
    let database = MemoryDatabase::new();
    let dispatcher = Arc::new(CommandDispatcher::new(
        Arc::new(database.clone()),
        clock.clone(),
    ));
    let todos = Arc::new(TodoService::new(
        dispatcher,
        Arc::new(MemoryTodoReadStore::new(database)),
        clock,
    ));
    let tokens = Arc::new(ApiTokenService::new(
        Arc::new(MemoryApiTokenStore::new()),
        Arc::new(DeterministicTokenGenerator::new()),
    ));

    let registration = auth
        .register("api@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("api@example.com", "correct horse battery")
        .await
        .expect("login");

    let state = AppState::new(
        server::leptos_options(),
        None,
        Arc::new(CookieSessionResolver::new(auth.clone())),
        Some(Arc::new(AuthContext::new(auth, false))),
        Some(todos.clone()),
        None,
        None,
    )
    .with_api_tokens(tokens.clone());

    Harness {
        router: server::router_with_state(state),
        tokens,
        todos,
        account_id: registration.account_id,
        cookie: format!("momentum_session={}", login.token),
    }
}

async fn request(
    h: &Harness,
    method: &str,
    uri: &str,
    bearer: Option<&str>,
    body: Option<&str>,
    content_type: Option<&str>,
) -> (StatusCode, Option<String>, String) {
    let mut builder = Request::builder()
        .method(method)
        .uri(uri)
        .header(header::COOKIE, &h.cookie);
    if let Some(bearer) = bearer {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {bearer}"));
    }
    if let Some(content_type) = content_type {
        builder = builder.header(header::CONTENT_TYPE, content_type);
    }
    let request = builder
        .body(body.map_or_else(Body::empty, |body| Body::from(body.to_string())))
        .expect("request");
    let response = h.router.clone().oneshot(request).await.expect("response");
    let status = response.status();
    let location = response
        .headers()
        .get(header::LOCATION)
        .and_then(|value| value.to_str().ok())
        .map(str::to_string);
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    (
        status,
        location,
        String::from_utf8(bytes.to_vec()).expect("utf-8"),
    )
}

async fn token(h: &Harness, name: &str, scopes: &[ApiScope]) -> String {
    h.tokens
        .create(h.account_id, name, scopes, Utc::now())
        .await
        .expect("create")
        .token
}

#[tokio::test]
async fn bearer_tokens_are_required_and_scoped() {
    let h = harness().await;
    let (status, _, body) = request(&h, "GET", "/api/v1/todos", None, None, None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert!(body.contains("\"code\":\"unauthorized\""), "{body}");

    let (status, _, _) = request(&h, "GET", "/api/v1/todos", Some("garbage"), None, None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let reader = token(&h, "reader", &[ApiScope::Read]).await;
    let (status, _, body) = request(&h, "GET", "/api/v1/todos", Some(&reader), None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.starts_with('['), "{body}");

    let (status, _, body) = request(
        &h,
        "POST",
        "/api/v1/todos",
        Some(&reader),
        Some("{\"title\":\"Nope\"}"),
        Some("application/json"),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert!(body.contains("\"code\":\"forbidden\""), "{body}");
    assert!(h
        .todos
        .list(h.account_id, TodoScope::Inbox)
        .await
        .expect("inbox")
        .is_empty());
}

#[tokio::test]
async fn write_tokens_create_update_and_act_on_todos() {
    let h = harness().await;
    let writer = token(&h, "writer", &[ApiScope::Write]).await;

    let (status, _, body) = request(
        &h,
        "POST",
        "/api/v1/todos",
        Some(&writer),
        Some("{\"title\":\"From API\",\"notes\":\"Note\",\"when\":\"tomorrow\"}"),
        Some("application/json"),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert!(body.contains("\"title\":\"From API\""), "{body}");
    assert!(body.contains("\"status\":\"open\""), "{body}");
    let id = h
        .todos
        .list(h.account_id, TodoScope::AllOpen)
        .await
        .expect("all open")
        .into_iter()
        .find(|todo| todo.title == "From API")
        .expect("todo")
        .id;

    let (status, _, body) = request(
        &h,
        "GET",
        &format!("/api/v1/todos/{id}"),
        Some(&writer),
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("\"id\":\""), "{body}");

    let (status, _, body) = request(
        &h,
        "PATCH",
        &format!("/api/v1/todos/{id}"),
        Some(&writer),
        Some("{\"title\":\"Updated\",\"deadline\":\"2026-06-20\"}"),
        Some("application/json"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("\"title\":\"Updated\""), "{body}");
    assert!(body.contains("\"deadline\":\"2026-06-20\""), "{body}");

    let (status, _, body) = request(
        &h,
        "POST",
        &format!("/api/v1/todos/{id}/complete"),
        Some(&writer),
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("\"status\":\"ok\""), "{body}");
    assert!(h
        .todos
        .get(h.account_id, id)
        .await
        .unwrap()
        .unwrap()
        .completed_at
        .is_some());

    // Unsupported values and actions are typed bad requests; unknown ids are typed 404s.
    let (status, _, body) = request(
        &h,
        "POST",
        "/api/v1/todos",
        Some(&writer),
        Some("{\"title\":\"Bad\",\"when\":\"banana\"}"),
        Some("application/json"),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(body.contains("\"code\":\"invalid_request\""), "{body}");

    let (status, _, body) = request(
        &h,
        "POST",
        &format!("/api/v1/todos/{id}/explode"),
        Some(&writer),
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(body.contains("\"code\":\"invalid_request\""), "{body}");

    let (status, _, body) = request(
        &h,
        "GET",
        &format!("/api/v1/todos/{}", momentum_domain::EntityId::new()),
        Some(&writer),
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(body.contains("\"code\":\"not_found\""), "{body}");
}

#[tokio::test]
async fn rotation_and_revocation_take_effect_immediately() {
    let h = harness().await;
    let created = h
        .tokens
        .create(h.account_id, "rotating", &[ApiScope::Read], Utc::now())
        .await
        .expect("create");
    let rotated = h
        .tokens
        .rotate(h.account_id, created.summary.id)
        .await
        .expect("rotate")
        .expect("rotated");

    let (status, _, _) =
        request(&h, "GET", "/api/v1/todos", Some(&created.token), None, None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    let (status, _, _) =
        request(&h, "GET", "/api/v1/todos", Some(&rotated.token), None, None).await;
    assert_eq!(status, StatusCode::OK);

    h.tokens
        .revoke(h.account_id, created.summary.id, Utc::now())
        .await
        .expect("revoke");
    let (status, _, _) =
        request(&h, "GET", "/api/v1/todos", Some(&rotated.token), None, None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn settings_creates_tokens_with_view_once_display() {
    let h = harness().await;
    let (status, _, body) = request(&h, "GET", "/settings", None, None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("data-testid=\"api-tokens\""), "{body}");
    assert!(body.contains("data-testid=\"api-token-form\""), "{body}");

    let (status, location, _) = request(
        &h,
        "POST",
        "/settings/tokens",
        None,
        Some("name=CI&scopes=read"),
        Some("application/x-www-form-urlencoded"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let location = location.expect("location");
    assert!(location.starts_with("/settings?token="), "{location}");
    let plaintext = location
        .strip_prefix("/settings?token=")
        .expect("token param")
        .to_string();

    let (status, _, body) = request(
        &h,
        "GET",
        &format!("/settings?token={plaintext}"),
        None,
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("data-testid=\"api-token-view-once\""),
        "{body}"
    );
    assert!(body.contains(&plaintext), "{body}");
    assert!(body.contains("data-testid=\"api-token\""), "{body}");
    assert!(body.contains("CI"), "{body}");

    // The plaintext is not persisted: the token list only ever shows metadata.
    let listed = h.tokens.list(h.account_id).await.expect("list");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].name, "CI");
    assert_eq!(listed[0].scopes, vec![ApiScope::Read]);
}
