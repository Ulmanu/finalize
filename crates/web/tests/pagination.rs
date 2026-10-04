#![cfg(feature = "ssr")]

//! Large-list windowing: bounded API list reads.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::api_tokens::{ApiScope, ApiTokenService};
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::todos::TodoService;
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
        .register("page@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("page@example.com", "correct horse battery")
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

async fn get(h: &Harness, uri: &str, bearer: &str) -> (StatusCode, String) {
    let request = Request::builder()
        .method("GET")
        .uri(uri)
        .header(header::COOKIE, &h.cookie)
        .header(header::AUTHORIZATION, format!("Bearer {bearer}"))
        .body(Body::empty())
        .expect("request");
    let response = h.router.clone().oneshot(request).await.expect("response");
    let status = response.status();
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    (status, String::from_utf8(bytes.to_vec()).expect("utf-8"))
}

#[tokio::test]
async fn list_reads_are_windowed_with_clamped_limits() {
    let h = harness().await;
    for index in 0..5 {
        h.todos
            .create(h.account_id, &format!("Task {index}"), None)
            .await
            .expect("create");
    }
    let reader = h
        .tokens
        .create(h.account_id, "reader", &[ApiScope::Read], Utc::now())
        .await
        .expect("token")
        .token;

    let (status, body) = get(&h, "/api/v1/todos?scope=all&limit=2", &reader).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.matches("\"id\":").count(), 2, "{body}");

    let (status, body) = get(&h, "/api/v1/todos?scope=all&limit=2&offset=2", &reader).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.matches("\"id\":").count(), 2, "{body}");

    let (status, body) = get(&h, "/api/v1/todos?scope=all&limit=999999", &reader).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.matches("\"id\":").count(), 5, "{body}");

    let (status, body) = get(&h, "/api/v1/todos?scope=all", &reader).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.matches("\"id\":").count(), 5, "{body}");

    let (status, _) = get(&h, "/api/v1/todos?scope=all&limit=abc", &reader).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}
