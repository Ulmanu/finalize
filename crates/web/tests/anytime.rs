#![cfg(feature = "ssr")]

//! Anytime projection web-surface tests.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::areas::AreaService;
use momentum_app::auth::AuthService;
use momentum_app::commands::{CommandDispatcher, ScheduleInput};
use momentum_app::projects::ProjectService;
use momentum_app::todos::TodoService;
use momentum_domain::model::ContextRef;
use momentum_domain::time::LocalDate;
use momentum_testkit::areas::MemoryAreaReadStore;
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
    let areas = Arc::new(AreaService::new(
        dispatcher.clone(),
        Arc::new(MemoryAreaReadStore::new(database.clone())),
        clock.clone(),
    ));
    let projects = Arc::new(ProjectService::new(
        dispatcher,
        Arc::new(MemoryProjectReadStore::new(database)),
        clock,
    ));

    let registration = auth
        .register("anytime@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("anytime@example.com", "correct horse battery")
        .await
        .expect("login");

    let state = AppState::new(
        server::leptos_options(),
        None,
        Arc::new(CookieSessionResolver::new(auth.clone())),
        Some(Arc::new(AuthContext::new(auth, false))),
        Some(todos.clone()),
        Some(areas),
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
async fn anytime_groups_actionable_items_and_excludes_inactive_ones() {
    let h = harness().await;
    let project = h
        .projects
        .create(h.account_id, "Web project", None)
        .await
        .expect("project")
        .entity_id
        .expect("id");
    h.todos
        .create(h.account_id, "Loose actionable", None)
        .await
        .expect("create");
    h.todos
        .create_in(
            h.account_id,
            "Project actionable",
            None,
            ContextRef::Project(project),
        )
        .await
        .expect("create");

    let future = h
        .todos
        .create(h.account_id, "Future item", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .schedule(
            h.account_id,
            future,
            ScheduleInput::On(LocalDate::from_ymd(2026, 6, 20).expect("date")),
        )
        .await
        .expect("schedule");
    let someday = h
        .todos
        .create(h.account_id, "Someday item", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .schedule(h.account_id, someday, ScheduleInput::Someday)
        .await
        .expect("schedule");
    let done = h
        .todos
        .create(h.account_id, "Done item", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .complete(h.account_id, done)
        .await
        .expect("complete");

    let (status, page) = request(&h, "GET", "/anytime", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(page.matches("data-testid=\"anytime-group\"").count(), 2);
    assert!(page.contains("data-context=\"inbox\""), "{page}");
    assert!(page.contains("data-context=\"project:"), "{page}");
    assert!(
        page.contains("Web project"),
        "group labels use existing titles: {page}"
    );
    assert!(page.contains("Loose actionable"), "{page}");
    assert!(page.contains("Project actionable"), "{page}");
    assert!(!page.contains("Future item"), "{page}");
    assert!(!page.contains("Someday item"), "{page}");
    assert!(!page.contains("Done item"), "{page}");
}

#[tokio::test]
async fn anytime_empty_state_when_nothing_is_actionable() {
    let h = harness().await;
    let (status, page) = request(&h, "GET", "/anytime", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(page.contains("data-testid=\"anytime-empty\""), "{page}");
}
