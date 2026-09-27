#![cfg(feature = "ssr")]

//! Drag-and-drop drop-target web-surface tests.

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
        .register("drag@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("drag@example.com", "correct horse battery")
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

#[tokio::test]
async fn drop_position_form_reorders_and_markup_carries_drag_hooks() {
    let h = harness().await;
    let mut ids = Vec::new();
    for title in ["First", "Second", "Third"] {
        let id = h
            .todos
            .create(h.account_id, title, None)
            .await
            .expect("create")
            .entity_id
            .expect("id");
        ids.push(id);
    }

    let (status, page) = request(&h, "GET", "/inbox", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(page.contains("draggable=\"true\""), "{page}");
    assert!(page.contains("data-drag-index=\"2\""), "{page}");
    assert!(page.contains("data-drop-position=\"3\""), "{page}");
    assert!(page.contains("aria-label=\"Move to position\""), "{page}");

    let body = "position=1&return_to=%2Finbox";
    let (status, _) = request(
        &h,
        "POST",
        &format!("/todos/{}/position", ids[2]),
        Some(body),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (_, page) = request(&h, "GET", "/inbox", None).await;
    let third = page.find("Third").expect("third");
    let first = page.find("First").expect("first");
    assert!(third < first, "dropped item moves to the top: {page}");
    let stored = h
        .todos
        .get(h.account_id, ids[2])
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(stored.title, "Third");
}

#[tokio::test]
async fn today_drop_and_upcoming_day_targets_are_wired() {
    let h = harness().await;
    let mut today_ids = Vec::new();
    for title in ["Today one", "Today two"] {
        let id = h
            .todos
            .create(h.account_id, title, None)
            .await
            .expect("create")
            .entity_id
            .expect("id");
        h.todos
            .schedule(
                h.account_id,
                id,
                ScheduleInput::On(LocalDate::from_ymd(2026, 6, 10).expect("date")),
            )
            .await
            .expect("schedule");
        today_ids.push(id);
    }
    let future = h
        .todos
        .create(h.account_id, "Future item", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");

    let body = "position=1&return_to=%2Ftoday";
    let (status, _) = request(
        &h,
        "POST",
        &format!("/todos/{}/today-position", today_ids[1]),
        Some(body),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (_, page) = request(&h, "GET", "/today", None).await;
    let second = page.find("Today two").expect("second");
    let first = page.find("Today one").expect("first");
    assert!(second < first, "Today drop order applies: {page}");

    // Upcoming day sections expose drop targets for rescheduling.
    h.todos
        .schedule(
            h.account_id,
            future,
            ScheduleInput::On(LocalDate::from_ymd(2026, 6, 20).expect("date")),
        )
        .await
        .expect("schedule");
    let (_, upcoming) = request(&h, "GET", "/upcoming", None).await;
    assert!(
        upcoming.contains("data-drop-date=\"2026-06-20\""),
        "{upcoming}"
    );
    assert!(
        upcoming.contains("aria-label=\"Move to date\""),
        "{upcoming}"
    );
}
