#![cfg(feature = "ssr")]

//! Authentication route tests: forms, login/logout, cookie sessions, and isolation.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use momentum_app::auth::AuthService;
use momentum_testkit::auth::{
    DeterministicHasher, DeterministicTokenGenerator, MemoryAuthStore, TestClock,
};
use momentum_web::auth_routes::AuthContext;
use momentum_web::server::{self, AppState};
use momentum_web::session::CookieSessionResolver;
use tower::ServiceExt;

struct Harness {
    router: axum::Router,
    service: Arc<AuthService>,
    store: Arc<MemoryAuthStore>,
}

fn harness() -> Harness {
    let store = Arc::new(MemoryAuthStore::new());
    let clock = Arc::new(TestClock::new(
        Utc.with_ymd_and_hms(2026, 1, 1, 9, 0, 0).unwrap(),
    ));
    let service = Arc::new(AuthService::new(
        store.clone(),
        store.clone(),
        store.clone(),
        Arc::new(DeterministicHasher),
        Arc::new(DeterministicTokenGenerator::new()),
        clock,
    ));
    let resolver = Arc::new(CookieSessionResolver::new(service.clone()));
    let state = AppState::new(
        server::leptos_options(),
        None,
        resolver,
        Some(Arc::new(AuthContext::new(service.clone(), false))),
        None,
        None,
        None,
    );
    Harness {
        router: server::router_with_state(state),
        service,
        store,
    }
}

