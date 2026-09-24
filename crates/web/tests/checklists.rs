#![cfg(feature = "ssr")]

//! Checklist web-surface tests.

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
        .register("checklist@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("checklist@example.com", "correct horse battery")
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

async fn todo(h: &Harness) -> momentum_domain::EntityId {
    h.todos
        .create(h.account_id, "Web checklist", None)
        .await
        .expect("create")
        .entity_id
        .expect("id")
}

#[tokio::test]
async fn checklist_editor_renders_and_edits_items() {
    let h = harness().await;
    let id = todo(&h).await;

    let (status, _) = request(
        &h,
        "POST",
        &format!("/todos/{id}/checklist"),
        Some(&format!("title=First+step&return_to=%2Ftodos%2F{id}")),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);

    let (status, body) = request(&h, "GET", &format!("/todos/{id}"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("data-testid=\"checklist-editor\""), "{body}");
    assert!(body.contains("First step"), "{body}");

    let summary = h
        .todos
        .get(h.account_id, id)
        .await
        .expect("get")
        .expect("todo");
    let item = summary.checklist[0].id;

    let (status, _) = request(
        &h,
        "POST",
        &format!("/todos/{id}/checklist/{item}/toggle"),
        Some(&format!("completed=1&return_to=%2Ftodos%2F{id}")),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let summary = h
        .todos
        .get(h.account_id, id)
        .await
        .expect("get")
        .expect("todo");
    assert!(summary.checklist[0].completed);

    let (status, _) = request(
        &h,
        "POST",
        &format!("/todos/{id}/checklist/{item}/rename"),
        Some(&format!("title=Renamed+step&return_to=%2Ftodos%2F{id}")),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let summary = h
        .todos
        .get(h.account_id, id)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(summary.checklist[0].title, "Renamed step");

    let (status, _) = request(
        &h,
        "POST",
        &format!("/todos/{id}/checklist/{item}/remove"),
        Some(&format!("return_to=%2Ftodos%2F{id}")),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let summary = h
        .todos
        .get(h.account_id, id)
        .await
        .expect("get")
        .expect("todo");
    assert!(summary.checklist.is_empty());
}

#[tokio::test]
async fn bulk_expansion_endpoint_adds_ordered_items() {
    let h = harness().await;
    let id = todo(&h).await;

    let body = format!("text=-+%5B+%5D+Alpha%0A-+Beta%0A%0A*+Gamma&return_to=%2Ftodos%2F{id}");
    let (status, _) = request(
        &h,
        "POST",
        &format!("/todos/{id}/checklist/expand"),
        Some(&body),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);

    let summary = h
        .todos
        .get(h.account_id, id)
        .await
        .expect("get")
        .expect("todo");
    let titles: Vec<&str> = summary
        .checklist
        .iter()
        .map(|item| item.title.as_str())
        .collect();
    assert_eq!(titles, vec!["Alpha", "Beta", "Gamma"]);

    let (_, detail) = request(&h, "GET", &format!("/todos/{id}"), None).await;
    assert!(detail.contains("Alpha"), "{detail}");
    assert!(
        detail.contains("data-testid=\"expand-checklist-form\""),
        "{detail}"
    );
}
