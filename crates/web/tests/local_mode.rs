#![cfg(feature = "ssr")]

//! Local mode: no sign-in, in-memory services, and the Finalize brand.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::areas::AreaService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::logbook::LogbookService;
use momentum_app::projects::ProjectService;
use momentum_app::recurring::RecurringService;
use momentum_app::session::{AccountSession, SessionState};
use momentum_app::tags::TagService;
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::AccountId;
use momentum_testkit::areas::MemoryAreaReadStore;
use momentum_testkit::auth::TestClock;
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::logbook::MemoryLoggingPolicyStore;
use momentum_testkit::projects::MemoryProjectReadStore;
use momentum_testkit::recurring::MemoryRecurrenceReadStore;
use momentum_testkit::tags::MemoryTagReadStore;
use momentum_testkit::todos::MemoryTodoReadStore;
use momentum_web::server::{self, AppState};
use momentum_web::session::FixedSessionResolver;
use tower::ServiceExt;

const LOCAL_ACCOUNT: &str = "00000000-0000-0000-0000-000000000001";

struct Harness {
    router: axum::Router,
    todos: Arc<TodoService>,
    account_id: AccountId,
}

/// Builds the same composition as the server's local mode branch.
fn harness() -> Harness {
    let database = MemoryDatabase::new();
    let clock = Arc::new(TestClock::new(
        Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap(),
    ));
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
        dispatcher.clone(),
        Arc::new(MemoryProjectReadStore::new(database.clone())),
        clock.clone(),
    ));
    let tags = Arc::new(TagService::new(
        dispatcher.clone(),
        Arc::new(MemoryTagReadStore::new(database.clone())),
        clock.clone(),
    ));
    let recurring = Arc::new(RecurringService::new(Arc::new(
        MemoryRecurrenceReadStore::new(),
    )));
    let logbook = Arc::new(LogbookService::new(
        dispatcher,
        todos.clone(),
        projects.clone(),
        Arc::new(MemoryLoggingPolicyStore::new()),
        clock,
    ));
    let account_id =
        AccountId::from_uuid(uuid::Uuid::parse_str(LOCAL_ACCOUNT).expect("local uuid"));
    let resolver = FixedSessionResolver(SessionState::Authenticated(AccountSession::new(
        account_id,
        "local@finalize",
    )));
    let state = AppState::new(
        server::leptos_options(),
        None,
        Arc::new(resolver),
        None,
        Some(todos.clone()),
        Some(areas),
        Some(projects),
    )
    .with_tags(tags)
    .with_logbook(logbook)
    .with_recurring(recurring);

    Harness {
        router: server::router_with_state(state),
        todos,
        account_id,
    }
}

async fn request(
    h: &Harness,
    method: &str,
    uri: &str,
    body: Option<&str>,
) -> (StatusCode, Option<String>, String) {
    let mut builder = Request::builder().method(method).uri(uri);
    if body.is_some() {
        builder = builder.header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");
    }
    let request = builder
        .body(body.map_or_else(Body::empty, |body| Body::from(body.to_string())))
        .expect("request");
    let response = h.router.clone().oneshot(request).await.expect("response");
    let status = response.status();
    let location = response
        .headers()
        .get(header::LOCATION)
        .and_then(|value| value.to_str().ok())
        .map(str::to_string);
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    (
        status,
        location,
        String::from_utf8(bytes.to_vec()).expect("utf-8"),
    )
}

#[tokio::test]
async fn local_mode_serves_authenticated_pages_without_sign_in() {
    let h = harness();
    for path in ["/", "/login", "/signup"] {
        let (status, location, _) = request(&h, "GET", path, None).await;
        assert_eq!(status, StatusCode::SEE_OTHER, "{path}");
        assert_eq!(location.as_deref(), Some("/today"), "{path}");
    }
    let (status, _, body) = request(&h, "GET", "/today", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Finalize"), "{body}");
    assert!(body.contains("local@finalize"), "{body}");
    assert!(!body.contains("Sign in"), "{body}");
    assert!(!body.contains("Authentication is not configured"), "{body}");

    let (status, _, manifest) = request(&h, "GET", "/manifest.webmanifest", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(manifest.contains("\"name\":\"Finalize\""), "{manifest}");
}

#[tokio::test]
async fn local_mode_captures_and_completes_through_normal_commands() {
    let h = harness();
    let (status, location, _) = request(
        &h,
        "POST",
        "/todos",
        Some("title=Local+task&return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/inbox"));
    let todo = h
        .todos
        .list(h.account_id, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .find(|todo| todo.title == "Local task")
        .expect("created");
    assert_eq!(todo.context, momentum_domain::model::ContextRef::Inbox);

    let (status, _, _) = request(&h, "POST", &format!("/todos/{}/complete", todo.id), None).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert!(h
        .todos
        .get(h.account_id, todo.id)
        .await
        .unwrap()
        .unwrap()
        .completed_at
        .is_some());
}
