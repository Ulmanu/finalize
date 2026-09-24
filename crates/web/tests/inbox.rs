#![cfg(feature = "ssr")]

//! Inbox tests: badge, manual ordering, and processing controls.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::todos::{QuickSchedule, TodoService};
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
    let service = Arc::new(TodoService::new(dispatcher, reader, clock));

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
        cookie: format!("momentum_session={}", login.token),
    }
}

async fn request(
    h: &Harness,
    method: &str,
    uri: &str,
    body: Option<&str>,
) -> (StatusCode, axum::http::HeaderMap, String) {
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

async fn inbox_titles(h: &Harness) -> Vec<String> {
    h.service
        .list(h.account_id, momentum_app::todos::TodoScope::Inbox)
        .await
        .expect("list")
        .into_iter()
        .map(|todo| todo.title)
        .collect()
}

#[tokio::test]
async fn badge_tracks_open_inbox_count() {
    let h = harness().await;
    let empty = request(&h, "GET", "/inbox", None).await.2;
    assert!(empty.contains("data-testid=\"inbox-badge\""), "{empty}");
    assert!(empty.contains(">0</span>"), "badge starts at zero: {empty}");

    h.service
        .create(h.account_id, "One", None)
        .await
        .expect("create");
    h.service
        .create(h.account_id, "Two", None)
        .await
        .expect("create");

    let body = request(&h, "GET", "/inbox", None).await.2;
    assert!(
        body.contains("aria-label=\"2 open in Inbox\""),
        "badge must count open items: {body}"
    );
}

#[tokio::test]
async fn badge_drops_after_completion() {
    let h = harness().await;
    let created = h
        .service
        .create(h.account_id, "Finish", None)
        .await
        .expect("create");
    let id = created.entity_id.expect("id");
    request(&h, "POST", &format!("/todos/{id}/complete"), None).await;

    let body = request(&h, "GET", "/inbox", None).await.2;
    assert!(
        body.contains("aria-label=\"0 open in Inbox\""),
        "completed items leave the badge: {body}"
    );
}

#[tokio::test]
async fn reorder_buttons_change_inbox_order() {
    let h = harness().await;
    h.service
        .create(h.account_id, "First", None)
        .await
        .expect("create");
    h.service
        .create(h.account_id, "Second", None)
        .await
        .expect("create");
    let second = h
        .service
        .list(h.account_id, momentum_app::todos::TodoScope::Inbox)
        .await
        .expect("list")[1]
        .id;

    let (status, _, _) = request(
        &h,
        "POST",
        &format!("/todos/{second}/move-up"),
        Some("return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(inbox_titles(&h).await, vec!["Second", "First"]);

    request(
        &h,
        "POST",
        &format!("/todos/{second}/move-down"),
        Some("return_to=%2Finbox"),
    )
    .await;
    assert_eq!(inbox_titles(&h).await, vec!["First", "Second"]);
}

#[tokio::test]
async fn reorder_at_the_boundary_is_a_no_op() {
    let h = harness().await;
    h.service
        .create(h.account_id, "Only", None)
        .await
        .expect("create");
    let id = h
        .service
        .list(h.account_id, momentum_app::todos::TodoScope::Inbox)
        .await
        .expect("list")[0]
        .id;

    request(
        &h,
        "POST",
        &format!("/todos/{id}/move-up"),
        Some("return_to=%2Finbox"),
    )
    .await;
    assert_eq!(inbox_titles(&h).await, vec!["Only"]);
}

#[tokio::test]
async fn scheduling_from_the_list_removes_the_item_from_inbox() {
    let h = harness().await;
    h.service
        .create(h.account_id, "Later", None)
        .await
        .expect("create");
    let id = h
        .service
        .list(h.account_id, momentum_app::todos::TodoScope::Inbox)
        .await
        .expect("list")[0]
        .id;

    let (status, _, _) = request(
        &h,
        "POST",
        &format!("/todos/{id}/schedule"),
        Some("when=someday&return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert!(inbox_titles(&h).await.is_empty(), "someday leaves Inbox");

    let stored = h
        .service
        .get(h.account_id, id)
        .await
        .expect("get")
        .expect("todo");
    assert!(stored.schedule.is_someday());
}

#[tokio::test]
async fn quick_schedule_tomorrow_uses_the_next_local_date() {
    let h = harness().await;
    let created = h
        .service
        .create(h.account_id, "Tomorrow", None)
        .await
        .expect("create");
    let id = created.entity_id.expect("id");
    let today = momentum_domain::time::LocalDate::from_ymd(2026, 6, 1).expect("date");

    h.service
        .schedule_quick(h.account_id, id, QuickSchedule::Tomorrow, today)
        .await
        .expect("schedule");
    let stored = h
        .service
        .get(h.account_id, id)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(
        stored.schedule.date(),
        Some(momentum_domain::time::LocalDate::from_ymd(2026, 6, 2).expect("date"))
    );
}

#[tokio::test]
async fn move_assigns_a_project_context() {
    let h = harness().await;
    h.service
        .create(h.account_id, "Assign me", None)
        .await
        .expect("create");
    let id = h
        .service
        .list(h.account_id, momentum_app::todos::TodoScope::Inbox)
        .await
        .expect("list")[0]
        .id;
    let project = momentum_domain::EntityId::new();

    let body = format!("context_kind=project&context_id={project}&return_to=%2Finbox");
    let (status, _, _) = request(&h, "POST", &format!("/todos/{id}/move"), Some(&body)).await;
    assert_eq!(status, StatusCode::SEE_OTHER);

    assert!(
        inbox_titles(&h).await.is_empty(),
        "assigned items leave Inbox"
    );
    let stored = h
        .service
        .get(h.account_id, id)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(
        stored.context,
        momentum_domain::model::ContextRef::Project(project)
    );
}
