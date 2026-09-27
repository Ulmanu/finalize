#![cfg(feature = "ssr")]

//! When/deadline/reminder picker web-surface tests.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::model::Schedule;
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
        .register("pickers@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("pickers@example.com", "correct horse battery")
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

async fn create(h: &Harness, title: &str) -> momentum_domain::EntityId {
    h.todos
        .create(h.account_id, title, None)
        .await
        .expect("create");
    h.todos
        .list(h.account_id, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .find(|todo| todo.title == title)
        .expect("created todo")
        .id
}

fn date(day: u32) -> LocalDate {
    LocalDate::from_ymd(2026, 6, day).expect("date")
}

#[tokio::test]
async fn editor_exposes_when_deadline_and_reminder_pickers() {
    let h = harness().await;
    create(&h, "Alpha").await;

    let (status, body) = request(&h, "GET", "/inbox", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("data-testid=\"editor-schedule-date\""),
        "{body}"
    );
    assert!(body.contains(">When date<"), "{body}");
    assert!(body.contains("data-testid=\"editor-deadline\""), "{body}");
    assert!(body.contains("Clear deadline"), "{body}");
    assert!(body.contains("data-testid=\"editor-reminder\""), "{body}");
    assert!(body.contains("name=\"timezone\""), "{body}");
    assert!(body.contains("Set reminder"), "{body}");
    assert!(body.contains("Clear reminder"), "{body}");
}

#[tokio::test]
async fn when_deadline_and_reminder_commit_independently() {
    let h = harness().await;
    let id = create(&h, "Alpha").await;

    let (status, _) = request(
        &h,
        "POST",
        &format!("/todos/{id}/schedule"),
        Some("when=date&date=2026-06-10&return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let scheduled = h.todos.get(h.account_id, id).await.unwrap().unwrap();
    assert_eq!(scheduled.schedule, Schedule::on(date(10)));

    let (status, _) = request(
        &h,
        "POST",
        &format!("/todos/{id}/deadline"),
        Some("date=2026-06-18&return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let dated = h.todos.get(h.account_id, id).await.unwrap().unwrap();
    assert_eq!(dated.deadline.unwrap().date, date(18));
    assert_eq!(dated.schedule, Schedule::on(date(10)));

    let (status, _) = request(
        &h,
        "POST",
        &format!("/todos/{id}/reminder"),
        Some("at=2026-06-10T09%3A00&timezone=Europe%2FBerlin&return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let reminded = h.todos.get(h.account_id, id).await.unwrap().unwrap();
    let reminder = reminded.reminder.expect("reminder");
    assert_eq!(
        reminder.at,
        Utc.with_ymd_and_hms(2026, 6, 10, 7, 0, 0).unwrap()
    );
    assert_eq!(reminder.timezone, "Europe/Berlin");

    let (_, body) = request(&h, "GET", "/inbox", None).await;
    assert!(body.contains("data-testid=\"reminder-badge\""), "{body}");
    assert!(body.contains("2026-06-10 07:00 UTC"), "{body}");

    let (status, _) = request(
        &h,
        "POST",
        &format!("/todos/{id}/reminder"),
        Some("at=&return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert!(h
        .todos
        .get(h.account_id, id)
        .await
        .unwrap()
        .unwrap()
        .reminder
        .is_none());
}

#[tokio::test]
async fn natural_date_expression_schedules_and_rejects_unknown_input() {
    let h = harness().await;
    let id = create(&h, "Alpha").await;

    let (_, body) = request(&h, "GET", "/inbox", None).await;
    assert!(
        body.contains("data-testid=\"editor-natural-date\""),
        "{body}"
    );
    assert!(body.contains("Parse date"), "{body}");

    let (status, _) = request(
        &h,
        "POST",
        &format!("/todos/{id}/schedule"),
        Some("when=natural&text=tomorrow&return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(
        h.todos
            .get(h.account_id, id)
            .await
            .unwrap()
            .unwrap()
            .schedule,
        Schedule::on(h.todos.local_today().next_day())
    );

    let (status, _) = request(
        &h,
        "POST",
        &format!("/todos/{id}/schedule"),
        Some("when=natural&text=jun+20&return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(
        h.todos
            .get(h.account_id, id)
            .await
            .unwrap()
            .unwrap()
            .schedule,
        Schedule::on(date(20))
    );

    let (status, _) = request(
        &h,
        "POST",
        &format!("/todos/{id}/schedule"),
        Some("when=natural&text=whenever&return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(
        h.todos
            .get(h.account_id, id)
            .await
            .unwrap()
            .unwrap()
            .schedule,
        Schedule::on(date(20))
    );
}

#[tokio::test]
async fn reminder_requires_schedule_and_valid_inputs() {
    let h = harness().await;
    let unscheduled = create(&h, "Unscheduled").await;

    let (status, _) = request(
        &h,
        "POST",
        &format!("/todos/{unscheduled}/reminder"),
        Some("at=2026-06-20T09%3A00&timezone=UTC&return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert!(h
        .todos
        .get(h.account_id, unscheduled)
        .await
        .unwrap()
        .unwrap()
        .reminder
        .is_none());

    let scheduled = create(&h, "Scheduled").await;
    request(
        &h,
        "POST",
        &format!("/todos/{scheduled}/schedule"),
        Some("when=date&date=2026-06-20&return_to=%2Finbox"),
    )
    .await;

    let (status, _) = request(
        &h,
        "POST",
        &format!("/todos/{scheduled}/reminder"),
        Some("at=2026-06-20T09%3A00&timezone=Mars%2FOlympus&return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert!(h
        .todos
        .get(h.account_id, scheduled)
        .await
        .unwrap()
        .unwrap()
        .reminder
        .is_none());

    let (status, _) = request(
        &h,
        "POST",
        &format!("/todos/{scheduled}/reminder"),
        Some("at=2026-06-20T09%3A00&timezone=UTC&return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert!(h
        .todos
        .get(h.account_id, scheduled)
        .await
        .unwrap()
        .unwrap()
        .reminder
        .is_some());
}
