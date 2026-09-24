#![cfg(feature = "ssr")]

//! End-to-end to-do CRUD tests over the in-memory command/read adapters.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::{CommandDispatcher, ScheduleInput};
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
    service: Arc<TodoService>,
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
    let reader = Arc::new(MemoryTodoReadStore::new(database));
    let service = Arc::new(TodoService::new(dispatcher.clone(), reader, clock));

    let registration = auth
        .register("person@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("person@example.com", "correct horse battery")
        .await
        .expect("login");
    let cookie = format!("momentum_session={}", login.token);

    let resolver = Arc::new(CookieSessionResolver::new(auth.clone()));
    let state = AppState::new(
        server::leptos_options(),
        None,
        resolver,
        Some(Arc::new(AuthContext::new(auth, false))),
        Some(service.clone()),
    );

    Harness {
        router: server::router_with_state(state),
        service,
        account_id: registration.account_id,
        cookie,
    }
}

async fn request(
    h: &Harness,
    method: &str,
    uri: &str,
    body: Option<&str>,
    with_cookie: bool,
) -> (StatusCode, axum::http::HeaderMap, String) {
    let mut builder = Request::builder().method(method).uri(uri);
    if body.is_some() {
        builder = builder.header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");
    }
    if with_cookie {
        builder = builder.header(header::COOKIE, &h.cookie);
    }
    let request = builder
        .body(body.map_or_else(Body::empty, |body| Body::from(body.to_string())))
        .expect("request");
    let response = h.router.clone().oneshot(request).await.expect("response");
    let status = response.status();
    let headers = response.headers().clone();
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    (
        status,
        headers,
        String::from_utf8(bytes.to_vec()).expect("utf-8"),
    )
}

async fn get(h: &Harness, uri: &str) -> String {
    let (status, _, body) = request(h, "GET", uri, None, true).await;
    assert_eq!(status, StatusCode::OK, "GET {uri}");
    body
}

#[tokio::test]
async fn create_todo_appears_in_inbox() {
    let h = harness().await;
    let (status, headers, _) = request(
        &h,
        "POST",
        "/todos",
        Some("title=Write+the+report&return_to=%2Finbox"),
        true,
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(
        headers.get(header::LOCATION).and_then(|v| v.to_str().ok()),
        Some("/inbox")
    );

    let body = get(&h, "/inbox").await;
    assert!(body.contains("data-testid=\"task-list\""), "{body}");
    assert!(body.contains("Write the report"), "{body}");
    assert!(body.contains("data-testid=\"add-todo-form\""), "{body}");
}

#[tokio::test]
async fn create_requires_authentication() {
    let h = harness().await;
    let (status, headers, _) = request(&h, "POST", "/todos", Some("title=Nope"), false).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(
        headers.get(header::LOCATION).and_then(|v| v.to_str().ok()),
        Some("/login")
    );
}

#[tokio::test]
async fn completing_a_todo_moves_it_out_of_inbox() {
    let h = harness().await;
    h.service
        .create(h.account_id, "Finish me", None)
        .await
        .expect("create");
    let id = h
        .service
        .list(h.account_id, momentum_app::todos::TodoScope::Inbox)
        .await
        .expect("list")[0]
        .id;

    let (status, _, _) = request(&h, "POST", &format!("/todos/{id}/complete"), None, true).await;
    assert_eq!(status, StatusCode::SEE_OTHER);

    let inbox = get(&h, "/inbox").await;
    assert!(!inbox.contains("Finish me"), "completed items leave Inbox");

    let editor = get(&h, &format!("/todos/{id}")).await;
    assert!(editor.contains("data-testid=\"todo-editor\""), "{editor}");
    assert!(editor.contains("Status: completed"), "{editor}");
}

#[tokio::test]
async fn delete_and_restore_round_trip() {
    let h = harness().await;
    h.service
        .create(h.account_id, "Disposable", None)
        .await
        .expect("create");
    let id = h
        .service
        .list(h.account_id, momentum_app::todos::TodoScope::Inbox)
        .await
        .expect("list")[0]
        .id;

    request(&h, "POST", &format!("/todos/{id}/delete"), None, true).await;
    let inbox = get(&h, "/inbox").await;
    assert!(!inbox.contains("Disposable"), "deleted items leave Inbox");

    let editor = get(&h, &format!("/todos/{id}")).await;
    assert!(editor.contains("Status: deleted"), "{editor}");
    assert!(editor.contains("Restore"), "deleted items offer restore");

    request(&h, "POST", &format!("/todos/{id}/restore"), None, true).await;
    let inbox = get(&h, "/inbox").await;
    assert!(
        inbox.contains("Disposable"),
        "restored items return to Inbox"
    );
}

#[tokio::test]
async fn duplicate_creates_a_second_row_with_the_same_title() {
    let h = harness().await;
    h.service
        .create(h.account_id, "Copy me", None)
        .await
        .expect("create");
    let id = h
        .service
        .list(h.account_id, momentum_app::todos::TodoScope::Inbox)
        .await
        .expect("list")[0]
        .id;

    request(&h, "POST", &format!("/todos/{id}/duplicate"), None, true).await;
    let inbox = get(&h, "/inbox").await;
    assert!(inbox.contains("Copy me"), "title must render");
    assert_eq!(
        inbox.matches("data-testid=\"task-row\"").count(),
        2,
        "duplicate must create a second row"
    );
}

#[tokio::test]
async fn editor_updates_title_and_notes_and_preserves_metadata() {
    let h = harness().await;
    let created = h
        .service
        .create(h.account_id, "Original", None)
        .await
        .expect("create");
    let id = created.entity_id.expect("id");
    let date = momentum_domain::time::LocalDate::from_ymd(2026, 6, 10).expect("date");
    h.service
        .schedule(h.account_id, id, ScheduleInput::On(date))
        .await
        .expect("schedule");

    let (status, _, _) = request(
        &h,
        "POST",
        &format!("/todos/{id}"),
        Some("title=Updated+title&notes=Some+notes"),
        true,
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);

    let editor = get(&h, &format!("/todos/{id}")).await;
    assert!(editor.contains("Updated title"), "{editor}");
    assert!(editor.contains("Some notes"), "{editor}");
    assert!(
        editor.contains("When: 2026-06-10"),
        "schedule must be preserved: {editor}"
    );
}

#[tokio::test]
async fn reopen_returns_the_todo_to_the_open_state() {
    let h = harness().await;
    h.service
        .create(h.account_id, "Reopen me", None)
        .await
        .expect("create");
    let id = h
        .service
        .list(h.account_id, momentum_app::todos::TodoScope::Inbox)
        .await
        .expect("list")[0]
        .id;

    request(&h, "POST", &format!("/todos/{id}/complete"), None, true).await;
    request(&h, "POST", &format!("/todos/{id}/reopen"), None, true).await;
    let editor = get(&h, &format!("/todos/{id}")).await;
    assert!(editor.contains("Status: open"), "{editor}");
}

#[tokio::test]
async fn unknown_todo_id_renders_not_found() {
    let h = harness().await;
    let missing = momentum_domain::EntityId::new();
    let editor = get(&h, &format!("/todos/{missing}")).await;
    assert!(
        editor.contains("data-testid=\"todo-not-found\""),
        "{editor}"
    );
}
