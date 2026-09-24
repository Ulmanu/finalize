#![cfg(feature = "ssr")]

//! Project lifecycle tests over the in-memory adapters.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::{CommandDispatcher, ProjectDeletionPolicy};
use momentum_app::projects::ProjectService;
use momentum_app::todos::TodoService;
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
    projects: Arc<ProjectService>,
    todos: Arc<TodoService>,
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
    let todos = Arc::new(TodoService::new(
        dispatcher.clone(),
        Arc::new(MemoryTodoReadStore::new(database.clone())),
        clock.clone(),
    ));
    let projects = Arc::new(ProjectService::new(
        dispatcher,
        Arc::new(MemoryProjectReadStore::new(database.clone())),
        clock,
    ));

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
        Some(todos.clone()),
        None,
        Some(projects.clone()),
    );

    Harness {
        router: server::router_with_state(state),
        projects,
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

async fn create_project(h: &Harness, title: &str) -> momentum_domain::EntityId {
    request(h, "POST", "/projects", Some(&format!("title={title}"))).await;
    h.projects.list(h.account_id).await.expect("list")[0].id
}

#[tokio::test]
async fn create_list_and_rename_project() {
    let h = harness().await;
    let id = create_project(&h, "Website+refresh").await;

    let list = request(&h, "GET", "/projects", None).await.2;
    assert!(list.contains("data-testid=\"project-list\""), "{list}");
    assert!(list.contains("Website refresh"), "{list}");
    assert!(list.contains("0 open · 0 done"), "{list}");

    request(
        &h,
        "POST",
        &format!("/projects/{id}/rename"),
        Some("title=Launch+plan"),
    )
    .await;
    let summary = h
        .projects
        .get(h.account_id, id)
        .await
        .expect("get")
        .expect("project");
    assert_eq!(summary.title, "Launch plan");
}

#[tokio::test]
async fn notes_and_schedule_are_project_level() {
    let h = harness().await;
    let id = create_project(&h, "Notes+project").await;
    let child = h
        .todos
        .create_in(
            h.account_id,
            "Child task",
            None,
            momentum_domain::model::ContextRef::Project(id),
        )
        .await
        .expect("create child")
        .entity_id
        .expect("id");

    request(
        &h,
        "POST",
        &format!("/projects/{id}/notes"),
        Some("notes=Important+context"),
    )
    .await;
    request(
        &h,
        "POST",
        &format!("/projects/{id}/schedule"),
        Some("when=date&date=2026-06-15"),
    )
    .await;

    let summary = h
        .projects
        .get(h.account_id, id)
        .await
        .expect("get")
        .expect("project");
    assert_eq!(summary.notes, "Important context");
    assert_eq!(
        summary.schedule.date(),
        Some(momentum_domain::time::LocalDate::from_ymd(2026, 6, 15).expect("date"))
    );

    let child_summary = h
        .todos
        .get(h.account_id, child)
        .await
        .expect("get")
        .expect("todo");
    assert!(
        child_summary.schedule.date().is_none(),
        "project schedule must not be copied to children"
    );
}

#[tokio::test]
async fn todo_created_in_project_appears_in_children() {
    let h = harness().await;
    let id = create_project(&h, "Membership").await;

    let body = format!(
        "title=Project+task&context_kind=project&context_id={id}&return_to=%2Fprojects%2F{id}"
    );
    let (status, _, _) = request(&h, "POST", "/todos", Some(&body)).await;
    assert_eq!(status, StatusCode::SEE_OTHER);

    let summary = h
        .projects
        .get(h.account_id, id)
        .await
        .expect("get")
        .expect("project");
    assert_eq!(summary.open_count, 1);

    let detail = request(&h, "GET", &format!("/projects/{id}"), None).await.2;
    assert!(
        detail.contains("data-testid=\"project-children\""),
        "{detail}"
    );
    assert!(detail.contains("Project task"), "{detail}");
}

#[tokio::test]
async fn completion_policies_are_explicit() {
    let h = harness().await;
    let id = create_project(&h, "Policy").await;
    h.todos
        .create_in(
            h.account_id,
            "Open child",
            None,
            momentum_domain::model::ContextRef::Project(id),
        )
        .await
        .expect("child");

    let (status, headers, _) = request(
        &h,
        "POST",
        &format!("/projects/{id}/complete"),
        Some("policy=reject"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert!(
        headers
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok())
            .is_some_and(|location| location.contains("error=open-children")),
        "reject policy must surface a conflict"
    );
    let summary = h
        .projects
        .get(h.account_id, id)
        .await
        .expect("get")
        .expect("project");
    assert_eq!(summary.status, momentum_domain::model::Status::Open);

    request(
        &h,
        "POST",
        &format!("/projects/{id}/complete"),
        Some("policy=cascade"),
    )
    .await;
    let summary = h
        .projects
        .get(h.account_id, id)
        .await
        .expect("get")
        .expect("project");
    assert_eq!(summary.status, momentum_domain::model::Status::Completed);
    assert_eq!(summary.completed_count, 1);
    assert_eq!(summary.open_count, 0);
}

#[tokio::test]
async fn delete_detach_returns_children_to_inbox() {
    let h = harness().await;
    let id = create_project(&h, "Delete me").await;
    let child = h
        .todos
        .create_in(
            h.account_id,
            "Rescued child",
            None,
            momentum_domain::model::ContextRef::Project(id),
        )
        .await
        .expect("child")
        .entity_id
        .expect("id");

    h.projects
        .delete(h.account_id, id, ProjectDeletionPolicy::DetachChildren)
        .await
        .expect("delete");

    let child_summary = h
        .todos
        .get(h.account_id, child)
        .await
        .expect("get")
        .expect("todo");
    assert!(child_summary.context.is_inbox());
    assert_eq!(child_summary.status, momentum_domain::model::Status::Open);
    assert!(h
        .projects
        .get(h.account_id, id)
        .await
        .expect("get")
        .is_none());
}

#[tokio::test]
async fn delete_trash_moves_children_to_trash() {
    let h = harness().await;
    let id = create_project(&h, "Trash children").await;
    let child = h
        .todos
        .create_in(
            h.account_id,
            "Doomed child",
            None,
            momentum_domain::model::ContextRef::Project(id),
        )
        .await
        .expect("child")
        .entity_id
        .expect("id");

    h.projects
        .delete(h.account_id, id, ProjectDeletionPolicy::TrashChildren)
        .await
        .expect("delete");

    let child_summary = h
        .todos
        .get(h.account_id, child)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(
        child_summary.status,
        momentum_domain::model::Status::Deleted
    );
}
