#![cfg(feature = "ssr")]

//! Tests for the design system: tokens, primitives, original assets, and the style guide.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use leptos::prelude::*;
use leptos::reactive::owner::Owner;
use momentum_app::session::SessionState;
use momentum_web::design::{MotionKind, ALL_TOKENS};
use momentum_web::session::FixedSessionResolver;
use tower::ServiceExt;

const DESIGN_CSS: &str = include_str!("../assets/design.css");

macro_rules! render {
    ($($tokens:tt)*) => {
        Owner::new().with(|| view! { $($tokens)* }.to_html())
    };
}

#[test]
fn rust_tokens_match_stylesheet_tokens() {
    let mut checked = 0;
    for group in ALL_TOKENS {
        for token in *group {
            let prefix = format!("{}:", token.name);
            let line = DESIGN_CSS
                .lines()
                .map(str::trim)
                .find(|line| line.starts_with(&prefix))
                .unwrap_or_else(|| panic!("token {} missing from design.css", token.name));
            let value = line
                .trim_start_matches(&prefix)
                .trim()
                .trim_end_matches(';')
                .trim();
            assert_eq!(
                value, token.value,
                "token {} differs between Rust and CSS",
                token.name
            );
            checked += 1;
        }
    }
    assert!(
        checked >= 40,
        "expected a substantial token set, got {checked}"
    );
}

#[test]
fn task_row_renders_checkbox_title_and_metadata() {
    let html = render!(
        <momentum_web::design::TaskRow
            title="Write the report"
            note="Include the appendix"
            tags=vec!["Work".to_string()]
            schedule="Today".to_string()
            deadline="Friday".to_string()
            has_recurrence=true
        />
    );

    assert!(html.contains("data-testid=\"task-row\""), "{html}");
    assert!(html.contains("type=\"checkbox\""), "{html}");
    assert!(html.contains("Write the report"), "{html}");
    assert!(html.contains("note-indicator"), "{html}");
    assert!(html.contains("date-badge-deadline"), "{html}");
    assert!(html.contains("tag-chip"), "{html}");
    assert!(html.contains("Repeats"), "{html}");
}

#[test]
fn completed_task_row_is_visually_distinct() {
    let html = render!( <momentum_web::design::TaskRow title="Done" completed=true/> );
    assert!(html.contains("task-row-completed"), "{html}");
    assert!(html.contains("checked"), "{html}");
}

#[test]
fn project_row_renders_derived_progress() {
    let html = render!(
        <momentum_web::design::ProjectRow title="Launch" open_count=4 progress=0.5/>
    );
    assert!(html.contains("data-testid=\"project-row\""), "{html}");
    assert!(html.contains("data-testid=\"progress-ring\""), "{html}");
    assert!(html.contains("50% complete"), "{html}");
    assert!(html.contains("4 open"), "{html}");
}

#[test]
fn progress_ring_clamps_and_labels() {
    let html = render!( <momentum_web::design::ProgressRing progress=1.5/> );
    assert!(html.contains("100% complete"), "{html}");
    assert!(html.contains("data-progress=\"1.00\""), "{html}");
}

#[test]
fn sidebar_row_is_a_link_with_active_state() {
    let html = render!(
        <momentum_web::design::SidebarRow
            label="Today"
            icon=momentum_web::design::SidebarIcon::Today
            count=3
            active=true
            href="/today".to_string()
        />
    );
    assert!(html.contains("href=\"/today\""), "{html}");
    assert!(html.contains("aria-current=\"page\""), "{html}");
    assert!(html.contains("data-active=\"true\""), "{html}");
    assert!(html.contains(">3</span>"), "{html}");
}

#[test]
fn check_control_exposes_native_semantics() {
    let html = render!( <momentum_web::design::CheckControl checked=true label="Buy milk"/> );
    assert!(html.contains("type=\"checkbox\""), "{html}");
    assert!(html.contains("aria-label=\"Buy milk\""), "{html}");
}

#[test]
fn popover_has_expanded_state_and_panel() {
    let html = render!(
        <momentum_web::design::Popover id="test-popover" trigger_label="When">
            <p>"Today"</p>
        </momentum_web::design::Popover>
    );
    assert!(html.contains("aria-expanded=\"false\""), "{html}");
    assert!(
        html.contains("aria-controls=\"test-popover-panel\""),
        "{html}"
    );
    assert!(html.contains("id=\"test-popover-panel\""), "{html}");
}

