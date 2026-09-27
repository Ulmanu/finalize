#![cfg(feature = "ssr")]

//! Expanded row-editor web-surface tests: expansion, progressive metadata, commit/cancel.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::tags::TagService;
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::model::Schedule;
use momentum_domain::time::LocalDate;
use momentum_domain::AccountId;
use momentum_testkit::auth::{
    DeterministicHasher, DeterministicTokenGenerator, MemoryAuthStore, TestClock,
};
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::tags::MemoryTagReadStore;
use momentum_testkit::todos::MemoryTodoReadStore;
use momentum_web::auth_routes::AuthContext;
use momentum_web::server::{self, AppState};
use momentum_web::session::CookieSessionResolver;
use tower::ServiceExt;

struct Harness {
    router: axum::Router,
    todos: Arc<TodoService>,
    tags: Arc<TagService>,
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
    let tags = Arc::new(TagService::new(
        dispatcher,
        Arc::new(MemoryTagReadStore::new(database)),
        clock,
    ));

    let registration = auth
        .register("editor@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("editor@example.com", "correct horse battery")
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
    )
    .with_tags(tags.clone());

    Harness {
        router: server::router_with_state(state),
        todos,
        tags,
        account_id: registration.account_id,
        cookie: format!("momentum_session={}", login.token),
    }
}

async fn raw_request(
    h: &Harness,
    method: &str,
    uri: &str,
    body: Option<&str>,
) -> (StatusCode, Option<String>, String) {
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

async fn request(h: &Harness, method: &str, uri: &str, body: Option<&str>) -> (StatusCode, String) {
    let (status, _, body) = raw_request(h, method, uri, body).await;
    (status, body)
}

#[tokio::test]
async fn inbox_rows_expand_into_editor_with_progressive_metadata() {
    let h = harness().await;
    h.todos
        .create(h.account_id, "Alpha", Some("some notes".to_string()))
        .await
        .expect("create");
    let todo = h
        .todos
        .list(h.account_id, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .next()
        .expect("todo");
    h.tags
        .create(h.account_id, "Focus", None)
        .await
        .expect("tag");
    let tag = h
        .tags
        .list(h.account_id)
        .await
        .expect("tags")
        .into_iter()
        .next()
        .expect("tag");

    let (status, body) = request(&h, "GET", "/inbox", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("data-testid=\"row-editor\""), "{body}");
    assert!(body.contains("data-testid=\"editor-form\""), "{body}");
    assert!(
        body.contains(&format!("action=\"/todos/{}\"", todo.id)),
        "{body}"
    );
    assert!(body.contains("value=\"Alpha\""), "{body}");
    assert!(body.contains("some notes"), "{body}");
    assert!(body.contains("data-testid=\"editor-meta\""), "{body}");
    assert!(body.contains("name=\"when\""), "{body}");
    assert!(body.contains("data-testid=\"editor-deadline\""), "{body}");
    assert!(
        body.contains(&format!("/entities/{}/tags/{}/assign", todo.id, tag.id)),
        "{body}"
    );
    assert!(body.contains("Tag Focus"), "{body}");
    assert!(body.contains("Checklist &amp; recurrence"), "{body}");
    assert!(body.contains("href=\"/inbox\""), "{body}");
    assert!(body.contains("Cancel</a>"), "{body}");
}

#[tokio::test]
async fn editor_commit_schedules_deadline_and_cancel_keeps_state() {
    let h = harness().await;
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
        .expect("todo");

    let (status, location, _) = raw_request(
        &h,
        "POST",
        &format!("/todos/{}", todo.id),
        Some("title=Alpha+edited&notes=updated+notes&return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/inbox?saved=1"));
    let updated = h.todos.get(h.account_id, todo.id).await.unwrap().unwrap();
    assert_eq!(updated.title, "Alpha edited");
    assert_eq!(updated.notes, "updated notes");

    let (status, _, _) = raw_request(
        &h,
        "POST",
        &format!("/todos/{}/schedule", todo.id),
        Some("when=today&return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let scheduled = h.todos.get(h.account_id, todo.id).await.unwrap().unwrap();
    assert_eq!(scheduled.schedule, Schedule::on(h.todos.local_today()));

    let (status, _, _) = raw_request(
        &h,
        "POST",
        &format!("/todos/{}/deadline", todo.id),
        Some("date=2026-06-15&return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let dated = h.todos.get(h.account_id, todo.id).await.unwrap().unwrap();
    assert_eq!(
        dated.deadline.unwrap().date,
        LocalDate::from_ymd(2026, 6, 15).unwrap()
    );

    let (status, _, _) = raw_request(
        &h,
        "POST",
        &format!("/todos/{}/schedule", todo.id),
        Some("when=date&date=2026-06-20&return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let rescheduled = h.todos.get(h.account_id, todo.id).await.unwrap().unwrap();
    assert_eq!(
        rescheduled.schedule,
        Schedule::on(LocalDate::from_ymd(2026, 6, 20).unwrap())
    );
}

#[tokio::test]
async fn editor_tag_controls_assign_and_offer_removal() {
    let h = harness().await;
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
        .expect("todo");
    h.tags
        .create(h.account_id, "Focus", None)
        .await
        .expect("tag");
    let tag = h
        .tags
        .list(h.account_id)
        .await
        .expect("tags")
        .into_iter()
        .next()
        .expect("tag");

    let (status, _, _) = raw_request(
        &h,
        "POST",
        &format!("/entities/{}/tags/{}/assign", todo.id, tag.id),
        Some("return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(
        h.todos
            .get(h.account_id, todo.id)
            .await
            .unwrap()
            .unwrap()
            .tags,
        vec![tag.id]
    );

    let (_, body) = request(&h, "GET", "/inbox", None).await;
    assert!(
        body.contains(&format!("/entities/{}/tags/{}/unassign", todo.id, tag.id)),
        "{body}"
    );
    assert!(body.contains("Remove Focus"), "{body}");
}
