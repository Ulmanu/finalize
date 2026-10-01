#![cfg(feature = "ssr")]

//! Sync protocol route: JSON push/pull and service availability.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::sync::SyncService;
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::AccountId;
use momentum_testkit::auth::{
    DeterministicHasher, DeterministicTokenGenerator, MemoryAuthStore, TestClock,
};
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::sync::MemoryChangeLogReader;
use momentum_testkit::todos::MemoryTodoReadStore;
use momentum_web::auth_routes::AuthContext;
use momentum_web::server::{self, AppState};
use momentum_web::session::CookieSessionResolver;
use tower::ServiceExt;
use uuid::Uuid;

struct Harness {
    router: axum::Router,
    todos: Arc<TodoService>,
    account_id: AccountId,
    cookie: String,
}

async fn harness(with_sync: bool) -> Harness {
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

    let registration = auth
        .register("sync@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("sync@example.com", "correct horse battery")
        .await
        .expect("login");

    let mut state = AppState::new(
        server::leptos_options(),
        None,
        Arc::new(CookieSessionResolver::new(auth.clone())),
        Some(Arc::new(AuthContext::new(auth, false))),
        Some(todos.clone()),
        None,
        None,
    );
    if with_sync {
        state = state.with_sync(Arc::new(SyncService::new(
            dispatcher,
            todos.clone(),
            Arc::new(MemoryChangeLogReader::new(database)),
            clock,
        )));
    }

    Harness {
        router: server::router_with_state(state),
        todos,
        account_id: registration.account_id,
        cookie: format!("momentum_session={}", login.token),
    }
}

async fn post_json(h: &Harness, uri: &str, body: &str) -> (StatusCode, String) {
    let request = Request::builder()
        .method("POST")
        .uri(uri)
        .header(header::COOKIE, &h.cookie)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body.to_string()))
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
async fn sync_route_pushes_operations_and_returns_changes() {
    let h = harness(true).await;
    h.todos
        .create(h.account_id, "Alpha", None)
        .await
        .expect("create");
    let todo = h
        .todos
        .list(h.account_id, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .next()
        .expect("todo")
        .id;
    let operation_id = Uuid::now_v7();
    let body = format!(
        "{{\"cursor\":0,\"operations\":[{{\"operation_id\":\"{operation_id}\",\
         \"command\":{{\"command\":\"complete_todo\",\"id\":\"{todo}\"}}}}]}}"
    );

    let (status, response) = post_json(&h, "/sync", &body).await;
    assert_eq!(status, StatusCode::OK);
    assert!(response.contains("\"acknowledged\""), "{response}");
    assert!(response.contains(&operation_id.to_string()), "{response}");
    assert!(response.contains("\"changes\""), "{response}");
    assert!(response.contains("\"cursor\""), "{response}");
    assert!(h
        .todos
        .get(h.account_id, todo)
        .await
        .unwrap()
        .unwrap()
        .completed_at
        .is_some());
}

#[tokio::test]
async fn sync_route_is_unavailable_without_the_service() {
    let h = harness(false).await;
    let (status, _) = post_json(&h, "/sync", "{\"cursor\":0,\"operations\":[]}").await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
}
