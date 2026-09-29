#![cfg(feature = "ssr")]

//! Quick Find web-surface tests: ranking, static destinations, and recents recording.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::areas::AreaService;
use momentum_app::auth::AuthService;
use momentum_app::commands::CommandDispatcher;
use momentum_app::projects::ProjectService;
use momentum_app::search::{MemoryRecentsStore, RecentsStore};
use momentum_app::tags::TagService;
use momentum_app::todos::TodoService;
use momentum_domain::{AccountId, EntityId};
use momentum_testkit::areas::MemoryAreaReadStore;
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
    recents: Arc<MemoryRecentsStore>,
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
        dispatcher.clone(),
        Arc::new(MemoryTagReadStore::new(database.clone())),
        clock.clone(),
    ));
    let areas = Arc::new(AreaService::new(
        dispatcher,
        Arc::new(MemoryAreaReadStore::new(database)),
        clock,
    ));

    let registration = auth
        .register("find@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("find@example.com", "correct horse battery")
        .await
        .expect("login");

    let recents = Arc::new(MemoryRecentsStore::new());
    let state = AppState::new(
        server::leptos_options(),
        None,
        Arc::new(CookieSessionResolver::new(auth.clone())),
        Some(Arc::new(AuthContext::new(auth, false))),
        Some(todos.clone()),
        Some(areas),
        Some(projects.clone()),
    )
    .with_tags(tags.clone())
    .with_recents(recents.clone() as Arc<dyn RecentsStore>);

    Harness {
        router: server::router_with_state(state),
        todos,
        projects,
        tags,
        recents,
        account_id: registration.account_id,
        cookie: format!("momentum_session={}", login.token),
    }
}