async fn request(
    router: &axum::Router,
    method: &str,
    uri: &str,
    body: Option<&str>,
    cookie: Option<&str>,
) -> (StatusCode, axum::http::HeaderMap, String) {
    let mut builder = Request::builder().method(method).uri(uri);
    if body.is_some() {
        builder = builder.header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");
    }
    if let Some(cookie) = cookie {
        builder = builder.header(header::COOKIE, cookie);
    }
    let request = builder
        .body(body.map_or_else(Body::empty, |body| Body::from(body.to_string())))
        .expect("request");

    let response = router.clone().oneshot(request).await.expect("response");
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

async fn verified_account(h: &Harness, email: &str, password: &str) -> String {
    let outcome = h
        .service
        .register(email, password, None)
        .await
        .expect("register");
    h.service
        .verify_email(&outcome.verification_token)
        .await
        .expect("verify");
    h.store
        .account_by_id(outcome.account_id)
        .expect("account")
        .email
}

fn cookie_value(headers: &axum::http::HeaderMap) -> String {
    headers
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .expect("set-cookie")
        .split(';')
        .next()
        .expect("cookie pair")
        .to_string()
}

#[tokio::test]
async fn login_page_renders_accessible_form() {
    let h = harness();
    let (status, _, body) = request(&h.router, "GET", "/login", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("data-testid=\"login-page\""));
    assert!(body.contains("action=\"/auth/login\""));
    assert!(body.contains("type=\"email\""));
    assert!(body.contains("autocomplete=\"current-password\""));
    assert!(body.contains("aria-label=\"Password\""));
}

#[tokio::test]
async fn signup_page_renders_accessible_form() {
    let h = harness();
    let (status, _, body) = request(&h.router, "GET", "/signup", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("data-testid=\"signup-page\""));
    assert!(body.contains("action=\"/auth/register\""));
    assert!(body.contains("autocomplete=\"new-password\""));
    assert!(body.contains("minlength=\"8\""));
}

#[tokio::test]
async fn registration_creates_an_unverified_account() {
    let h = harness();
    let (status, headers, _) = request(
        &h.router,
        "POST",
        "/auth/register",
        Some("email=new%40example.com&password=correct+horse+battery"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(
        headers.get(header::LOCATION).and_then(|v| v.to_str().ok()),
        Some("/login?registered=1")
    );

    let account = h
        .store
        .account_by_email("new@example.com")
        .expect("account");
    assert!(!account.is_verified());
    assert!(account
        .password_hash
        .expect("hash")
        .starts_with("test-hash:"));
}

#[tokio::test]
async fn login_sets_a_hardened_cookie_and_grants_access() {
    let h = harness();
    let email = verified_account(&h, "person@example.com", "correct horse battery").await;

    let (status, headers, _) = request(
        &h.router,
        "POST",
        "/auth/login",
        Some("email=person%40example.com&password=correct+horse+battery"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(
        headers.get(header::LOCATION).and_then(|v| v.to_str().ok()),
        Some("/today")
    );
    let set_cookie = headers
        .get(header::SET_COOKIE)
        .and_then(|v| v.to_str().ok())
        .expect("cookie");
    assert!(set_cookie.contains("HttpOnly"));
    assert!(set_cookie.contains("SameSite=Lax"));
    assert!(!set_cookie.contains("Secure"), "test harness is plain HTTP");

    let cookie = cookie_value(&headers);
    let (status, _, body) = request(&h.router, "GET", "/today", None, Some(&cookie)).await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("data-testid=\"list-page\""),
        "must render the list"
    );
    assert!(body.contains(&email), "bootstrap must carry the session");
}

#[tokio::test]
async fn invalid_credentials_are_uniform_and_do_not_set_cookies() {
    let h = harness();
    verified_account(&h, "person@example.com", "correct horse battery").await;

    for body in [
        "email=person%40example.com&password=wrong+password+value",
        "email=nobody%40example.com&password=wrong+password+value",
    ] {
        let (status, headers, _) =
            request(&h.router, "POST", "/auth/login", Some(body), None).await;
        assert_eq!(status, StatusCode::SEE_OTHER);
        assert_eq!(
            headers.get(header::LOCATION).and_then(|v| v.to_str().ok()),
            Some("/login?error=invalid")
        );
        assert!(headers.get(header::SET_COOKIE).is_none());
    }
}

#[tokio::test]
async fn unverified_login_is_reported_without_granting_access() {
    let h = harness();
    h.service
        .register("person@example.com", "correct horse battery", None)
        .await
        .expect("register");

    let (status, headers, _) = request(
        &h.router,
        "POST",
        "/auth/login",
        Some("email=person%40example.com&password=correct+horse+battery"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(
        headers.get(header::LOCATION).and_then(|v| v.to_str().ok()),
        Some("/login?error=unverified")
    );
    assert!(headers.get(header::SET_COOKIE).is_none());
}

#[tokio::test]
async fn logout_revokes_the_session_and_clears_the_cookie() {
    let h = harness();
    verified_account(&h, "person@example.com", "correct horse battery").await;
    let (_, headers, _) = request(
        &h.router,
        "POST",
        "/auth/login",
        Some("email=person%40example.com&password=correct+horse+battery"),
        None,
    )
    .await;
    let cookie = cookie_value(&headers);

    let (status, headers, _) =
        request(&h.router, "POST", "/auth/logout", None, Some(&cookie)).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let cleared = headers
        .get(header::SET_COOKIE)
        .and_then(|v| v.to_str().ok())
        .expect("clearing cookie");
    assert!(cleared.contains("Max-Age=0"));

    let (status, _, body) = request(&h.router, "GET", "/today", None, Some(&cookie)).await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("data-testid=\"unauthorized\""),
        "revoked session must not grant access"
    );
}

#[tokio::test]
async fn unknown_cookie_is_anonymous() {
    let h = harness();
    let (status, _, body) = request(
        &h.router,
        "GET",
        "/today",
        None,
        Some("momentum_session=not-a-real-token"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("data-testid=\"unauthorized\""));
}

#[tokio::test]
async fn sessions_are_isolated_between_accounts() {
    let h = harness();
    verified_account(&h, "first@example.com", "first account password").await;
    verified_account(&h, "second@example.com", "second account password").await;

    let (_, headers, _) = request(
        &h.router,
        "POST",
        "/auth/login",
        Some("email=first%40example.com&password=first+account+password"),
        None,
    )
    .await;
    let first_cookie = cookie_value(&headers);

    let (_, _, body) = request(&h.router, "GET", "/today", None, Some(&first_cookie)).await;
    assert!(
        body.contains("first@example.com"),
        "session A email expected"
    );
    assert!(
        !body.contains("second@example.com"),
        "session A must never expose account B"
    );
}

#[tokio::test]
async fn password_reset_request_is_uniform() {
    let h = harness();
    verified_account(&h, "person@example.com", "correct horse battery").await;

    for email in ["person%40example.com", "nobody%40example.com"] {
        let (status, headers, _) = request(
            &h.router,
            "POST",
            "/auth/password-reset",
            Some(&format!("email={email}")),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::SEE_OTHER);
        assert_eq!(
            headers.get(header::LOCATION).and_then(|v| v.to_str().ok()),
            Some("/login?reset=sent")
        );
    }
}

#[tokio::test]
async fn password_reset_confirm_updates_and_revokes() {
    let h = harness();
    let account_id = h
        .service
        .register("person@example.com", "correct horse battery", None)
        .await
        .expect("register")
        .account_id;
    let verification = h
        .service
        .request_password_reset("person@example.com")
        .await
        .expect("request")
        .expect("token");
    let _ = account_id;

    let body = format!(
        "token={}&password=a+brand+new+password",
        verification.reset_token
    );
    let (status, headers, _) = request(
        &h.router,
        "POST",
        "/auth/password-reset/confirm",
        Some(&body),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(
        headers.get(header::LOCATION).and_then(|v| v.to_str().ok()),
        Some("/login?reset=done")
    );

    let (_, headers, _) = request(
        &h.router,
        "POST",
        "/auth/login",
        Some("email=person%40example.com&password=a+brand+new+password"),
        None,
    )
    .await;
    assert!(
        headers.get(header::SET_COOKIE).is_some(),
        "new password must sign in"
    );
}

#[tokio::test]
async fn auth_routes_report_unavailable_without_auth_context() {
    let options = server::leptos_options();
    let router = server::router(options);
    let (status, headers, _) = request(
        &router,
        "POST",
        "/auth/login",
        Some("email=person%40example.com&password=correct+horse+battery"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(
        headers.get(header::LOCATION).and_then(|v| v.to_str().ok()),
        Some("/login?error=unavailable")
    );
}
