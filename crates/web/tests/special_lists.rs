#![cfg(feature = "ssr")]

//! Special-list web-surface tests.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::{CommandDispatcher, ScheduleInput};
use momentum_app::projects::ProjectService;
use momentum_app::recurring::RecurringService;
use momentum_app::todos::TodoService;
use momentum_domain::time::LocalDate;
use momentum_testkit::auth::{
    DeterministicHasher, DeterministicTokenGenerator, MemoryAuthStore, TestClock,
};
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::projects::MemoryProjectReadStore;
use momentum_testkit::recurring::MemoryRecurrenceReadStore;
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

async fn harness(seed_templates: bool) -> Harness {
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
    let recurrence_store = Arc::new(MemoryRecurrenceReadStore::new());
    let registration = auth
        .register("special@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("special@example.com", "correct horse battery")
        .await
        .expect("login");
    if seed_templates {
        recurrence_store.add(
            registration.account_id,
            momentum_domain::EntityId::new(),
            "Weekly review",
            Some(LocalDate::from_ymd(2026, 6, 17).expect("date")),
        );
    }
    let recurring = Arc::new(RecurringService::new(recurrence_store));
    let state = AppState::new(
        server::leptos_options(),
        None,
        Arc::new(CookieSessionResolver::new(auth.clone())),
        Some(Arc::new(AuthContext::new(auth, false))),
        Some(todos.clone()),
        None,
        Some(projects.clone()),
    )
    .with_recurring(recurring);

    Harness {
        router: server::router_with_state(state),
        todos,
        projects,
        account_id: registration.account_id,
        cookie: format!("momentum_session={}", login.token),
    }
}

async fn request(h: &Harness, method: &str, uri: &str) -> (StatusCode, String) {
    let builder = Request::builder()
        .method(method)
        .uri(uri)
        .header(header::COOKIE, &h.cookie);
    let request = builder.body(Body::empty()).expect("request");
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
async fn tomorrow_page_shows_next_day_items_only() {
    let h = harness(false).await;
    let tomorrow = h
        .todos
        .create(h.account_id, "Tomorrow task", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.todos
        .schedule(
            h.account_id,
            tomorrow,
            ScheduleInput::On(LocalDate::from_ymd(2026, 6, 11).expect("date")),
        )
        .await
        .expect("schedule");
    let future = h
        .todos
        .create(h.account_id, "Later task", None)
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

    let (status, page) = request(&h, "GET", "/tomorrow").await;
    assert_eq!(status, StatusCode::OK);
    assert!(page.contains("data-testid=\"tomorrow-list\""), "{page}");
    assert!(page.contains("Tomorrow task"), "{page}");
    assert!(!page.contains("Later task"), "{page}");
}

#[tokio::test]
async fn logged_projects_page_and_projects_link() {
    let h = harness(false).await;
    let completed = h
        .projects
        .create(h.account_id, "Finished project", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    h.projects
        .complete(
            h.account_id,
            completed,
            momentum_domain::model::ProjectCompletionPolicy::CompleteOpenChildren,
        )
        .await
        .expect("complete");

    let (status, page) = request(&h, "GET", "/projects/logged").await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        page.contains("data-testid=\"logged-projects-list\""),
        "{page}"
    );
    assert!(page.contains("Finished project"), "{page}");
    assert!(page.contains("completed"), "{page}");
    let (_, projects_page) = request(&h, "GET", "/projects").await;
    assert!(
        projects_page.contains("/projects/logged"),
        "{projects_page}"
    );
}

#[tokio::test]
async fn repeating_page_lists_templates_or_empty_state() {
    let empty = harness(false).await;
    let (status, page) = request(&empty, "GET", "/repeating").await;
    assert_eq!(status, StatusCode::OK);
    assert!(page.contains("data-testid=\"repeating-empty\""), "{page}");

    let seeded = harness(true).await;
    let (_, page) = request(&seeded, "GET", "/repeating").await;
    assert!(page.contains("data-testid=\"repeating-list\""), "{page}");
    assert!(page.contains("Weekly review"), "{page}");
    assert!(page.contains("Next: 2026-06-17"), "{page}");
}
