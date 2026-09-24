#![cfg(feature = "ssr")]

//! Project heading web-surface tests.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::{CommandDispatcher, ReorderDirection};
use momentum_app::projects::ProjectService;
use momentum_app::todos::TodoService;
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
    projects: Arc<ProjectService>,
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
    let projects = Arc::new(ProjectService::new(
        dispatcher,
        Arc::new(MemoryProjectReadStore::new(database)),
        clock,
    ));

    let registration = auth
        .register("headings@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("headings@example.com", "correct horse battery")
        .await
        .expect("login");

    let state = AppState::new(
        server::leptos_options(),
        None,
        Arc::new(CookieSessionResolver::new(auth.clone())),
        Some(Arc::new(AuthContext::new(auth, false))),
        Some(todos),
        None,
        Some(projects.clone()),
    );

    Harness {
        router: server::router_with_state(state),
        projects,
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

async fn project(h: &Harness) -> momentum_domain::EntityId {
    h.projects
        .create(h.account_id, "Web project", None)
        .await
        .expect("create")
        .entity_id
        .expect("id")
}

#[tokio::test]
async fn heading_controls_render_and_create_heading() {
    let h = harness().await;
    let id = project(&h).await;

    let (status, _, _) = request(
        &h,
        "POST",
        "/headings",
        Some(&format!(
            "project_id={id}&title=Phase+one&return_to=%2Fprojects%2F{id}"
        )),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);

    let (status, _, body) = request(&h, "GET", &format!("/projects/{id}"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("data-testid=\"heading-section\""), "{body}");
    assert!(body.contains("Phase one"), "{body}");
    assert!(body.contains("data-testid=\"add-heading-form\""), "{body}");

    let summary = h
        .projects
        .get(h.account_id, id)
        .await
        .expect("get")
        .expect("project");
    let heading = summary.headings[0].id;

    let (status, _, _) = request(
        &h,
        "POST",
        &format!("/headings/{heading}/rename"),
        Some(&format!("title=Phase+1&return_to=%2Fprojects%2F{id}")),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let summary = h
        .projects
        .get(h.account_id, id)
        .await
        .expect("get")
        .expect("project");
    assert_eq!(summary.headings[0].title, "Phase 1");
}

#[tokio::test]
async fn heading_scoped_todo_and_archive_flow() {
    let h = harness().await;
    let id = project(&h).await;
    h.projects
        .create_heading(h.account_id, id, "Section")
        .await
        .expect("heading");
    let heading = h
        .projects
        .get(h.account_id, id)
        .await
        .expect("get")
        .expect("project")
        .headings[0]
        .id;

    let body = format!(
        "title=Filed+under+heading&context_kind=heading&context_id={heading}\
         &return_to=%2Fprojects%2F{id}"
    );
    let (status, _, _) = request(&h, "POST", "/todos", Some(&body)).await;
    assert_eq!(status, StatusCode::SEE_OTHER);

    let (_, _, detail) = request(&h, "GET", &format!("/projects/{id}"), None).await;
    assert!(detail.contains("Filed under heading"), "{detail}");
    assert!(
        detail.contains("data-testid=\"heading-children\""),
        "{detail}"
    );

    let (status, _, _) = request(
        &h,
        "POST",
        &format!("/headings/{heading}/archive"),
        Some(&format!("return_to=%2Fprojects%2F{id}")),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (_, _, detail) = request(&h, "GET", &format!("/projects/{id}"), None).await;
    assert!(
        detail.contains("data-testid=\"archived-headings\""),
        "{detail}"
    );

    let (status, _, _) = request(
        &h,
        "POST",
        &format!("/headings/{heading}/restore"),
        Some(&format!("return_to=%2Fprojects%2F{id}")),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (_, _, detail) = request(&h, "GET", &format!("/projects/{id}"), None).await;
    assert!(
        !detail.contains("data-testid=\"archived-headings\""),
        "{detail}"
    );
}

#[tokio::test]
async fn heading_reorder_endpoints_move_the_heading() {
    let h = harness().await;
    let id = project(&h).await;
    for title in ["First", "Second"] {
        h.projects
            .create_heading(h.account_id, id, title)
            .await
            .expect("heading");
    }
    let summary = h
        .projects
        .get(h.account_id, id)
        .await
        .expect("get")
        .expect("project");
    let first = summary.headings[0].id;

    let (status, _, _) = request(
        &h,
        "POST",
        &format!("/headings/{first}/move_down"),
        Some(&format!("return_to=%2Fprojects%2F{id}")),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);

    let summary = h
        .projects
        .get(h.account_id, id)
        .await
        .expect("get")
        .expect("project");
    assert_eq!(order_titles(&summary), vec!["Second", "First"]);

    h.projects
        .reorder_heading(h.account_id, first, ReorderDirection::Up)
        .await
        .expect("move up");
    let summary = h
        .projects
        .get(h.account_id, id)
        .await
        .expect("get")
        .expect("project");
    assert_eq!(order_titles(&summary), vec!["First", "Second"]);
}

fn order_titles(summary: &momentum_app::projects::ProjectSummary) -> Vec<String> {
    summary
        .order
        .iter()
        .map(|item| match item {
            momentum_app::projects::ProjectOrderItem::Heading(id) => summary
                .headings
                .iter()
                .find(|heading| heading.id == *id)
                .map_or_else(|| "?".to_string(), |heading| heading.title.clone()),
            momentum_app::projects::ProjectOrderItem::Todo(_) => "(todo)".to_string(),
        })
        .collect()
}
