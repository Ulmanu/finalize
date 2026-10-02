#![cfg(feature = "ssr")]

//! Quick Entry: overlay markup, full-metadata capture, and no half-created items.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::projects::ProjectService;
use momentum_app::tags::TagService;
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::model::{ContextRef, Schedule};
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};
use momentum_testkit::auth::{
    DeterministicHasher, DeterministicTokenGenerator, MemoryAuthStore, TestClock,
};
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::projects::MemoryProjectReadStore;
use momentum_testkit::tags::MemoryTagReadStore;
use momentum_testkit::todos::MemoryTodoReadStore;
use momentum_web::auth_routes::AuthContext;
use momentum_web::server::{self, AppState};
use momentum_web::session::CookieSessionResolver;
use tower::ServiceExt;

struct Harness {
    router: axum::Router,
    todos: Arc<TodoService>,
    projects: Arc<ProjectService>,
    tags: Option<Arc<TagService>>,
    account_id: AccountId,
    cookie: String,
}

async fn harness(with_tags: bool) -> Harness {
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
        dispatcher.clone(),
        Arc::new(MemoryProjectReadStore::new(database.clone())),
        clock.clone(),
    ));
    let tags = Arc::new(TagService::new(
        dispatcher,
        Arc::new(MemoryTagReadStore::new(database)),
        clock,
    ));

    let registration = auth
        .register("quick@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("quick@example.com", "correct horse battery")
        .await
        .expect("login");

    let mut state = AppState::new(
        server::leptos_options(),
        None,
        Arc::new(CookieSessionResolver::new(auth.clone())),
        Some(Arc::new(AuthContext::new(auth, false))),
        Some(todos.clone()),
        None,
        Some(projects.clone()),
    );
    let tags = with_tags.then_some(tags);
    if let Some(tags) = &tags {
        state = state.with_tags(tags.clone());
    }

    Harness {
        router: server::router_with_state(state),
        todos,
        projects,
        tags,
        account_id: registration.account_id,
        cookie: format!("momentum_session={}", login.token),
    }
}

async fn request(
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

#[tokio::test]
async fn overlay_markup_is_available_on_every_page() {
    let h = harness(true).await;
    let (status, _, body) = request(&h, "GET", "/inbox", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("data-testid=\"quick-entry\""), "{body}");
    assert!(
        body.contains("data-testid=\"quick-entry-trigger\""),
        "{body}"
    );
    assert!(body.contains("action=\"/todos/quick\""), "{body}");
    assert!(body.contains("name=\"schedule_kind\""), "{body}");
    assert!(body.contains("name=\"checklist\""), "{body}");
    assert!(body.contains("name=\"destination\""), "{body}");
    assert!(body.contains("name=\"deadline\""), "{body}");
    assert!(body.contains(">Cancel</a>"), "{body}");
}

#[tokio::test]
async fn save_captures_full_metadata_without_leaving_the_destination() {
    let h = harness(true).await;
    h.projects
        .create(h.account_id, "Project", None)
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
    h.tags
        .as_ref()
        .expect("tags")
        .create(h.account_id, "Focus", None)
        .await
        .expect("tag");
    let tag = h
        .tags
        .as_ref()
        .expect("tags")
        .list(h.account_id)
        .await
        .expect("tags")
        .into_iter()
        .next()
        .expect("tag")
        .id;

    let body = format!(
        "title=Alpha&notes=remember+this&schedule_kind=today&deadline=2026-06-15\
         &destination=project%3A{project}&tag={tag}&checklist=One%0ATwo&return_to=%2Ftoday"
    );
    let (status, location, _) = request(&h, "POST", "/todos/quick", Some(&body)).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/today"));

    let created = h
        .todos
        .list(h.account_id, TodoScope::AllOpen)
        .await
        .expect("all open")
        .into_iter()
        .find(|todo| todo.title == "Alpha")
        .expect("created");
    assert_eq!(created.notes, "remember this");
    assert_eq!(created.schedule, Schedule::on(h.todos.local_today()));
    assert_eq!(created.deadline.unwrap().date, date(15));
    assert_eq!(created.context, ContextRef::Project(project));
    assert_eq!(created.tags, vec![tag]);
    assert_eq!(created.checklist.len(), 2);
}

#[tokio::test]
async fn invalid_input_never_leaves_a_half_created_item() {
    let h = harness(true).await;

    // Bad explicit date: rejected before creation.
    let (status, location, _) = request(
        &h,
        "POST",
        "/todos/quick",
        Some("title=Bad&schedule_kind=date&return_to=%2Finbox"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/inbox?error=quick"));
    assert!(open_titles(&h).await.is_empty());

    // Tag requested with no tag service: rejected before creation.
    let plain = harness(false).await;
    let (status, location, _) = request(
        &plain,
        "POST",
        "/todos/quick",
        Some(&format!(
            "title=Tagged&tag={}&return_to=%2Finbox",
            EntityId::new()
        )),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/inbox?error=quick"));
    assert!(open_titles(&plain).await.is_empty());
}

async fn open_titles(h: &Harness) -> Vec<String> {
    h.todos
        .list(h.account_id, TodoScope::AllOpen)
        .await
        .expect("all open")
        .into_iter()
        .map(|todo| todo.title)
        .collect()
}

fn date(day: u32) -> LocalDate {
    LocalDate::from_ymd(2026, 6, day).expect("date")
}
