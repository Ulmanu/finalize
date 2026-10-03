#![cfg(feature = "ssr")]

//! Export/import: versioned backup, CSV/Markdown exports, and validated additive import.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_app::commands::{CommandDispatcher, ScheduleInput};
use momentum_app::projects::ProjectService;
use momentum_app::tags::TagService;
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::time::LocalDate;
use momentum_domain::AccountId;
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
        .register("backup@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("backup@example.com", "correct horse battery")
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
    )
    .with_tags(tags.clone());

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
    body: Option<String>,
    content_type: Option<&str>,
) -> (StatusCode, String, String) {
    let mut builder = Request::builder()
        .method(method)
        .uri(uri)
        .header(header::COOKIE, &h.cookie);
    if let Some(content_type) = content_type {
        builder = builder.header(header::CONTENT_TYPE, content_type);
    }
    let request = builder
        .body(body.map_or_else(Body::empty, Body::from))
        .expect("request");
    let response = h.router.clone().oneshot(request).await.expect("response");
    let status = response.status();
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
    (
        status,
        content_type,
        String::from_utf8(bytes.to_vec()).expect("utf-8"),
    )
}

#[tokio::test]
async fn exports_backup_csv_and_markdown() {
    let h = harness().await;
    h.todos
        .create(h.account_id, "Exported", Some("Notes".to_string()))
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
            ScheduleInput::On(LocalDate::from_ymd(2026, 6, 12).unwrap()),
        )
        .await
        .expect("schedule");
    h.todos
        .add_checklist_item(h.account_id, todo, "Step")
        .await
        .expect("checklist");
    h.projects
        .create(h.account_id, "Backup project", None)
        .await
        .expect("project");
    h.tags
        .create(h.account_id, "Backup tag", None)
        .await
        .expect("tag");

    let (status, content_type, body) = request(&h, "GET", "/export/backup.json", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(content_type.contains("application/json"), "{content_type}");
    assert!(body.contains("\"version\":1"), "{body}");
    assert!(body.contains("Exported"), "{body}");
    assert!(body.contains("Backup project"), "{body}");
    assert!(body.contains("Backup tag"), "{body}");
    assert!(body.contains("2026-06-12"), "{body}");

    let (status, content_type, body) = request(&h, "GET", "/export/todos.csv", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(content_type.contains("text/csv"), "{content_type}");
    assert!(
        body.starts_with("title,status,scheduled_on,evening,deadline,notes\n"),
        "{body}"
    );
    assert!(body.contains("Exported,open,2026-06-12"), "{body}");

    let (status, content_type, body) = request(&h, "GET", "/export/todos.md", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(content_type.contains("text/markdown"), "{content_type}");
    assert!(body.contains("- [ ] Exported (when: 2026-06-12)"), "{body}");
    assert!(body.contains("  - [ ] Step"), "{body}");
}

#[tokio::test]
async fn import_validates_and_applies_additively() {
    let h = harness().await;
    h.todos
        .create(h.account_id, "Existing", None)
        .await
        .expect("existing");

    let document = serde_json::json!({
        "version": 1,
        "exported_at": "2026-06-10T09:00:00Z",
        "areas": [],
        "projects": [],
        "tags": [],
        "todos": [
            {
                "title": "Imported",
                "notes": "From backup",
                "status": "open",
                "scheduled_on": "2026-06-12",
                "evening": false,
                "deadline": null,
                "checklist": ["One"]
            },
            {
                "title": "",
                "notes": "",
                "status": "open",
                "scheduled_on": null,
                "evening": false,
                "deadline": null,
                "checklist": []
            }
        ]
    })
    .to_string();
    let (status, _, body) = request(
        &h,
        "POST",
        "/import/backup.json",
        Some(document),
        Some("application/json"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("\"imported\":1"), "{body}");
    assert!(body.contains("title is empty"), "{body}");

    let titles: Vec<String> = h
        .todos
        .list(h.account_id, TodoScope::AllOpen)
        .await
        .expect("all open")
        .into_iter()
        .map(|todo| todo.title)
        .collect();
    assert!(titles.contains(&"Existing".to_string()));
    assert!(titles.contains(&"Imported".to_string()));

    let unsupported = serde_json::json!({
        "version": 99,
        "exported_at": "2026-06-10T09:00:00Z",
        "areas": [],
        "projects": [],
        "tags": [],
        "todos": []
    })
    .to_string();
    let (status, _, body) = request(
        &h,
        "POST",
        "/import/backup.json",
        Some(unsupported),
        Some("application/json"),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(body.contains("\"code\":\"unsupported_version\""), "{body}");
}
