#![cfg(feature = "ssr")]

//! Tag web-surface tests.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::tags::TagService;
use momentum_app::todos::TodoService;
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
    tags: Arc<TagService>,
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
    let tags = Arc::new(TagService::new(
        dispatcher,
        Arc::new(MemoryTagReadStore::new(database)),
        clock,
    ));

    let registration = auth
        .register("tags@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("tags@example.com", "correct horse battery")
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
        tags,
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

#[tokio::test]
async fn tags_page_manages_hierarchy() {
    let h = harness().await;
    let (status, _) = request(&h, "POST", "/tags", Some("title=Work&return_to=%2Ftags")).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let parent = h.tags.list(h.account_id).await.expect("list")[0].id;

    let body = format!("title=Deep&parent={parent}&return_to=%2Ftags");
    let (status, _) = request(&h, "POST", "/tags", Some(&body)).await;
    assert_eq!(status, StatusCode::SEE_OTHER);

    let (status, page) = request(&h, "GET", "/tags", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(page.contains("data-testid=\"tag-list\""), "{page}");
    assert!(page.contains("Deep"), "{page}");
    assert!(page.contains("data-testid=\"add-tag-form\""), "{page}");

    let (status, _) = request(
        &h,
        "POST",
        &format!("/tags/{parent}/rename"),
        Some("title=Work%21&return_to=%2Ftags"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let listed = h.tags.list(h.account_id).await.expect("list");
    assert!(listed.iter().any(|tag| tag.title == "Work!"));

    let (status, _) = request(
        &h,
        "POST",
        &format!("/tags/{parent}/delete"),
        Some("return_to=%2Ftags"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let listed = h.tags.list(h.account_id).await.expect("list");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].title, "Deep");
    assert_eq!(
        listed[0].parent, None,
        "orphans are reparented to top level"
    );
}

#[tokio::test]
async fn assignment_and_filtering_include_inherited_context_tags() {
    let h = harness().await;
    let tag = h
        .tags
        .create(h.account_id, "Urgent", None)
        .await
        .expect("tag")
        .entity_id
        .expect("id");
    let tagged = h
        .todos
        .create(h.account_id, "Tagged task", None)
        .await
        .expect("todo")
        .entity_id
        .expect("id");
    h.todos
        .create(h.account_id, "Plain task", None)
        .await
        .expect("todo");

    let (status, _) = request(
        &h,
        "POST",
        &format!("/entities/{tagged}/tags/{tag}/assign"),
        Some("return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);

    let (_, filtered) = request(&h, "GET", &format!("/inbox?tags={tag}"), None).await;
    assert!(filtered.contains("Tagged task"), "{filtered}");
    assert!(!filtered.contains("Plain task"), "{filtered}");

    let (_, unfiltered) = request(&h, "GET", "/inbox", None).await;
    assert!(unfiltered.contains("Plain task"), "{unfiltered}");

    let (status, _) = request(
        &h,
        "POST",
        &format!("/entities/{tagged}/tags/{tag}/unassign"),
        Some("return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (_, filtered) = request(&h, "GET", &format!("/inbox?tags={tag}"), None).await;
    assert!(!filtered.contains("Tagged task"), "{filtered}");
}
