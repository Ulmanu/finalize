#![cfg(feature = "ssr")]

//! Upcoming agenda web-surface tests.

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
        .register("upcoming@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("upcoming@example.com", "correct horse battery")
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

fn date(day: u32) -> LocalDate {
    LocalDate::from_ymd(2026, 6, day).expect("date")
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
async fn upcoming_renders_date_groups_and_moves_items_between_days() {
    let h = harness().await;
    let first = scheduled(&h, "Day fifteen", ScheduleInput::On(date(15))).await;
    scheduled(&h, "Day fifteen again", ScheduleInput::On(date(15))).await;
    scheduled(&h, "Day twenty", ScheduleInput::On(date(20))).await;

    let (status, page) = request(&h, "GET", "/upcoming", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(page.matches("data-testid=\"upcoming-day\"").count(), 2);
    assert!(page.contains("data-date=\"2026-06-15\""), "{page}");
    assert!(page.contains("data-date=\"2026-06-20\""), "{page}");
    assert!(page.contains("Monday, 15 June 2026"), "{page}");
    assert!(page.contains("aria-label=\"Move to date\""), "{page}");

    let body = "when=date&date=2026-06-25&return_to=%2Fupcoming";
    let (status, _) = request(&h, "POST", &format!("/todos/{first}/schedule"), Some(body)).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (_, page) = request(&h, "GET", "/upcoming", None).await;
    assert!(page.contains("data-date=\"2026-06-25\""), "{page}");
    let day_15 = page.find("data-date=\"2026-06-15\"").expect("day 15");
    let day_25 = page.find("data-date=\"2026-06-25\"").expect("day 25");
    assert!(day_15 < day_25, "groups stay in date order: {page}");
    let day_15_section = &page[day_15..day_25];
    assert!(
        !day_15_section.contains("Day fifteen<"),
        "moved item left the earlier day: {day_15_section}"
    );

    let body = "when=date&date=2026-06-10&return_to=%2Fupcoming";
    let (status, _) = request(&h, "POST", &format!("/todos/{first}/schedule"), Some(body)).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (_, page) = request(&h, "GET", "/upcoming", None).await;
    assert!(
        !page.contains("Day fifteen<"),
        "item scheduled for today leaves Upcoming: {page}"
    );
    let (_, today) = request(&h, "GET", "/today", None).await;
    assert!(today.contains("Day fifteen"), "{today}");
}

#[tokio::test]
async fn upcoming_empty_state_and_no_calendar_rows_without_integration() {
    let h = harness().await;
    let (_, page) = request(&h, "GET", "/upcoming", None).await;
    assert!(page.contains("data-testid=\"upcoming-empty\""), "{page}");
    assert!(
        !page.contains("data-testid=\"upcoming-calendar\""),
        "{page}"
    );
}
