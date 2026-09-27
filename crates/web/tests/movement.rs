#![cfg(feature = "ssr")]

//! Move engine web-surface tests.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::projects::ProjectService;
use momentum_app::todos::TodoService;
use momentum_domain::model::ContextRef;
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
        .register("move@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("move@example.com", "correct horse battery")
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
async fn todo_move_dialog_and_heading_move_use_the_same_commands() {
    let h = harness().await;
    let source = h
        .projects
        .create(h.account_id, "Source project", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    let target = h
        .projects
        .create(h.account_id, "Target project", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    let todo = h
        .todos
        .create_in(
            h.account_id,
            "Movable todo",
            None,
            ContextRef::Project(source),
        )
        .await
        .expect("create")
        .entity_id
        .expect("id");

    let body = format!("context_kind=project&context_id={target}&return_to=%2Fprojects%2F{target}");
    let (status, _) = request(&h, "POST", &format!("/todos/{todo}/move"), Some(&body)).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (_, target_page) = request(&h, "GET", &format!("/projects/{target}"), None).await;
    assert!(target_page.contains("Movable todo"), "{target_page}");
    let (_, source_page) = request(&h, "GET", &format!("/projects/{source}"), None).await;
    assert!(!source_page.contains("Movable todo"), "{source_page}");

    let heading = h
        .projects
        .create_heading(h.account_id, source, "Phase")
        .await
        .expect("heading")
        .entity_id
        .expect("id");
    h.todos
        .create_in(
            h.account_id,
            "Headed child",
            None,
            ContextRef::Heading(heading),
        )
        .await
        .expect("create");
    let body = format!("project={target}&return_to=%2Fprojects%2F{target}");
    let (status, _) = request(
        &h,
        "POST",
        &format!("/headings/{heading}/move_project"),
        Some(&body),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (_, target_page) = request(&h, "GET", &format!("/projects/{target}"), None).await;
    assert!(target_page.contains("Phase"), "{target_page}");
    assert!(target_page.contains("Headed child"), "{target_page}");
    let (_, source_page) = request(&h, "GET", &format!("/projects/{source}"), None).await;
    assert!(!source_page.contains("Phase"), "{source_page}");
}
