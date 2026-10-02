#![cfg(feature = "ssr")]

//! Command URLs: add, add-project, update, show, and search automation surfaces.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::projects::ProjectService;
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::model::Schedule;
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};
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
        .register("commands@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("commands@example.com", "correct horse battery")
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

async fn get(h: &Harness, uri: &str) -> (StatusCode, Option<String>, String) {
    let request = Request::builder()
        .method("GET")
        .uri(uri)
        .header(header::COOKIE, &h.cookie)
        .body(Body::empty())
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
async fn add_url_creates_a_fully_specified_todo() {
    let h = harness().await;
    let (status, location, _) = get(
        &h,
        "/add?title=From+URL&notes=Command+note&schedule_kind=tomorrow&return_to=%2Ftoday",
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/today"));

    let created = h
        .todos
        .list(h.account_id, TodoScope::AllOpen)
        .await
        .expect("all open")
        .into_iter()
        .find(|todo| todo.title == "From URL")
        .expect("created");
    assert_eq!(created.notes, "Command note");
    assert_eq!(
        created.schedule,
        Schedule::on(h.todos.local_today().next_day())
    );
}

#[tokio::test]
async fn add_project_url_creates_a_project_with_notes() {
    let h = harness().await;
    let (status, location, _) = get(&h, "/add-project?title=Automation&notes=Created+by+URL").await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let location = location.expect("location");
    assert!(location.starts_with("/projects/"), "{location}");

    let project = h
        .projects
        .list(h.account_id)
        .await
        .expect("projects")
        .into_iter()
        .find(|project| project.title == "Automation")
        .expect("created");
    assert_eq!(project.notes, "Created by URL");
}

#[tokio::test]
async fn update_url_applies_typed_fields_and_rejects_unsupported_values() {
    let h = harness().await;
    h.todos
        .create(h.account_id, "Original", None)
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

    let (status, location, _) = get(
        &h,
        &format!("/update?todo={todo}&title=Updated&when=today&deadline=2026-06-20"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/inbox?saved=1"));
    let updated = h.todos.get(h.account_id, todo).await.unwrap().unwrap();
    assert_eq!(updated.title, "Updated");
    assert_eq!(updated.schedule, Schedule::on(date(10)));
    assert_eq!(updated.deadline.unwrap().date, date(20));

    let (status, location, _) = get(&h, &format!("/update?todo={todo}&when=banana")).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/inbox?error=update"));
    assert_eq!(
        h.todos
            .get(h.account_id, todo)
            .await
            .unwrap()
            .unwrap()
            .schedule,
        Schedule::on(date(10))
    );
}

#[tokio::test]
async fn show_urls_are_bookmarkable_and_reject_unsupported_references() {
    let h = harness().await;
    let (status, location, _) = get(&h, "/show?list=today").await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/today"));

    let (status, location, _) = get(&h, "/show?list=not-a-list").await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/inbox?error=show"));

    let (status, location, _) = get(&h, &format!("/show?todo={}", EntityId::new())).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/inbox?error=notfound"));

    let (status, location, _) = get(&h, "/show?todo=not-a-uuid&list=today").await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/inbox?error=show"));

    h.projects
        .create(h.account_id, "Showable", None)
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
    let (status, location, _) = get(&h, &format!("/show?project={project}")).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(
        location.as_deref(),
        Some(format!("/projects/{project}").as_str())
    );

    // Search stays a stable, non-mutating navigation URL.
    let (status, _, body) = get(&h, "/search?q=anything").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("data-testid=\"quick-find\""), "{body}");
}

fn date(day: u32) -> LocalDate {
    LocalDate::from_ymd(2026, 6, day).expect("date")
}
