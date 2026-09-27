#![cfg(feature = "ssr")]

//! Multi-selection web-surface tests: checkboxes, the bulk form, and bulk command results.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::model::Schedule;
use momentum_domain::{AccountId, EntityId};
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
    todos: Arc<TodoService>,
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
        dispatcher,
        Arc::new(MemoryTodoReadStore::new(database)),
        clock,
    ));

    let registration = auth
        .register("selection@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("selection@example.com", "correct horse battery")
        .await
        .expect("login");

    let state = AppState::new(
        server::leptos_options(),
        None,
        Arc::new(CookieSessionResolver::new(auth.clone())),
        Some(Arc::new(AuthContext::new(auth, false))),
        Some(todos.clone()),
        None,
        None,
    );

    Harness {
        router: server::router_with_state(state),
        todos,
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

async fn create(h: &Harness, title: &str) -> EntityId {
    h.todos
        .create(h.account_id, title, None)
        .await
        .expect("create");
    h.todos
        .list(h.account_id, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .find(|todo| todo.title == title)
        .expect("created todo")
        .id
}

#[tokio::test]
async fn inbox_rows_render_selection_checkboxes_and_bulk_form() {
    let h = harness().await;
    let a = create(&h, "Alpha").await;
    let b = create(&h, "Beta").await;

    let (status, body) = request(&h, "GET", "/inbox", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("data-testid=\"bulk-form\""), "{body}");
    assert!(body.contains("action=\"/todos/bulk\""), "{body}");
    assert!(body.contains("data-testid=\"bulk-actions\""), "{body}");
    assert!(body.contains("name=\"ids\""), "{body}");
    assert!(body.contains(&format!("value=\"{a}\"")), "{body}");
    assert!(body.contains(&format!("value=\"{b}\"")), "{body}");
    assert!(body.contains("name=\"action\""), "{body}");
}

#[tokio::test]
async fn bulk_complete_processes_eligible_items_and_reports_failures() {
    let h = harness().await;
    let a = create(&h, "Alpha").await;
    let b = create(&h, "Beta").await;
    let c = create(&h, "Gamma").await;
    let unknown = EntityId::new();

    let body = format!("ids={a}&ids={b}&ids={unknown}&action=complete&return_to=%2Finbox");
    let (status, _) = request(&h, "POST", "/todos/bulk", Some(&body)).await;
    assert_eq!(status, StatusCode::SEE_OTHER);

    assert_eq!(
        h.todos.get(h.account_id, a).await.unwrap().unwrap().status,
        momentum_domain::model::Status::Completed
    );
    assert_eq!(
        h.todos.get(h.account_id, b).await.unwrap().unwrap().status,
        momentum_domain::model::Status::Completed
    );
    assert_eq!(
        h.todos.get(h.account_id, c).await.unwrap().unwrap().status,
        momentum_domain::model::Status::Open
    );

    let body = format!("ids={c}&action=complete&return_to=%2Finbox");
    let (status, _) = request(&h, "POST", "/todos/bulk", Some(&body)).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(
        h.todos.get(h.account_id, c).await.unwrap().unwrap().status,
        momentum_domain::model::Status::Completed
    );

    let (status, _) = request(
        &h,
        "POST",
        "/todos/bulk",
        Some("action=complete&return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
}

#[tokio::test]
async fn bulk_schedule_and_delete_apply_to_every_selected_item() {
    let h = harness().await;
    let a = create(&h, "Alpha").await;
    let b = create(&h, "Beta").await;

    let body = format!("ids={a}&ids={b}&action=schedule_tomorrow&return_to=%2Finbox");
    let (status, _) = request(&h, "POST", "/todos/bulk", Some(&body)).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let tomorrow = h.todos.local_today().next_day();
    assert_eq!(
        h.todos
            .get(h.account_id, a)
            .await
            .unwrap()
            .unwrap()
            .schedule,
        Schedule::on(tomorrow)
    );
    assert_eq!(
        h.todos
            .get(h.account_id, b)
            .await
            .unwrap()
            .unwrap()
            .schedule,
        Schedule::on(tomorrow)
    );

    let body = format!("ids={a}&ids={b}&action=delete&return_to=%2Finbox");
    let (status, _) = request(&h, "POST", "/todos/bulk", Some(&body)).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert!(h
        .todos
        .list(h.account_id, TodoScope::Inbox)
        .await
        .expect("inbox")
        .is_empty());
}
