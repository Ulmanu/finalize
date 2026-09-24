#![cfg(feature = "ssr")]

//! Area lifecycle tests over the in-memory adapters.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::areas::AreaService;
use momentum_app::auth::AuthService;
use momentum_app::commands::{AreaDeletionPolicy, CommandDispatcher};
use momentum_app::todos::TodoService;
use momentum_testkit::areas::MemoryAreaReadStore;
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
    areas: Arc<AreaService>,
    todos: Arc<TodoService>,
    database: MemoryDatabase,
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
    let areas = Arc::new(AreaService::new(
        dispatcher,
        Arc::new(MemoryAreaReadStore::new(database.clone())),
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
        Some(areas.clone()),
        None,
    );

    Harness {
        router: server::router_with_state(state),
        areas,
        todos,
        database,
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
async fn create_list_and_rename_area() {
    let h = harness().await;
    let (status, headers, _) = request(&h, "POST", "/areas", Some("title=Work")).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(
        headers.get(header::LOCATION).and_then(|v| v.to_str().ok()),
        Some("/areas")
    );

    let created = h.areas.list(h.account_id).await.expect("list");
    assert_eq!(created.len(), 1);
    assert_eq!(created[0].title, "Work");
    let id = created[0].id;

    let body = request(&h, "GET", "/areas", None).await.2;
    assert!(body.contains("Work"), "{body}");
    assert!(body.contains("0 projects · 0 to-dos"), "{body}");

    let (status, _, _) = request(
        &h,
        "POST",
        &format!("/areas/{id}/rename"),
        Some("title=Work+%26+Life"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let renamed = h
        .areas
        .get(h.account_id, id)
        .await
        .expect("get")
        .expect("area");
    assert_eq!(renamed.title, "Work & Life");
}

#[tokio::test]
async fn area_counts_include_loose_todos() {
    let h = harness().await;
    let area_id = h
        .areas
        .create(h.account_id, "Home")
        .await
        .expect("create area")
        .entity_id
        .expect("id");

    let todo = h
        .todos
        .create(h.account_id, "Fix the sink", None)
        .await
        .expect("create todo")
        .entity_id
        .expect("id");
    h.todos
        .move_to(
            h.account_id,
            todo,
            momentum_domain::model::ContextRef::Area(area_id),
        )
        .await
        .expect("move");

    let summary = h
        .areas
        .get(h.account_id, area_id)
        .await
        .expect("get")
        .expect("area");
    assert_eq!(summary.todo_count, 1);
    assert_eq!(summary.project_count, 0);

    let detail = request(&h, "GET", &format!("/areas/{area_id}"), None)
        .await
        .2;
    assert!(
        detail.contains("data-testid=\"area-loose-list\""),
        "{detail}"
    );
    assert!(detail.contains("Fix the sink"), "{detail}");
}

#[tokio::test]
async fn detach_policy_returns_todos_to_inbox() {
    let h = harness().await;
    let area_id = h
        .areas
        .create(h.account_id, "Detach me")
        .await
        .expect("create")
        .entity_id
        .expect("id");
    let todo = h
        .todos
        .create(h.account_id, "Loose item", None)
        .await
        .expect("create todo")
        .entity_id
        .expect("id");
    h.todos
        .move_to(
            h.account_id,
            todo,
            momentum_domain::model::ContextRef::Area(area_id),
        )
        .await
        .expect("move");

    h.areas
        .delete(h.account_id, area_id, AreaDeletionPolicy::Detach)
        .await
        .expect("delete area");

    let stored = h
        .todos
        .get(h.account_id, todo)
        .await
        .expect("get")
        .expect("todo");
    assert!(stored.context.is_inbox(), "detached to-do returns to Inbox");
    assert_eq!(stored.status, momentum_domain::model::Status::Open);
    assert!(h
        .areas
        .get(h.account_id, area_id)
        .await
        .expect("get")
        .is_none());
}

#[tokio::test]
async fn trash_policy_moves_todos_to_trash() {
    let h = harness().await;
    let area_id = h
        .areas
        .create(h.account_id, "Trash me")
        .await
        .expect("create")
        .entity_id
        .expect("id");
    let todo = h
        .todos
        .create(h.account_id, "Doomed item", None)
        .await
        .expect("create todo")
        .entity_id
        .expect("id");
    h.todos
        .move_to(
            h.account_id,
            todo,
            momentum_domain::model::ContextRef::Area(area_id),
        )
        .await
        .expect("move");

    h.areas
        .delete(h.account_id, area_id, AreaDeletionPolicy::TrashContents)
        .await
        .expect("delete area");

    let stored = h
        .todos
        .get(h.account_id, todo)
        .await
        .expect("get")
        .expect("todo");
    assert_eq!(stored.status, momentum_domain::model::Status::Deleted);
}

#[tokio::test]
async fn detach_policy_clears_project_assignment() {
    let h = harness().await;
    let area_id = h
        .areas
        .create(h.account_id, "Projects area")
        .await
        .expect("create")
        .entity_id
        .expect("id");
    let project_id = momentum_domain::EntityId::new();
    h.database
        .assign_project_area(h.account_id, project_id, Some(area_id));

    let summary = h
        .areas
        .get(h.account_id, area_id)
        .await
        .expect("get")
        .expect("area");
    assert_eq!(summary.project_count, 1);

    h.areas
        .delete(h.account_id, area_id, AreaDeletionPolicy::Detach)
        .await
        .expect("delete");

    assert!(h.database.project_area(h.account_id, project_id).is_none());
}

#[tokio::test]
async fn areas_page_requires_authentication_for_mutations() {
    let h = harness().await;
    let builder = Request::builder()
        .method("POST")
        .uri("/areas")
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");
    let request = builder.body(Body::from("title=Nope")).expect("request");
    let response = h.router.clone().oneshot(request).await.expect("response");
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        response
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok()),
        Some("/login")
    );
}
