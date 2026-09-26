#![cfg(feature = "ssr")]

//! Trash web-surface tests.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::areas::AreaService;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::projects::ProjectService;
use momentum_app::todos::TodoService;
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
        Arc::new(MemoryProjectReadStore::new(database.clone())),
        clock.clone(),
    ));
    let areas = Arc::new(AreaService::new(
        dispatcher,
        Arc::new(MemoryAreaReadStore::new(database)),
        clock,
    ));

    let registration = auth
        .register("trash@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("trash@example.com", "correct horse battery")
        .await
        .expect("login");

    let state = AppState::new(
        server::leptos_options(),
        None,
        Arc::new(CookieSessionResolver::new(auth.clone())),
        Some(Arc::new(AuthContext::new(auth, false))),
        Some(todos.clone()),
        Some(areas),
        Some(projects),
    );

    Harness {
        router: server::router_with_state(state),
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

#[tokio::test]
async fn trash_lists_deleted_items_and_restores_them() {
    let h = harness().await;
    let id = h
        .todos
        .create(h.account_id, "Recoverable", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    let (status, _, _) = request(&h, "POST", &format!("/todos/{id}/delete"), None).await;
    assert_eq!(status, StatusCode::SEE_OTHER);

    let (status, _, page) = request(&h, "GET", "/trash", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(page.contains("data-testid=\"trash-todos\""), "{page}");
    assert!(page.contains("Recoverable"), "{page}");
    assert!(page.contains("data-testid=\"empty-trash-form\""), "{page}");
    assert!(page.contains("Delete permanently"), "{page}");

    let (status, _, _) = request(
        &h,
        "POST",
        &format!("/todos/{id}/restore"),
        Some("return_to=%2Ftrash"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (_, _, page) = request(&h, "GET", "/trash", None).await;
    assert!(page.contains("data-testid=\"trash-empty\""), "{page}");
    let (_, _, inbox) = request(&h, "GET", "/inbox", None).await;
    assert!(inbox.contains("Recoverable"), "{inbox}");
}

#[tokio::test]
async fn permanent_deletion_is_guarded_and_empty_trash_purges() {
    let h = harness().await;
    let active = h
        .todos
        .create(h.account_id, "Active", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    let (status, headers, _) = request(
        &h,
        "POST",
        &format!("/todos/{active}/purge"),
        Some("return_to=%2Ftrash"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let location = headers
        .get(header::LOCATION)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    assert!(location.contains("error=purge"), "{location}");
    assert!(
        h.todos
            .get(h.account_id, active)
            .await
            .expect("get")
            .is_some(),
        "active items survive a rejected purge"
    );

    let doomed = h
        .todos
        .create(h.account_id, "Doomed", None)
        .await
        .expect("create")
        .entity_id
        .expect("id");
    let (status, _, _) = request(&h, "POST", &format!("/todos/{doomed}/delete"), None).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (status, _, _) = request(&h, "POST", "/trash/empty", Some("return_to=%2Ftrash")).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (_, _, page) = request(&h, "GET", "/trash", None).await;
    assert!(page.contains("data-testid=\"trash-empty\""), "{page}");
    assert!(h
        .todos
        .get(h.account_id, doomed)
        .await
        .expect("get")
        .is_none());
}
