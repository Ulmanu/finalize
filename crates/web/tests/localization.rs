#![cfg(feature = "ssr")]

//! Localization: locale/timezone/hour preferences and their runtime integration.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::{CommandDispatcher, ReminderInput, ScheduleInput};
use momentum_app::preferences::{PreferenceScope, PreferenceStore};
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::model::Schedule;
use momentum_domain::time::LocalDate;
use momentum_domain::AccountId;
use momentum_testkit::auth::{
    DeterministicHasher, DeterministicTokenGenerator, MemoryAuthStore, TestClock,
};
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::preferences::MemoryPreferenceStore;
use momentum_testkit::todos::MemoryTodoReadStore;
use momentum_web::auth_routes::AuthContext;
use momentum_web::server::{self, AppState};
use momentum_web::session::CookieSessionResolver;
use tower::ServiceExt;

struct Harness {
    router: axum::Router,
    store: Arc<MemoryPreferenceStore>,
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
        .register("locale@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("locale@example.com", "correct horse battery")
        .await
        .expect("login");

    let preferences = Arc::new(MemoryPreferenceStore::new());
    let store_handle: Arc<dyn PreferenceStore> = preferences.clone();
    let state = AppState::new(
        server::leptos_options(),
        None,
        Arc::new(CookieSessionResolver::new(auth.clone())),
        Some(Arc::new(AuthContext::new(auth, false))),
        Some(todos.clone()),
        None,
        None,
    )
    .with_preferences(store_handle);

    Harness {
        router: server::router_with_state(state),
        store: preferences,
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
        .expect("todo")
        .id
}

fn date(year: i32, month: u32, day: u32) -> LocalDate {
    LocalDate::from_ymd(year, month, day).expect("date")
}

#[tokio::test]
async fn settings_lists_locale_timezone_and_hour_preferences() {
    let h = harness().await;
    let (status, _, body) = request(&h, "GET", "/settings", None).await;
    assert_eq!(status, StatusCode::OK);
    for key in [
        "general.locale",
        "general.hour_format",
        "general.timezone",
        "general.parser_locale",
        "general.week_start",
    ] {
        assert!(
            body.contains(&format!(
                "data-preference-key=\"{key}\" data-scope=\"account\""
            )),
            "missing {key}: {body}"
        );
    }
    assert!(body.contains("name=\"value\""), "{body}");

    let (status, location, _) = request(
        &h,
        "POST",
        "/settings/preferences",
        Some("key=general.timezone&value=Europe%2FBerlin"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/settings"));
    let stored = h.store.load_all(h.account_id).await.expect("load");
    assert!(stored.contains(&("general.timezone".to_string(), "Europe/Berlin".to_string())));

    let (status, location, _) = request(
        &h,
        "POST",
        "/settings/preferences",
        Some("key=general.timezone&value=Mars%2FOlympus"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/settings?error=preference"));
}

#[tokio::test]
async fn parser_locale_preference_changes_natural_date_reading() {
    let h = harness().await;
    h.store
        .save(
            h.account_id,
            "general.parser_locale",
            PreferenceScope::Account,
            "en-GB",
        )
        .await
        .expect("save");
    let todo = create(&h, "Locale task").await;

    // Day-first locale: 05/06 reads as 5 June; since that is before the June 10 base it rolls.
    let (status, _, _) = request(
        &h,
        "POST",
        &format!("/todos/{todo}/schedule"),
        Some("when=natural&text=05%2F06&return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(
        h.todos
            .get(h.account_id, todo)
            .await
            .unwrap()
            .unwrap()
            .schedule,
        Schedule::on(date(2027, 6, 5))
    );
}

#[tokio::test]
async fn timezone_preference_is_the_reminder_default() {
    let h = harness().await;
    h.store
        .save(
            h.account_id,
            "general.timezone",
            PreferenceScope::Account,
            "Europe/Berlin",
        )
        .await
        .expect("save");
    let todo = create(&h, "Reminder task").await;
    h.todos
        .schedule(h.account_id, todo, ScheduleInput::On(date(2026, 6, 10)))
        .await
        .expect("schedule");
    h.todos
        .set_reminder(
            h.account_id,
            todo,
            Some(ReminderInput {
                at: Utc.with_ymd_and_hms(2026, 6, 10, 7, 0, 0).unwrap(),
                timezone: "UTC".to_string(),
            }),
        )
        .await
        .expect("reminder");

    // Clearing then re-setting through the route with an empty timezone uses the preference.
    let (status, _, _) = request(
        &h,
        "POST",
        &format!("/todos/{todo}/reminder"),
        Some("at=&return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (status, _, _) = request(
        &h,
        "POST",
        &format!("/todos/{todo}/reminder"),
        Some("at=2026-06-10T09%3A00&timezone=&return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let reminder = h
        .todos
        .get(h.account_id, todo)
        .await
        .unwrap()
        .unwrap()
        .reminder
        .expect("reminder");
    assert_eq!(reminder.timezone, "Europe/Berlin");
    assert_eq!(
        reminder.at,
        Utc.with_ymd_and_hms(2026, 6, 10, 7, 0, 0).unwrap()
    );
}
