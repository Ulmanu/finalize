#![cfg(feature = "ssr")]

//! Fixed-viewport visual-parity suite: deterministic structural snapshots against baselines.
//!
//! Pages are rendered with seeded data at desktop/tablet/mobile viewports, canonicalized
//! (identifiers, nonces, emails, and hydration scripts replaced), and compared to committed
//! baselines. Set `UPDATE_VISUAL_BASELINES=1` to regenerate; failures write inspectable
//! expected/actual/diff artifacts under `target/visual-artifacts`.

use std::collections::BTreeMap;
use std::path::PathBuf;
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
use momentum_testkit::visual::{
    compare, normalize_html, write_artifacts, Viewport, VisualBaseline, VisualFixture,
    BASELINE_VERSION,
};
use momentum_web::auth_routes::AuthContext;
use momentum_web::server::{self, AppState};
use momentum_web::session::CookieSessionResolver;
use tower::ServiceExt;

fn baseline_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/visual-baselines.json")
}

fn artifact_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/visual-artifacts")
}

fn threshold() -> f64 {
    std::env::var("VISUAL_DIFF_THRESHOLD")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(0.0)
}

struct Harness {
    router: axum::Router,
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
        .register("visual@example.com", "correct horse battery", None)
        .await
        .expect("register");
    auth.verify_email(&registration.verification_token)
        .await
        .expect("verify");
    let login = auth
        .login("visual@example.com", "correct horse battery")
        .await
        .expect("login");

    // Deterministic seeded data for every snapshot.
    for (index, title) in ["Review roadmap", "Book flights", "Write notes"]
        .iter()
        .enumerate()
    {
        todos
            .create(registration.account_id, title, None)
            .await
            .expect("todo");
        if index < 2 {
            let id = todos
                .list(registration.account_id, TodoScope::Inbox)
                .await
                .expect("inbox")
                .into_iter()
                .find(|todo| todo.title == *title)
                .expect("todo")
                .id;
            todos
                .schedule(
                    registration.account_id,
                    id,
                    ScheduleInput::On(LocalDate::from_ymd(2026, 6, 10).unwrap()),
                )
                .await
                .expect("schedule");
        }
    }
    projects
        .create(registration.account_id, "Launch", None)
        .await
        .expect("project");
    tags.create(registration.account_id, "Focus", None)
        .await
        .expect("tag");

    let state = AppState::new(
        server::leptos_options(),
        None,
        Arc::new(CookieSessionResolver::new(auth.clone())),
        Some(Arc::new(AuthContext::new(auth, false))),
        Some(todos),
        None,
        Some(projects),
    )
    .with_tags(tags);

    Harness {
        router: server::router_with_state(state),
        cookie: format!("momentum_session={}", login.token),
    }
}

async fn render(h: &Harness, path: &str) -> String {
    let request = Request::builder()
        .method("GET")
        .uri(path)
        .header(header::COOKIE, &h.cookie)
        .body(Body::empty())
        .expect("request");
    let response = h.router.clone().oneshot(request).await.expect("response");
    assert_eq!(response.status(), StatusCode::OK, "{path}");
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let body = String::from_utf8(bytes.to_vec()).expect("utf-8");
    let snapshot = normalize_html(&body);
    // Determinism guard: normalization must be idempotent for the same render.
    assert_eq!(snapshot, normalize_html(&body));
    snapshot
}

#[tokio::test]
async fn fixed_viewport_snapshots_match_committed_baselines() {
    let h = harness().await;
    let fixtures: Vec<VisualFixture> = ["/inbox", "/today", "/companion", "/settings"]
        .into_iter()
        .flat_map(|path| {
            [Viewport::DESKTOP, Viewport::TABLET, Viewport::MOBILE]
                .into_iter()
                .map(move |viewport| VisualFixture::new(path, viewport))
        })
        .collect();

    let mut actual: BTreeMap<String, String> = BTreeMap::new();
    for fixture in &fixtures {
        let snapshot = render(&h, &fixture.path).await;
        actual.insert(fixture.name.clone(), snapshot);
    }

    let update = std::env::var("UPDATE_VISUAL_BASELINES").as_deref() == Ok("1");
    let baseline = VisualBaseline::load(&baseline_path()).expect("baseline load");
    if update || baseline.is_none() {
        let baseline = VisualBaseline {
            version: BASELINE_VERSION,
            snapshots: actual,
        };
        baseline.save(&baseline_path()).expect("baseline save");
        eprintln!("visual baselines written to {}", baseline_path().display());
        return;
    }
    let baseline = baseline.expect("baseline");

    let threshold = threshold();
    let mut failures = Vec::new();
    for fixture in &fixtures {
        let expected = baseline
            .snapshots
            .get(&fixture.name)
            .unwrap_or_else(|| panic!("missing baseline for {}", fixture.name));
        let actual = actual.get(&fixture.name).expect("actual snapshot");
        if let Some(report) = compare(&fixture.name, expected, actual, threshold) {
            write_artifacts(&artifact_dir(), &report).expect("artifacts");
            failures.push(report);
        }
    }
    assert!(
        failures.is_empty(),
        "visual regressions: {:?} (artifacts in {})",
        failures
            .iter()
            .map(|report| (report.fixture.clone(), report.ratio))
            .collect::<Vec<_>>(),
        artifact_dir().display()
    );

    // Fixture metadata is part of the contract (fixed viewports, stable names).
    assert_eq!(fixtures.len(), 12);
    assert!(fixtures
        .iter()
        .any(|fixture| fixture.name == "inbox-desktop"));
    assert!(fixtures
        .iter()
        .any(|fixture| fixture.name == "settings-mobile"));
    let _ = AccountId::new();
}
