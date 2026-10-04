#![cfg(feature = "ssr")]

//! Accessibility audit: keyboard access, semantics, labeled controls, and motion/target rules.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::{CommandDispatcher, ScheduleInput};
use momentum_app::preferences::PreferenceStore;
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::time::LocalDate;
use momentum_domain::AccountId;
use momentum_testkit::auth::{
    DeterministicHasher, DeterministicTokenGenerator, MemoryAuthStore, TestClock,
};
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::preferences::MemoryPreferenceStore;
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
        .register("a11y@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("a11y@example.com", "correct horse battery")
        .await
        .expect("login");

    let preferences = Arc::new(MemoryPreferenceStore::new());
    let store_handle: Arc<dyn PreferenceStore> = preferences;
    let state = AppState::new(
        server::leptos_options(),
        None,
        Arc::new(CookieSessionResolver::new(auth.clone())),
        Some(Arc::new(AuthContext::new(auth, false))),
        Some(todos.clone()),
        None,
        None,
    )
    .with_preferences(store_handle);

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

fn audit_controls(body: &str) {
    for element in ["<input", "<select", "<textarea"] {
        let mut rest = body;
        while let Some(start) = rest.find(element) {
            let after = &rest[start..];
            let Some(end) = after.find('>') else { break };
            let tag = &after[..end];
            let exempt = tag.contains("type=\"hidden\"");
            assert!(
                exempt || tag.contains("aria-label") || tag.contains("aria-labelledby"),
                "unlabeled {element}: {tag}"
            );
            rest = &after[end..];
        }
    }
    let mut rest = body;
    while let Some(start) = rest.find("<button") {
        let after = &rest[start..];
        let Some(open_end) = after.find('>') else {
            break;
        };
        let tag = &after[..open_end];
        let Some(close) = after.find("</button>") else {
            break;
        };
        let text = &after[open_end + 1..close];
        let labeled = tag.contains("aria-label") || text.chars().any(|c| c.is_alphanumeric());
        assert!(labeled, "unlabeled button: {tag}");
        rest = &after[close..];
    }
}

#[tokio::test]
async fn core_pages_offer_keyboard_access_and_labeled_controls() {
    let h = harness().await;
    h.todos
        .create(h.account_id, "Accessible task", None)
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
    h.todos
        .schedule(
            h.account_id,
            todo,
            ScheduleInput::On(LocalDate::from_ymd(2026, 6, 10).unwrap()),
        )
        .await
        .expect("schedule");

    for page in ["/inbox", "/today", "/companion", "/settings"] {
        let (status, body) = get(&h, page).await;
        assert_eq!(status, StatusCode::OK, "{page}");
        assert!(body.contains("class=\"skip-link\""), "{page}");
        assert!(body.contains("href=\"#app-main\""), "{page}");
        assert!(body.contains("id=\"app-main\""), "{page}");
        assert!(body.contains("aria-label=\"Primary\""), "{page}");
        if page != "/companion" {
            assert!(body.contains("aria-current"), "{page}");
        }
        audit_controls(&body);
    }
}

#[tokio::test]
async fn semantics_cover_progress_dialogs_and_live_states() {
    let h = harness().await;
    h.todos
        .create(h.account_id, "Progress task", None)
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
    h.todos
        .schedule(
            h.account_id,
            todo,
            ScheduleInput::On(LocalDate::from_ymd(2026, 6, 10).unwrap()),
        )
        .await
        .expect("schedule");
    let (_, companion) = get(&h, "/companion").await;
    assert!(companion.contains("role=\"progressbar\""), "{companion}");
    assert!(companion.contains("aria-valuenow"), "{companion}");
    assert!(companion.contains("aria-valuemax"), "{companion}");

    let (_, inbox) = get(&h, "/inbox").await;
    assert!(inbox.contains("role=\"dialog\""), "{inbox}");
    assert!(inbox.contains("aria-modal=\"true\""), "{inbox}");
    assert!(inbox.contains("aria-keyshortcuts"), "{inbox}");

    let (_, today) = get(&h, "/today").await;
    assert!(today.contains("<progress"), "{today}");
    assert!(today.contains("aria-label"), "{today}");
}

#[tokio::test]
async fn stylesheets_respect_focus_motion_targets_and_contrast() {
    let h = harness().await;
    let (status, app) = get(&h, "/static/app.css").await;
    assert_eq!(status, StatusCode::OK);
    for needle in [
        ".skip-link",
        ":focus-visible",
        "pointer: coarse",
        "min-height: 44px",
        "prefers-reduced-motion",
    ] {
        assert!(app.contains(needle), "app.css missing {needle}");
    }
    let (status, design) = get(&h, "/static/design.css").await;
    assert_eq!(status, StatusCode::OK);
    assert!(design.contains("prefers-contrast: more"), "{design}");
}
