#![cfg(feature = "ssr")]

//! Tests for the responsive layout foundation.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use momentum_app::session::{AccountSession, SessionState};
use momentum_domain::AccountId;
use momentum_web::responsive::{LayoutMode, DESKTOP_MEDIA, TABLET_MEDIA};
use momentum_web::session::FixedSessionResolver;
use tower::ServiceExt;

const APP_CSS: &str = include_str!("../assets/app.css");

async fn get_authenticated(uri: &str) -> String {
    let session = SessionState::Authenticated(AccountSession::new(
        AccountId::new(),
        "responsive@example.com",
    ));
    let options = momentum_web::server::leptos_options();
    let router =
        momentum_web::server::router_with_session(options, Arc::new(FixedSessionResolver(session)));

    let response = router
        .oneshot(
            Request::builder()
                .uri(uri)
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
    String::from_utf8(bytes.to_vec()).expect("utf-8")
}

#[test]
fn layout_mode_boundaries_are_documented() {
    assert_eq!(LayoutMode::from_width(767), LayoutMode::Mobile);
    assert_eq!(LayoutMode::from_width(768), LayoutMode::Tablet);
    assert_eq!(LayoutMode::from_width(1023), LayoutMode::Tablet);
    assert_eq!(LayoutMode::from_width(1024), LayoutMode::Desktop);
}

#[tokio::test]
async fn shell_renders_responsive_navigation_regions() {
    let body = get_authenticated("/today").await;

    assert!(
        body.contains("id=\"app-sidebar\""),
        "sidebar region missing"
    );
    assert!(
        body.contains("aria-controls=\"app-sidebar\""),
        "toggle must control the sidebar"
    );
    assert!(
        body.contains("aria-label=\"Toggle navigation\""),
        "toggle must have an accessible name"
    );
    assert!(body.contains("class=\"bottom-nav\""), "bottom bar missing");
    assert_eq!(
        body.matches("<main").count(),
        1,
        "exactly one main landmark expected"
    );
    assert!(
        body.contains("data-layout=\"desktop\""),
        "SSR default layout mode must be desktop"
    );
    assert!(
        body.contains("aria-expanded=\"false\""),
        "drawer starts closed"
    );
    assert!(
        body.contains("class=\"drawer-scrim\""),
        "scrim is rendered for the drawer state"
    );
}

#[tokio::test]
async fn every_primary_destination_is_reachable_in_all_modes() {
    let body = get_authenticated("/inbox").await;
    for href in [
        "/inbox",
        "/today",
        "/upcoming",
        "/anytime",
        "/someday",
        "/logbook",
    ] {
        assert!(body.contains(&format!("href=\"{href}\"")), "missing {href}");
    }
    for href in ["/projects", "/areas", "/settings"] {
        assert!(body.contains(&format!("href=\"{href}\"")), "missing {href}");
    }
}

#[test]
fn stylesheet_defines_breakpoints_and_touch_targets() {
    assert!(APP_CSS.contains(TABLET_MEDIA), "tablet media query missing");
    assert!(
        APP_CSS.contains(DESKTOP_MEDIA),
        "desktop media query missing"
    );
    assert!(
        APP_CSS.contains("--touch-target"),
        "touch target token missing"
    );
    assert!(
        APP_CSS.contains("prefers-reduced-motion"),
        "drawer motion must respect reduced motion"
    );
    assert!(
        APP_CSS.contains("data-drawer=\"open\""),
        "drawer open state must be styleable"
    );
}
