#![cfg(feature = "ssr")]

//! Reminder snooze web-surface tests: delivered-only controls and choice routing.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{Duration, TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::{CommandDispatcher, ReminderInput, ScheduleInput};
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::model::ReminderState;
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};
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
    now: chrono::DateTime<Utc>,
}

async fn harness() -> Harness {
    let store = Arc::new(MemoryAuthStore::new());
    let now = Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap();
    let clock = Arc::new(TestClock::new(now));
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
        .register("snooze@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("snooze@example.com", "correct horse battery")
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
        now,
    }
}

async fn raw_request(
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

async fn create_with_due_reminder(h: &Harness) -> EntityId {
    h.todos
        .create(h.account_id, "Water plants", None)
        .await
        .expect("create");
    let id = h
        .todos
        .list(h.account_id, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .next()
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
    h.todos
        .set_reminder(
            h.account_id,
            id,
            Some(ReminderInput {
                at: h.now - Duration::minutes(5),
                timezone: "UTC".to_string(),
            }),
        )
        .await
        .expect("reminder");
    id
}

#[tokio::test]
async fn snooze_controls_appear_for_delivered_reminders_and_route_choices() {
    let h = harness().await;
    let id = create_with_due_reminder(&h).await;

    let (_, _, body) = raw_request(&h, "GET", "/inbox", None).await;
    assert!(!body.contains("data-testid=\"row-snooze\""), "{body}");

    h.todos
        .mark_reminder_delivered(h.account_id, id)
        .await
        .expect("deliver");
    let (_, _, body) = raw_request(&h, "GET", "/inbox", None).await;
    assert!(body.contains("data-testid=\"row-snooze\""), "{body}");
    assert!(
        body.contains("data-testid=\"snooze-ten_minutes\""),
        "{body}"
    );
    assert!(body.contains("data-testid=\"snooze-one_hour\""), "{body}");
    assert!(body.contains("data-testid=\"snooze-tomorrow\""), "{body}");

    let (status, location, _) = raw_request(
        &h,
        "POST",
        &format!("/todos/{id}/snooze"),
        Some("option=one_hour&return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/inbox"));
    let reminder = h
        .todos
        .get(h.account_id, id)
        .await
        .unwrap()
        .unwrap()
        .reminder
        .expect("reminder");
    assert_eq!(reminder.state, ReminderState::Snoozed);
    assert_eq!(reminder.at, h.now + Duration::hours(1));

    let (status, location, _) = raw_request(
        &h,
        "POST",
        &format!("/todos/{id}/snooze"),
        Some("option=bogus&return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/inbox?error=snooze"));
    let unchanged = h
        .todos
        .get(h.account_id, id)
        .await
        .unwrap()
        .unwrap()
        .reminder
        .expect("reminder");
    assert_eq!(unchanged.at, reminder.at);
    assert_eq!(unchanged.state, ReminderState::Snoozed);
}

#[tokio::test]
async fn snoozing_without_a_reminder_reports_the_error_state() {
    let h = harness().await;
    h.todos
        .create(h.account_id, "No reminder", None)
        .await
        .expect("create");
    let id = h
        .todos
        .list(h.account_id, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .next()
        .expect("todo")
        .id;

    let (status, location, _) = raw_request(
        &h,
        "POST",
        &format!("/todos/{id}/snooze"),
        Some("option=tomorrow&return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/inbox?error=snooze"));
    assert!(h
        .todos
        .get(h.account_id, id)
        .await
        .unwrap()
        .unwrap()
        .reminder
        .is_none());
}
