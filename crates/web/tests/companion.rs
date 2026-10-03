#![cfg(feature = "ssr")]

//! Companion surface: compact Today, derived progress, and quick add via normal commands.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::{CommandDispatcher, ScheduleInput};
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::time::LocalDate;
use momentum_domain::AccountId;
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

    let registration = auth
        .register("companion@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("companion@example.com", "correct horse battery")
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

async fn request(
    h: &Harness,
    method: &str,
    uri: &str,
    body: Option<&str>,
) -> (StatusCode, Option<String>, String) {
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

async fn create_scheduled_today(h: &Harness, title: &str) -> momentum_domain::EntityId {
    h.todos
        .create(h.account_id, title, None)
        .await
        .expect("create");
    let id = h
        .todos
        .list(h.account_id, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .find(|todo| todo.title == title)
        .expect("todo")
        .id;
    h.todos
        .schedule(
            h.account_id,
            id,
            ScheduleInput::On(LocalDate::from_ymd(2026, 6, 10).unwrap()),
        )
        .await
        .expect("schedule");
    id
}

#[tokio::test]
async fn companion_shows_compact_today_with_shared_progress() {
    let h = harness().await;
    let first = create_scheduled_today(&h, "First").await;
    create_scheduled_today(&h, "Second").await;
    h.todos
        .complete(h.account_id, first)
        .await
        .expect("complete");

    let (status, _, body) = request(&h, "GET", "/companion", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("data-testid=\"companion\""), "{body}");
    assert!(
        body.contains("data-testid=\"companion-progress\""),
        "{body}"
    );
    assert!(body.contains("data-completed=\"1\""), "{body}");
    assert!(body.contains("data-total=\"2\""), "{body}");
    assert!(body.contains("1 of 2 done"), "{body}");
    assert!(
        body.contains("data-testid=\"companion-today-list\""),
        "{body}"
    );
    assert!(body.contains("Second"), "{body}");
    assert!(body.contains("data-testid=\"companion-add\""), "{body}");
}

#[tokio::test]
async fn companion_actions_use_normal_commands() {
    let h = harness().await;
    let todo = create_scheduled_today(&h, "Compact task").await;

    let (status, location, _) = request(
        &h,
        "POST",
        &format!("/todos/{todo}/complete"),
        Some("return_to=%2Fcompanion"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some(format!("/todos/{todo}").as_str()));
    assert!(h
        .todos
        .get(h.account_id, todo)
        .await
        .unwrap()
        .unwrap()
        .completed_at
        .is_some());

    let (status, location, _) = request(
        &h,
        "POST",
        "/todos",
        Some("title=Quick+add&return_to=%2Fcompanion"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/companion"));
    let inbox = h
        .todos
        .list(h.account_id, TodoScope::Inbox)
        .await
        .expect("inbox");
    assert!(inbox.iter().any(|todo| todo.title == "Quick add"));

    // Empty state remains compact and actionable.
    let empty = harness().await;
    let (_, _, body) = request(&empty, "GET", "/companion", None).await;
    assert!(body.contains("data-testid=\"companion-empty\""), "{body}");
}
