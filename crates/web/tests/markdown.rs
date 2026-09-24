#![cfg(feature = "ssr")]

//! Markdown subset rendering web-surface tests.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::todos::TodoService;
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
    todos: Arc<TodoService>,
    account_id: momentum_domain::AccountId,
    cookie: String,
}

async fn harness() -> Harness {
    let store = Arc::new(MemoryAuthStore::new());
    let clock = Arc::new(TestClock::new(
        Utc.with_ymd_and_hms(2026, 6, 1, 9, 0, 0).unwrap(),
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

    let registration = auth
        .register("markdown@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("markdown@example.com", "correct horse battery")
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
    );

    Harness {
        router: server::router_with_state(state),
        todos,
        account_id: registration.account_id,
        cookie: format!("momentum_session={}", login.token),
    }
}

async fn request(h: &Harness, method: &str, uri: &str, body: Option<&str>) -> (StatusCode, String) {
    let mut builder = Request::builder()
        .method(method)
        .uri(uri)
        .header(header::COOKIE, &h.cookie);
    if body.is_some() {
        builder = builder.header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");
    }
    let request = builder
        .body(body.map_or_else(Body::empty, |body| Body::from(body.to_string())))
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

fn urlencode(value: &str) -> String {
    let mut out = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            out.push(char::from(byte));
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
    }
    out
}

async fn todo_with_notes(h: &Harness, notes: &str) -> momentum_domain::EntityId {
    let id = h
        .todos
        .create(h.account_id, "Markdown task", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    let body = format!("title=Markdown+task&notes={}", urlencode(notes));
    let (status, _) = request(h, "POST", &format!("/todos/{id}"), Some(&body)).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    id
}

#[tokio::test]
async fn preview_renders_the_supported_subset_while_source_stays_editable() {
    let h = harness().await;
    let source = "# Heading\n\n**bold** and *em* and ==mark==\n\n- one\n- two\n\n> quote\n\n```rust\nfn main() {}\n```";
    let id = todo_with_notes(&h, source).await;

    let (status, page) = request(&h, "GET", &format!("/todos/{id}"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(page.contains("data-testid=\"markdown-preview\""), "{page}");
    assert!(page.contains("<strong>bold</strong>"), "{page}");
    assert!(page.contains("<em>em</em>"), "{page}");
    assert!(page.contains("<mark"), "{page}");
    assert!(page.contains("md-list"), "{page}");
    assert!(page.contains("<blockquote"), "{page}");
    assert!(page.contains("Code block (rust)"), "{page}");

    let summary = h
        .todos
        .get(h.account_id, id)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(
        summary.notes, source,
        "stored source is preserved verbatim; rendering is derived"
    );
}

#[tokio::test]
async fn markdown_links_follow_the_safe_policy() {
    let h = harness().await;
    let id = todo_with_notes(
        &h,
        "[docs](https://example.com) and [bad](javascript:alert(1))",
    )
    .await;

    let (_, page) = request(&h, "GET", &format!("/todos/{id}"), None).await;
    assert!(page.contains("data-testid=\"markdown-link\""), "{page}");
    assert!(page.contains("href=\"https://example.com\""), "{page}");
    assert!(page.contains("rel=\"noopener noreferrer\""), "{page}");
    assert!(
        page.contains("data-testid=\"markdown-blocked-link\""),
        "{page}"
    );
    assert!(
        !page.contains("href=\"javascript:alert(1)"),
        "unsafe markdown links must never become hrefs: {page}"
    );
}
