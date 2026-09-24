#![cfg(feature = "ssr")]

//! Integration tests for the application shell: routes, layouts, session boundary, and
//! boundary pages.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use momentum_app::session::{AccountSession, SessionState};
use momentum_domain::AccountId;
use momentum_web::session::FixedSessionResolver;
use tower::ServiceExt;

fn authenticated_state() -> SessionState {
    SessionState::Authenticated(AccountSession::new(
        AccountId::new(),
        "shell-test@example.com",
    ))
}

async fn get_with_session(uri: &str, session: SessionState) -> (StatusCode, String) {
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

    let status = response.status();
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("read body")
        .to_bytes();
    let body = String::from_utf8(bytes.to_vec()).expect("utf-8 body");
    (status, body)
}

async fn get_anonymous(uri: &str) -> (StatusCode, String) {
    get_with_session(uri, SessionState::Anonymous).await
}

#[tokio::test]
async fn every_builtin_list_route_renders_its_destination() {
    let cases = [
        ("/inbox", "Inbox"),
        ("/today", "Today"),
        ("/upcoming", "Upcoming"),
        ("/anytime", "Anytime"),
        ("/someday", "Someday"),
        ("/logbook", "Logbook"),
    ];

    for (uri, title) in cases {
        let (status, body) = get_with_session(uri, authenticated_state()).await;
        assert_eq!(status, StatusCode::OK, "route {uri} status");
        assert!(
            body.contains("data-testid=\"list-page\""),
            "route {uri} must render the list page: {body}"
        );
        assert!(
            body.contains(&format!(">{title}</h1>")),
            "route {uri} must render heading {title}: {body}"
        );
        assert!(
            body.contains("data-testid=\"app-layout\""),
            "route {uri} must render the authenticated layout"
        );
    }
}

#[tokio::test]
async fn entity_routes_are_addressable() {
    let area_id = AccountId::new().to_string();
    let (status, body) =
        get_with_session(&format!("/areas/{area_id}"), authenticated_state()).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("data-testid=\"entity-page\""));
    assert!(body.contains(&area_id));

    let project_id = AccountId::new().to_string();
    let (status, body) =
        get_with_session(&format!("/projects/{project_id}"), authenticated_state()).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains(&project_id));

    let todo_id = AccountId::new().to_string();
    let (status, body) =
        get_with_session(&format!("/todos/{todo_id}"), authenticated_state()).await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("data-testid=\"todo-editor\"")
            || body.contains("data-testid=\"todo-not-found\""),
        "to-do route must render the editor or its not-found state"
    );
}

#[tokio::test]
async fn anonymous_protected_route_renders_unauthorized_boundary() {
    let (status, body) = get_anonymous("/today").await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("data-testid=\"unauthorized\""),
        "anonymous request must render the unauthorized boundary: {body}"
    );
    assert!(
        !body.contains("data-testid=\"app-layout\""),
        "anonymous request must not render the authenticated layout"
    );
    assert!(
        !body.contains("data-testid=\"list-page\""),
        "anonymous request must not render task content"
    );
    assert!(body.contains("/login"), "boundary must link to sign-in");
}

#[tokio::test]
async fn unknown_route_renders_not_found_boundary() {
    let (status, body) = get_anonymous("/this/does/not/exist").await;
    assert!(
        status == StatusCode::OK || status == StatusCode::NOT_FOUND,
        "unexpected status {status}"
    );
    assert!(
        body.contains("data-testid=\"not-found\""),
        "unknown route must render the not-found boundary: {body}"
    );
}

#[tokio::test]
async fn landing_page_renders_for_anonymous_users() {
    let (status, body) = get_anonymous("/").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("data-testid=\"landing-page\""));
    assert!(body.contains("data-testid=\"public-layout\""));
}

#[tokio::test]
async fn ssr_embeds_session_bootstrap_for_hydration() {
    let (_, body) = get_with_session("/today", authenticated_state()).await;
    assert!(
        body.contains("id=\"momentum-session-bootstrap\""),
        "SSR must embed the session bootstrap element"
    );
    assert!(
        body.contains("shell-test@example.com"),
        "bootstrap must carry the resolved session"
    );
    assert!(
        body.contains("application/json"),
        "bootstrap must be a JSON element"
    );
}

#[tokio::test]
async fn stylesheet_is_served_from_rust_embedded_assets() {
    let (status, body) = get_anonymous("/static/app.css").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains(".app-layout"), "stylesheet content expected");
    assert!(
        body.contains("var(--color-accent)"),
        "layout must use design tokens"
    );
}

#[test]
fn loading_boundary_is_a_polite_live_region() {
    use leptos::prelude::*;

    let html = view! { <momentum_web::boundaries::Loading/> }.to_html();
    assert!(html.contains("role=\"status\""), "{html}");
    assert!(html.contains("aria-live=\"polite\""), "{html}");
    assert!(html.contains("data-testid=\"loading\""), "{html}");
}

#[test]
fn error_view_exposes_alert_and_recovery_link() {
    use leptos::prelude::*;
    use leptos::reactive::owner::Owner;

    let html = Owner::new()
        .with(|| view! { <momentum_web::boundaries::ErrorView message="boom"/> }.to_html());
    assert!(html.contains("role=\"alert\""), "{html}");
    assert!(html.contains("data-testid=\"error-boundary\""), "{html}");
    assert!(html.contains("href=\"/today\""), "{html}");
}

#[test]
fn unauthorized_boundary_links_to_sign_in() {
    use leptos::prelude::*;

    let html = view! { <momentum_web::boundaries::Unauthorized/> }.to_html();
    assert!(html.contains("data-testid=\"unauthorized\""), "{html}");
    assert!(html.contains("href=\"/login\""), "{html}");
}

#[test]
fn not_found_boundary_links_back_to_today() {
    use leptos::prelude::*;

    let html = view! { <momentum_web::boundaries::NotFound/> }.to_html();
    assert!(html.contains("data-testid=\"not-found\""), "{html}");
    assert!(html.contains("href=\"/today\""), "{html}");
}
