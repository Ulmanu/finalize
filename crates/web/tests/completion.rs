#![cfg(feature = "ssr")]

//! Completion/cancellation web-surface tests.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::projects::ProjectService;
use momentum_app::todos::TodoService;
use momentum_domain::model::{ContextRef, ProjectCompletionPolicy};
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
        .register("completion@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("completion@example.com", "correct horse battery")
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

#[tokio::test]
async fn editor_cancel_then_reopen_round_trips() {
    let h = harness().await;
    let id = h
        .todos
        .create(h.account_id, "Terminal item", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");

    let (status, _, _) = request(&h, "POST", &format!("/todos/{id}/cancel"), None).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (_, _, page) = request(&h, "GET", &format!("/todos/{id}"), None).await;
    assert!(page.contains("Status: canceled"), "{page}");
    let summary = h
        .todos
        .get(h.account_id, id)
        .await
        .expect("get")
        .expect("todo");
    assert!(summary.canceled_at.is_some());

    let (status, _, _) = request(&h, "POST", &format!("/todos/{id}/reopen"), None).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let summary = h
        .todos
        .get(h.account_id, id)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(summary.status, momentum_domain::model::Status::Open);
    assert!(summary.canceled_at.is_none());
    assert!(summary.completed_at.is_none());
    let (_, _, page) = request(&h, "GET", &format!("/todos/{id}"), None).await;
    assert!(page.contains("Status: open"), "{page}");
}

#[tokio::test]
async fn project_completion_policies_are_enforced_on_the_web_surface() {
    let h = harness().await;
    let project = h
        .projects
        .create(h.account_id, "Guarded", None)
        .await
        .expect("project")
        .entity_id
        .expect("id");
    let heading = h
        .projects
        .create_heading(h.account_id, project, "Phase")
        .await
        .expect("heading")
        .entity_id
        .expect("id");
    let headed = h
        .todos
        .create_in(
            h.account_id,
            "Headed child",
            None,
            ContextRef::Heading(heading),
        )
        .await
        .expect("create")
        .entity_id
        .expect("id");

    let (status, headers, _) = request(
        &h,
        "POST",
        &format!("/projects/{project}/complete"),
        Some("policy=reject"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let location = headers
        .get(header::LOCATION)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    assert!(location.contains("error=open-children"), "{location}");
    let summary = h
        .projects
        .get(h.account_id, project)
        .await
        .expect("get")
        .expect("project");
    assert_eq!(summary.status, momentum_domain::model::Status::Open);

    let (status, _, _) = request(
        &h,
        "POST",
        &format!("/projects/{project}/complete"),
        Some("policy=cascade"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let summary = h
        .projects
        .get(h.account_id, project)
        .await
        .expect("get")
        .expect("project");
    assert_eq!(summary.status, momentum_domain::model::Status::Completed);
    assert!(summary.completed_at.is_some());
    let child = h
        .todos
        .get(h.account_id, headed)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(
        child.status,
        momentum_domain::model::Status::Completed,
        "cascade completes heading children through the command layer"
    );

    // The same policy path is available directly on the service for symmetry.
    h.projects
        .complete(
            h.account_id,
            project,
            ProjectCompletionPolicy::CompleteOpenChildren,
        )
        .await
        .expect("idempotent cascade");
}
