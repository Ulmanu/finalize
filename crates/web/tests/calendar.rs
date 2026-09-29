#![cfg(feature = "ssr")]

//! Calendar projection web-surface tests: read-only rows and degraded banners.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{NaiveTime, TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::calendar::{
    CalendarError, CalendarEvent, CalendarService, FlakyCalendarProvider,
};
use momentum_app::commands::{CommandDispatcher, ScheduleInput};
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::time::LocalDate;
use momentum_domain::AccountId;
use momentum_testkit::auth::{
    DeterministicHasher, DeterministicTokenGenerator, MemoryAuthStore, TestClock,
};
use momentum_testkit::calendar::FakeCalendarProvider;
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::todos::MemoryTodoReadStore;
use momentum_web::auth_routes::AuthContext;
use momentum_web::server::{self, AppState};
use momentum_web::session::CookieSessionResolver;
use tower::ServiceExt;

struct Harness {
    router: axum::Router,
    todos: Arc<TodoService>,
    flaky: Arc<FlakyCalendarProvider>,
    account_id: AccountId,
    cookie: String,
}

fn date(day: u32) -> LocalDate {
    LocalDate::from_ymd(2026, 6, day).expect("date")
}

fn event(id: &str, title: &str, day: u32) -> CalendarEvent {
    CalendarEvent {
        id: id.to_string(),
        title: title.to_string(),
        starts_on: date(day),
        starts_at: NaiveTime::from_hms_opt(9, 0, 0),
        ends_on: date(day),
        calendar_name: "Work".to_string(),
    }
}

async fn harness(events: Vec<CalendarEvent>) -> Harness {
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
        .register("calendar@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("calendar@example.com", "correct horse battery")
        .await
        .expect("login");

    let flaky = Arc::new(FlakyCalendarProvider::new(Arc::new(
        FakeCalendarProvider::new(events),
    )));
    let state = AppState::new(
        server::leptos_options(),
        None,
        Arc::new(CookieSessionResolver::new(auth.clone())),
        Some(Arc::new(AuthContext::new(auth, false))),
        Some(todos.clone()),
        None,
        None,
    )
    .with_calendar(Arc::new(CalendarService::new(Some(flaky.clone()))));

    Harness {
        router: server::router_with_state(state),
        todos,
        flaky,
        account_id: registration.account_id,
        cookie: format!("momentum_session={}", login.token),
    }
}

async fn request(h: &Harness, uri: &str) -> (StatusCode, String) {
    let request = Request::builder()
        .method("GET")
        .uri(uri)
        .header(header::COOKIE, &h.cookie)
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
async fn today_shows_read_only_calendar_rows() {
    let h = harness(vec![event("a", "Standup", 10)]).await;
    let (status, body) = request(&h, "/today").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("data-testid=\"calendar-events\""), "{body}");
    assert!(body.contains("data-testid=\"calendar-row\""), "{body}");
    assert!(body.contains("data-external=\"true\""), "{body}");
    assert!(body.contains("Standup"), "{body}");
    assert!(body.contains("task-row-readonly"), "{body}");
    assert!(
        !body.contains("data-testid=\"calendar-degraded\""),
        "{body}"
    );
}

#[tokio::test]
async fn upcoming_shows_calendar_rows_and_degrades_after_failure() {
    let h = harness(vec![event("a", "Offsite", 20)]).await;
    let (status, body) = request(&h, "/upcoming").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("data-testid=\"upcoming-calendar\""), "{body}");
    assert!(body.contains("Offsite"), "{body}");

    h.flaky.fail_with(CalendarError::AuthExpired);
    let (status, body) = request(&h, "/upcoming").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("data-testid=\"calendar-degraded\""), "{body}");
    assert!(body.contains("authorization expired"), "{body}");
    assert!(body.contains("Offsite"), "{body}");
}

#[tokio::test]
async fn calendar_failure_never_blocks_task_data() {
    let h = harness(Vec::new()).await;
    h.flaky.fail_with(CalendarError::Network);
    h.todos
        .create(h.account_id, "Task still here", None)
        .await
        .expect("create");
    let todo = h
        .todos
        .list(h.account_id, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .next()
        .expect("todo")
        .id;
    h.todos
        .schedule(h.account_id, todo, ScheduleInput::On(date(10)))
        .await
        .expect("schedule");

    let (status, body) = request(&h, "/today").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Task still here"), "{body}");
    assert!(body.contains("data-testid=\"calendar-degraded\""), "{body}");
    assert!(body.contains("unreachable"), "{body}");
}
