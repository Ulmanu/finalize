#![cfg(feature = "ssr")]

//! Today projection web-surface tests.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::{CommandDispatcher, ScheduleInput};
use momentum_app::todos::TodoService;
use momentum_domain::time::LocalDate;
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
        .register("today@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("today@example.com", "correct horse battery")
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

fn today() -> LocalDate {
    LocalDate::from_ymd(2026, 6, 10).expect("date")
}

async fn scheduled(h: &Harness, title: &str, schedule: ScheduleInput) -> momentum_domain::EntityId {
    let id = h
        .todos
        .create(h.account_id, title, None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .schedule(h.account_id, id, schedule)
        .await
        .expect("schedule");
    id
}

#[tokio::test]
async fn today_renders_sections_and_manual_order_without_touching_context() {
    let h = harness().await;
    let first = scheduled(&h, "Day first", ScheduleInput::On(today())).await;
    scheduled(&h, "Day second", ScheduleInput::On(today())).await;
    scheduled(&h, "Evening item", ScheduleInput::ThisEvening(today())).await;

    let (status, page) = request(&h, "GET", "/today", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(page.contains("data-testid=\"today-day-list\""), "{page}");
    assert!(
        page.contains("data-testid=\"today-evening-list\""),
        "{page}"
    );
    assert!(page.contains("Evening item"), "{page}");
    let first_at = page.find("Day first").expect("first");
    let second_at = page.find("Day second").expect("second");
    assert!(first_at < second_at, "creation order initially: {page}");

    let body = "return_to=%2Ftoday";
    let (status, _) = request(
        &h,
        "POST",
        &format!("/todos/{first}/today-down"),
        Some(body),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (_, page) = request(&h, "GET", "/today", None).await;
    let first_at = page.find("Day first").expect("first");
    let second_at = page.find("Day second").expect("second");
    assert!(second_at < first_at, "manual Today order applied: {page}");
    assert!(page.contains("Evening item"), "{page}");

    let split = page.find("This Evening").expect("heading");
    assert!(
        page[split..].contains("Evening item"),
        "evening item renders in the evening section: {page}"
    );

    let inbox = request(&h, "GET", "/inbox", None).await.1;
    let first_at = inbox.find("Day first").expect("first");
    let second_at = inbox.find("Day second").expect("second");
    assert!(
        first_at < second_at,
        "context order remains creation order: {inbox}"
    );
}
