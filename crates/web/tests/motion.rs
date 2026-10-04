#![cfg(feature = "ssr")]

//! Motion system: centralized primitives in markup and reduced-motion suppression.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::motion::registry;
use momentum_app::todos::TodoService;
use momentum_domain::AccountId;
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
        .register("motion@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("motion@example.com", "correct horse battery")
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

async fn get(h: &Harness, uri: &str) -> (StatusCode, String) {
    let request = Request::builder()
        .method("GET")
        .uri(uri)
        .header(header::COOKIE, &h.cookie)
        .body(Body::empty())
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
async fn markup_uses_the_central_motion_classes() {
    let h = harness().await;
    h.todos
        .create(h.account_id, "Animated task", None)
        .await
        .expect("create");

    let (status, body) = get(&h, "/inbox").await;
    assert_eq!(status, StatusCode::OK);
    for primitive in registry() {
        // `motion-remove` is applied by the client as a row leaves; server markup never
        // contains a leaving row.
        if primitive.id == "remove" {
            continue;
        }
        assert!(
            body.contains(primitive.class_name),
            "missing {}: {body}",
            primitive.class_name
        );
    }
}

#[tokio::test]
async fn stylesheet_declares_primitives_and_suppresses_nonessential_motion() {
    let h = harness().await;
    let (status, css) = get(&h, "/static/app.css").await;
    assert_eq!(status, StatusCode::OK);
    for primitive in registry() {
        assert!(
            css.contains(&format!(".{}", primitive.class_name)),
            "missing rule for {}",
            primitive.class_name
        );
    }
    assert!(css.contains("prefers-reduced-motion: reduce"), "{css}");
    assert!(
        css.contains("[data-reduced-motion=\"true\"] .motion-insert"),
        "{css}"
    );
    assert!(css.contains("animation: none !important"), "{css}");
    assert!(css.contains("transition: none !important"), "{css}");
}