async fn raw_request(h: &Harness, method: &str, uri: &str) -> (StatusCode, Option<String>, String) {
    let request = Request::builder()
        .method(method)
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

async fn create_project(h: &Harness, title: &str) -> EntityId {
    h.projects
        .create(h.account_id, title, None)
        .await
        .expect("project");
    h.projects
        .list(h.account_id)
        .await
        .expect("projects")
        .into_iter()
        .find(|project| project.title == title)
        .expect("created project")
        .id
}

#[tokio::test]
async fn search_ranks_matches_and_covers_lists_entities_and_settings() {
    let h = harness().await;
    let project = create_project(&h, "Roadmap").await;
    h.tags
        .create(h.account_id, "roadmap", None)
        .await
        .expect("tag");
    let tag = h
        .tags
        .list(h.account_id)
        .await
        .expect("tags")
        .into_iter()
        .next()
        .expect("tag")
        .id;
    h.todos
        .create(h.account_id, "Prepare roadmap", None)
        .await
        .expect("todo");
    let todo = h
        .todos
        .list(h.account_id, momentum_app::todos::TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .next()
        .expect("todo")
        .id;

    let prefix_project = create_project(&h, "Roadmaps future").await;

    let (status, _, body) = raw_request(&h, "GET", "/search?q=roadmap").await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("data-testid=\"quick-find-results\""),
        "{body}"
    );
    let tag_href = format!("%2Finbox%3Ftags%3D{tag}");
    let project_href = format!("%2Fprojects%2F{project}");
    let prefix_href = format!("%2Fprojects%2F{prefix_project}");
    let todo_href = format!("%2Ftodos%2F{todo}");
    let index = |needle: &str| {
        body.find(needle)
            .unwrap_or_else(|| panic!("missing {needle}: {body}"))
    };
    // Exact matches first (project before tag by kind order), then the prefix match,
    // then the word-start to-do match; every tier remains visible.
    assert!(
        index(&project_href) < index(&tag_href)
            && index(&tag_href) < index(&prefix_href)
            && index(&prefix_href) < index(&todo_href),
        "{body}"
    );
    assert!(body.contains("data-score=\"0\""), "{body}");
    assert!(body.contains("data-score=\"1\""), "{body}");
    assert!(body.contains("data-score=\"2\""), "{body}");

    let (_, _, body) = raw_request(&h, "GET", "/search?q=today").await;
    assert!(body.contains("%2Ftoday"), "{body}");
    let (_, _, body) = raw_request(&h, "GET", "/search?q=settings").await;
    assert!(body.contains("%2Fsettings"), "{body}");

    let (_, _, body) = raw_request(&h, "GET", "/search").await;
    assert!(body.contains("data-testid=\"quick-find-form\""), "{body}");
    assert!(body.contains("data-testid=\"quick-find-empty\""), "{body}");

    let (_, _, body) = raw_request(&h, "GET", "/inbox").await;
    assert!(body.contains("data-testid=\"quick-find-link\""), "{body}");
}

#[tokio::test]
async fn deep_search_matches_notes_checklists_and_logged_content() {
    let h = harness().await;
    h.todos
        .create(
            h.account_id,
            "Groceries",
            Some("Pick up for the röadmap offsite".to_string()),
        )
        .await
        .expect("todo");
    let groceries = h
        .todos
        .list(h.account_id, momentum_app::todos::TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .find(|todo| todo.title == "Groceries")
        .expect("groceries")
        .id;
    h.todos
        .add_checklist_item(h.account_id, groceries, "Café run")
        .await
        .expect("checklist");
    h.todos
        .create(h.account_id, "Roadmap review", None)
        .await
        .expect("todo");
    let review = h
        .todos
        .list(h.account_id, momentum_app::todos::TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .find(|todo| todo.title == "Roadmap review")
        .expect("review")
        .id;
    h.todos
        .create(h.account_id, "Old errand", None)
        .await
        .expect("todo");
    let errand = h
        .todos
        .list(h.account_id, momentum_app::todos::TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .find(|todo| todo.title == "Old errand")
        .expect("errand")
        .id;
    h.todos
        .complete(h.account_id, errand)
        .await
        .expect("complete");

    // Notes match, and an actual title match still ranks above it.
    let (_, _, body) = raw_request(&h, "GET", "/search?q=roadmap").await;
    let title_href = format!("%2Ftodos%2F{review}");
    let notes_href = format!("%2Ftodos%2F{groceries}");
    assert!(body.contains("data-matched-in=\"notes\""), "{body}");
    let title_index = body
        .find(&title_href)
        .unwrap_or_else(|| panic!("title href: {body}"));
    let notes_index = body
        .find(&notes_href)
        .unwrap_or_else(|| panic!("notes href: {body}"));
    assert!(title_index < notes_index, "{body}");

    // Checklist text matches through diacritic folding.
    let (_, _, body) = raw_request(&h, "GET", "/search?q=cafe").await;
    assert!(body.contains("data-matched-in=\"checklist\""), "{body}");
    assert!(body.contains("data-testid=\"field-hit\""), "{body}");

    // Logged/completed content remains searchable.
    let (_, _, body) = raw_request(&h, "GET", "/search?q=errand").await;
    assert!(body.contains(&format!("%2Ftodos%2F{errand}")), "{body}");
}

#[tokio::test]
async fn opening_a_destination_records_recents_and_boosts_it() {
    let h = harness().await;
    let project = create_project(&h, "Roadmap").await;
    let url = format!("/projects/{project}");

    let (status, location, _) = raw_request(
        &h,
        "GET",
        &format!("/search/go?url=%2Fprojects%2F{project}"),
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some(url.as_str()));
    assert_eq!(
        h.recents.list(h.account_id).await.expect("recents"),
        vec![url.clone()]
    );

    // External and protocol-relative URLs are refused without recording.
    let (status, location, _) =
        raw_request(&h, "GET", "/search/go?url=https%3A%2F%2Fevil.example").await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/search"));
    assert_eq!(
        h.recents.list(h.account_id).await.expect("recents").len(),
        1
    );

    // The recent destination is marked and ranked above an equal-confidence match.
    create_project(&h, "Roadmap archive").await;
    let (_, _, body) = raw_request(&h, "GET", "/search?q=roadmap").await;
    assert!(body.contains("data-testid=\"recent-hit\""), "{body}");
    let recent_index = body.find("Recent").expect("recent badge");
    let other_index = body.find("Roadmap archive").expect("second project");
    assert!(recent_index < other_index, "{body}");
}
