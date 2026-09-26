#![cfg(feature = "ssr")]

//! Progress indicator web-surface tests.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::{CommandDispatcher, ScheduleInput};
use momentum_app::projects::ProjectService;
use momentum_app::todos::TodoService;
use momentum_domain::model::ContextRef;
use momentum_domain::time::LocalDate;
use momentum_testkit::auth::{
    DeterministicHasher, DeterministicTokenGenerator, MemoryAuthStore, TestClock,
};
use momentum_testkit::commands::MemoryDatabase;
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
        dispatcher,
        Arc::new(MemoryProjectReadStore::new(database)),
        clock,
    ));

    let registration = auth
        .register("progress@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("progress@example.com", "correct horse battery")
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
    );

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
async fn project_pages_render_derived_progress() {
    let h = harness().await;
    let project = h
        .projects
        .create(h.account_id, "Measured project", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    let first = h
        .todos
        .create_in(
            h.account_id,
            "First child",
            None,
            ContextRef::Project(project),
        )
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .create_in(
            h.account_id,
            "Second child",
            None,
            ContextRef::Project(project),
        )
        .await
        .expect("create");
    h.todos
        .complete(h.account_id, first)
        .await
        .expect("complete");

    let (_, detail) = request(&h, "GET", &format!("/projects/{project}"), None).await;
    assert!(
        detail.contains("data-testid=\"project-progress\""),
        "{detail}"
    );
    assert!(detail.contains("50%"), "{detail}");
    assert!(
        detail.contains("aria-label=\"1 of 2 completed\""),
        "{detail}"
    );

    let (_, list) = request(&h, "GET", "/projects", None).await;
    assert!(
        list.contains("data-testid=\"project-progress-value\""),
        "{list}"
    );

    let empty = h
        .projects
        .create(h.account_id, "Empty project", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    let (_, detail) = request(&h, "GET", &format!("/projects/{empty}"), None).await;
    assert!(
        detail.contains("data-testid=\"project-progress-neutral\""),
        "{detail}"
    );
}

#[tokio::test]
async fn today_page_renders_and_updates_progress() {
    let h = harness().await;
    let mut ids = Vec::new();
    for title in ["One", "Two"] {
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
        ids.push(id);
    }

    let (_, page) = request(&h, "GET", "/today", None).await;
    assert!(page.contains("data-testid=\"today-progress\""), "{page}");
    assert!(page.contains("0 of 2 completed"), "{page}");

    h.todos
        .complete(h.account_id, ids[0])
        .await
        .expect("complete");
    let (_, page) = request(&h, "GET", "/today", None).await;
    assert!(page.contains("1 of 2 completed (50%)"), "{page}");

    h.todos
        .complete(h.account_id, ids[1])
        .await
        .expect("complete");
    let (_, page) = request(&h, "GET", "/today", None).await;
    assert!(page.contains("2 of 2 completed (100%)"), "{page}");
}

#[tokio::test]
async fn neutral_today_progress_renders_explicit_state() {
    let h = harness().await;
    let (_, page) = request(&h, "GET", "/today", None).await;
    assert!(
        page.contains("data-testid=\"today-progress-neutral\""),
        "{page}"
    );
}
