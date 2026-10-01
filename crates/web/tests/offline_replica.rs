#![cfg(feature = "ssr")]

//! Offline replica web-surface test: pending outbox indicator.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::{Command, CommandDispatcher};
use momentum_app::replica::{MemoryReplicaStore, OfflineCoordinator};
use momentum_app::todos::TodoService;
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
    account_id: AccountId,
    cookie: String,
}

async fn harness(with_replica: bool) -> Harness {
    let store = Arc::new(MemoryAuthStore::new());
    let now = Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap();
    let clock = Arc::new(TestClock::new(now));
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
        .register("offline@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("offline@example.com", "correct horse battery")
        .await
        .expect("login");

    let mut state = AppState::new(
        server::leptos_options(),
        None,
        Arc::new(CookieSessionResolver::new(auth.clone())),
        Some(Arc::new(AuthContext::new(auth, false))),
        Some(todos),
        None,
        None,
    );
    if with_replica {
        let coordinator = Arc::new(OfflineCoordinator::new(Box::new(MemoryReplicaStore::new())));
        coordinator
            .apply_locally(
                &Command::CompleteTodo {
                    id: EntityId::new(),
                },
                now,
            )
            .await
            .expect("enqueue");
        state = state.with_replica(coordinator);
    }

    Harness {
        router: server::router_with_state(state),
        account_id: registration.account_id,
        cookie: format!("momentum_session={}", login.token),
    }
}

async fn request(h: &Harness, uri: &str) -> (StatusCode, String) {
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
async fn pending_outbox_operations_surface_in_the_shell() {
    let h = harness(true).await;
    let (status, body) = request(&h, "/inbox").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("data-testid=\"outbox-pending\""), "{body}");
    assert!(body.contains("1 pending"), "{body}");
}

#[tokio::test]
async fn no_indicator_without_a_replica() {
    let h = harness(false).await;
    let (status, body) = request(&h, "/inbox").await;
    assert_eq!(status, StatusCode::OK);
    assert!(!body.contains("data-testid=\"outbox-pending\""), "{body}");
    let _ = h.account_id;
}
