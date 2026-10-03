#![cfg(feature = "ssr")]

//! Share and print: deterministic models, dedicated print documents, and escaping.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::{CommandDispatcher, ScheduleInput};
use momentum_app::projects::ProjectService;
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::time::LocalDate;
use momentum_domain::AccountId;
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
    projects: Arc<ProjectService>,
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
    let projects = Arc::new(ProjectService::new(
        dispatcher,
        Arc::new(MemoryProjectReadStore::new(database)),
        clock,
    ));

    let registration = auth
        .register("share@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("share@example.com", "correct horse battery")
        .await
        .expect("login");

    let state = AppState::new(
        server::leptos_options(),
        None,
        Arc::new(CookieSessionResolver::new(auth.clone())),
        Some(Arc::new(AuthContext::new(auth, false))),
        Some(todos.clone()),
        None,
        Some(projects.clone()),
    );

    Harness {
        router: server::router_with_state(state),
        todos,
        projects,
        account_id: registration.account_id,
        cookie: format!("momentum_session={}", login.token),
    }
}

async fn get(h: &Harness, uri: &str, with_cookie: bool) -> (StatusCode, String, String) {
    let mut builder = Request::builder().method("GET").uri(uri);
    if with_cookie {
        builder = builder.header(header::COOKIE, &h.cookie);
    }
    let request = builder.body(Body::empty()).expect("request");
    let response = h.router.clone().oneshot(request).await.expect("response");
    let status = response.status();
    let location = response
        .headers()
        .get(header::LOCATION)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_string();
    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_string();
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let body = String::from_utf8(bytes.to_vec()).expect("utf-8");
    let _ = location;
    (status, content_type, body)
}

async fn fixture(h: &Harness) -> (momentum_domain::EntityId, momentum_domain::EntityId) {
    h.todos
        .create(
            h.account_id,
            "<script>alert(1)</script>",
            Some("Notes *markup*".to_string()),
        )
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
    h.projects
        .create(h.account_id, "Share project", None)
        .await
        .expect("project");
    let project = h
        .projects
        .list(h.account_id)
        .await
        .expect("projects")
        .into_iter()
        .next()
        .expect("project")
        .id;
    h.projects
        .set_notes(h.account_id, project, "<b>bold</b>")
        .await
        .expect("notes");
    (todo, project)
}

#[tokio::test]
async fn share_models_are_deterministic_markdown() {
    let h = harness().await;
    let (todo, project) = fixture(&h).await;

    let (status, content_type, first) = get(&h, &format!("/share/todos/{todo}"), true).await;
    assert_eq!(status, StatusCode::OK);
    assert!(content_type.contains("text/markdown"), "{content_type}");
    assert!(
        first.contains("# <script\\>alert\\(1\\)</script\\>"),
        "{first}"
    );
    assert!(first.contains("Notes \\*markup\\*"), "{first}");
    let (_, _, second) = get(&h, &format!("/share/todos/{todo}"), true).await;
    assert_eq!(first, second);

    let (status, _, list) = get(&h, "/share/lists/today", true).await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        list.starts_with("# Today\n\n- [ ] <script\\>alert\\(1\\)</script\\>"),
        "{list}"
    );

    let (status, _, project_markdown) = get(&h, &format!("/share/projects/{project}"), true).await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        project_markdown.starts_with("# Share project"),
        "{project_markdown}"
    );

    let (status, _, _) = get(&h, "/share/lists/not-a-list", true).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn print_documents_escape_content_and_require_a_session() {
    let h = harness().await;
    let (_, project) = fixture(&h).await;

    let (status, content_type, html) = get(&h, "/print/lists/today", true).await;
    assert_eq!(status, StatusCode::OK);
    assert!(content_type.contains("text/html"), "{content_type}");
    assert!(html.contains("<title>Today</title>"), "{html}");
    assert!(
        html.contains("&lt;script&gt;alert(1)&lt;/script&gt;"),
        "{html}"
    );
    assert!(!html.contains("<script>alert(1)</script>"), "{html}");

    let (status, _, project_html) = get(&h, &format!("/print/projects/{project}"), true).await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        project_html.contains("&lt;b&gt;bold&lt;/b&gt;"),
        "{project_html}"
    );
    assert!(!project_html.contains("<b>bold</b>"), "{project_html}");

    let (status, _, _) = get(&h, "/print/lists/not-a-list", true).await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    let (status, _, _) = get(&h, "/print/lists/today", false).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
}
