#![cfg(feature = "ssr")]

//! Logbook web-surface tests.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::logbook::LogbookService;
use momentum_app::projects::ProjectService;
use momentum_app::todos::TodoService;
use momentum_testkit::auth::{
    DeterministicHasher, DeterministicTokenGenerator, MemoryAuthStore, TestClock,
};
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::logbook::MemoryLoggingPolicyStore;
use momentum_testkit::projects::MemoryProjectReadStore;
use momentum_testkit::todos::MemoryTodoReadStore;
use momentum_web::auth_routes::AuthContext;
use momentum_web::server::{self, AppState};
use momentum_web::session::CookieSessionResolver;
use tower::ServiceExt;

struct Harness {
    router: axum::Router,
    todos: Arc<TodoService>,
    projects: Arc<ProjectService>,
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
        dispatcher.clone(),
        Arc::new(MemoryTodoReadStore::new(database.clone())),
        clock.clone(),
    ));
    let projects = Arc::new(ProjectService::new(
        dispatcher.clone(),
        Arc::new(MemoryProjectReadStore::new(database)),
        clock.clone(),
    ));
    let logbook = Arc::new(LogbookService::new(
        dispatcher,
        todos.clone(),
        projects.clone(),
        Arc::new(MemoryLoggingPolicyStore::new()),
        clock,
    ));

    let registration = auth
        .register("logbook@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("logbook@example.com", "correct horse battery")
        .await
        .expect("login");

    let state = AppState::new(
        server::leptos_options(),
        None,
        Arc::new(CookieSessionResolver::new(auth.clone())),
        Some(Arc::new(AuthContext::new(auth, false))),
        Some(todos.clone()),
        None,
        Some(projects.clone()),
    )
    .with_logbook(logbook);

    Harness {
        router: server::router_with_state(state),
        todos,
        projects,
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
async fn automatic_logbook_groups_history_by_date() {
    let h = harness().await;
    let done = h
        .todos
        .create(h.account_id, "Finished task", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .complete(h.account_id, done)
        .await
        .expect("complete");
    let dropped = h
        .todos
        .create(h.account_id, "Dropped task", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos.cancel(h.account_id, dropped).await.expect("cancel");
    let project = h
        .projects
        .create(h.account_id, "Shipped project", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.projects
        .complete(
            h.account_id,
            project,
            momentum_domain::model::ProjectCompletionPolicy::CompleteOpenChildren,
        )
        .await
        .expect("complete project");

    let (status, page) = request(&h, "GET", "/logbook", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(page.contains("data-testid=\"logging-policy\""), "{page}");
    assert!(page.contains("Logging policy: automatic"), "{page}");
    assert!(page.contains("data-testid=\"logbook-day\""), "{page}");
    assert!(page.contains("data-date=\"2026-06-10\""), "{page}");
    assert!(page.contains("Finished task"), "{page}");
    assert!(page.contains("Dropped task"), "{page}");
    assert!(
        page.contains("data-testid=\"logbook-project-row\""),
        "{page}"
    );
    assert!(page.contains("Shipped project"), "{page}");
    assert!(page.contains("Completed"), "{page}");
    assert!(!page.contains("data-testid=\"logbook-pending\""), "{page}");
}

#[tokio::test]
async fn manual_policy_pends_items_until_explicitly_logged() {
    let h = harness().await;
    let done = h
        .todos
        .create(h.account_id, "Manual task", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .complete(h.account_id, done)
        .await
        .expect("complete");

    let (status, _) = request(
        &h,
        "POST",
        "/logbook/policy",
        Some("policy=manual&return_to=%2Flogbook"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (_, page) = request(&h, "GET", "/logbook", None).await;
    assert!(page.contains("Logging policy: manual"), "{page}");
    assert!(page.contains("data-testid=\"logbook-pending\""), "{page}");
    assert!(page.contains("Manual task"), "{page}");
    assert!(!page.contains("data-testid=\"logbook-day\""), "{page}");

    let (status, _) = request(
        &h,
        "POST",
        &format!("/todos/{done}/log"),
        Some("return_to=%2Flogbook"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (_, page) = request(&h, "GET", "/logbook", None).await;
    assert!(!page.contains("data-testid=\"logbook-pending\""), "{page}");
    assert!(page.contains("data-testid=\"logbook-day\""), "{page}");
    assert!(page.contains("Manual task"), "{page}");
    let summary = h
        .todos
        .get(h.account_id, done)
        .await
        .expect("get")
        .expect("todo");
    assert!(summary.logged_at.is_some(), "explicit logging recorded");
    assert!(summary.completed_at.is_some(), "completion state preserved");
}
