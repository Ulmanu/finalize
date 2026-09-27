#![cfg(feature = "ssr")]

//! Magic Plus web-surface tests: contextual creation, insertion positioning, alternate targets.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::projects::ProjectService;
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::model::{ContextRef, Schedule};
use momentum_domain::AccountId;
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
        .register("magic@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("magic@example.com", "correct horse battery")
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

async fn inbox_titles(h: &Harness) -> Vec<String> {
    h.todos
        .list(h.account_id, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .map(|todo| todo.title)
        .collect()
}

#[tokio::test]
async fn inbox_and_today_render_magic_plus_controls() {
    let h = harness().await;
    request(&h, "POST", "/todos", Some("title=Alpha&return_to=%2Finbox")).await;
    request(&h, "POST", "/todos", Some("title=Beta&return_to=%2Finbox")).await;

    let (status, body) = request(&h, "GET", "/inbox", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("data-testid=\"insert-here\""), "{body}");
    assert!(body.contains("Insert above Alpha"), "{body}");
    assert!(body.contains("name=\"position\""), "{body}");
    assert!(body.contains("value=\"1\""), "{body}");
    assert!(body.contains("value=\"2\""), "{body}");
    assert!(body.contains("Add here"), "{body}");

    let (status, body) = request(&h, "GET", "/today", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("data-testid=\"today-add-form\""), "{body}");
    assert!(body.contains("name=\"schedule_kind\""), "{body}");
}

#[tokio::test]
async fn positioned_create_inserts_between_existing_rows() {
    let h = harness().await;
    let (status, _) = request(&h, "POST", "/todos", Some("title=A&return_to=%2Finbox")).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (status, _) = request(&h, "POST", "/todos", Some("title=B&return_to=%2Finbox")).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(inbox_titles(&h).await, vec!["A", "B"]);

    let (status, _) = request(
        &h,
        "POST",
        "/todos",
        Some("title=C&position=2&return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(inbox_titles(&h).await, vec!["A", "C", "B"]);
}

#[tokio::test]
async fn alternate_target_creates_in_project_context() {
    let h = harness().await;
    h.projects
        .create(h.account_id, "Project", None)
        .await
        .expect("project");
    let project = h
        .projects
        .list(h.account_id)
        .await
        .expect("projects")
        .into_iter()
        .find(|project| project.title == "Project")
        .expect("created project")
        .id;

    let return_to = format!("%2Fprojects%2F{project}");
    let body = format!("title=X&context_kind=project&context_id={project}&return_to={return_to}");
    let (status, _) = request(&h, "POST", "/todos", Some(&body)).await;
    assert_eq!(status, StatusCode::SEE_OTHER);

    let created = h
        .todos
        .list(h.account_id, TodoScope::AllOpen)
        .await
        .expect("all open")
        .into_iter()
        .find(|todo| todo.title == "X")
        .expect("created todo");
    assert_eq!(created.context, ContextRef::Project(project));
}

#[tokio::test]
async fn today_positioned_create_is_scheduled_and_ordered_in_the_bucket() {
    let h = harness().await;
    let today = h.todos.local_today();
    let (status, _) = request(
        &h,
        "POST",
        "/todos",
        Some("title=T1&schedule_kind=today&return_to=%2Ftoday"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (status, _) = request(
        &h,
        "POST",
        "/todos",
        Some("title=T2&schedule_kind=today&position=1&position_scope=today&return_to=%2Ftoday"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);

    let scheduled = h
        .todos
        .list(h.account_id, TodoScope::Today)
        .await
        .expect("today");
    let titles: Vec<&str> = scheduled.iter().map(|todo| todo.title.as_str()).collect();
    assert_eq!(titles, vec!["T2", "T1"]);
    for todo in &scheduled {
        assert_eq!(todo.schedule, Schedule::on(today));
    }
}