#[test]
fn context_menu_exposes_menu_semantics() {
    use momentum_web::design::ContextMenuItem;

    let html = render!(
        <momentum_web::design::ContextMenu
            label="Actions"
            items=vec![
                ContextMenuItem::new("Complete"),
                ContextMenuItem::new("Delete").danger().disabled(),
            ]
        />
    );
    assert!(html.contains("role=\"menu\""), "{html}");
    assert_eq!(html.matches("role=\"menuitem\"").count(), 2, "{html}");
    assert!(html.contains("context-menu-item-danger"), "{html}");
    assert!(html.contains("disabled"), "{html}");
}

#[test]
fn date_and_add_controls_have_accessible_names() {
    let html = render!(
        <div>
            <momentum_web::design::DateControl label="Schedule" value="Today"/>
            <momentum_web::design::DateControl label="Deadline" value="Friday" deadline=true/>
            <momentum_web::design::AddControl label="Add to-do"/>
        </div>
    );
    assert!(html.contains("aria-label=\"Schedule\""), "{html}");
    assert!(html.contains("date-control-deadline"), "{html}");
    assert!(html.contains("aria-label=\"Add to-do\""), "{html}");
}

#[test]
fn motion_wrappers_expose_centralized_primitives() {
    for kind in [
        MotionKind::Insert,
        MotionKind::Remove,
        MotionKind::Editor,
        MotionKind::Popover,
        MotionKind::Drag,
        MotionKind::Navigation,
    ] {
        let html =
            render!( <momentum_web::design::Motion kind=kind>"x"</momentum_web::design::Motion> );
        assert!(
            html.contains(kind.class()),
            "missing {}: {html}",
            kind.class()
        );
        assert!(html.contains(&format!("data-motion=\"{}\"", kind.data_value())));
    }
    assert!(DESIGN_CSS.contains("prefers-reduced-motion"));
}

#[test]
fn icons_are_inline_svg_from_this_repository() {
    let html = render!(
        <div>
            <momentum_web::design::icons::CheckIcon/>
            <momentum_web::design::icons::PlusIcon/>
        </div>
    );
    assert_eq!(html.matches("<svg").count(), 2, "{html}");
    assert!(html.contains("aria-hidden=\"true\""), "{html}");
}

#[test]
fn no_proprietary_assets_are_committed() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("web crate lives in crates/web");

    let mut offenders = Vec::new();
    let mut stack = vec![
        root.join("crates"),
        root.join("scripts"),
        root.join("assets"),
    ];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            let extension = path
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or_default()
                .to_ascii_lowercase();
            if matches!(
                extension.as_str(),
                "png" | "jpg" | "jpeg" | "gif" | "ico" | "icns" | "woff" | "woff2" | "ttf" | "otf"
            ) {
                offenders.push(format!("binary asset: {}", path.display()));
            }
        }
    }

    for offender in &offenders {
        eprintln!("{offender}");
    }
    assert!(
        offenders.is_empty(),
        "proprietary/binary assets must not be committed; icons are inline Rust SVG"
    );
}

#[tokio::test]
async fn styleguide_route_renders_the_catalog() {
    let options = momentum_web::server::leptos_options();
    let router = momentum_web::server::router_with_session(
        options,
        Arc::new(FixedSessionResolver(SessionState::Anonymous)),
    );

    let response = router
        .oneshot(
            Request::builder()
                .uri("/styleguide")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("router response");

    assert_eq!(response.status(), StatusCode::OK);
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("read body")
        .to_bytes();
    let body = String::from_utf8(bytes.to_vec()).expect("utf-8");

    for testid in [
        "task-row",
        "project-row",
        "sidebar-row",
        "check-control",
        "progress-ring",
        "section-heading",
        "popover",
        "context-menu",
        "date-control",
        "add-control",
    ] {
        assert!(
            body.contains(&format!("data-testid=\"{testid}\"")),
            "style guide must render {testid}"
        );
    }
    assert!(body.contains("/static/design.css"));
}
