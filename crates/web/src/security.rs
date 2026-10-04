//! Security hardening: response headers, origin checks for mutating requests, and login
//! throttling.
//!
//! Sessions are HttpOnly + SameSite=Lax cookies (so cross-site POSTs cannot carry the session),
//! every response carries a restrictive CSP and related headers, mutating requests with a
//! foreign `Origin` are rejected, and repeated failed logins are throttled per account.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use axum::extract::Request;
use axum::http::{header, HeaderValue, Method, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

/// Content-Security-Policy applied to every response.
///
/// The hydration bootstrap is inline and per-render, so `script-src` allows inline scripts for
/// now (documented gap: a nonce-based policy arrives with the hydration harness); no external
/// origins are permitted anywhere.
pub const CSP: &str = "default-src 'self'; script-src 'self' 'unsafe-inline' 'wasm-unsafe-eval'; \
style-src 'self'; img-src 'self' data:; connect-src 'self'; object-src 'none'; \
base-uri 'self'; form-action 'self'; frame-ancestors 'none'";

/// Adds security headers to every response.
pub async fn headers(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(CSP),
    );
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    headers.insert(header::X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));
    headers.insert(
        header::HeaderName::from_static("permissions-policy"),
        HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
    );
    response
}

/// Rejects mutating requests whose `Origin` does not match the request host.
///
/// Requests without an `Origin` header (same-origin form posts in some browsers, non-browser
/// automation, and the mail provider webhook) pass; the check is defense-in-depth on top of the
/// SameSite=Lax session cookie.
pub async fn origin_guard(request: Request, next: Next) -> Response {
    let mutating = matches!(
        *request.method(),
        Method::POST | Method::PUT | Method::PATCH | Method::DELETE
    );
    if mutating {
        if let Some(origin) = request
            .headers()
            .get(header::ORIGIN)
            .and_then(|value| value.to_str().ok())
        {
            let origin_host = origin
                .split("://")
                .nth(1)
                .and_then(|rest| rest.split('/').next())
                .unwrap_or_default()
                .to_ascii_lowercase();
            let host = request
                .headers()
                .get(header::HOST)
                .and_then(|value| value.to_str().ok())
                .unwrap_or_default()
                .to_ascii_lowercase();
            if origin_host != host {
                return StatusCode::FORBIDDEN.into_response();
            }
        }
    }
    next.run(request).await
}

/// Per-account login attempt throttle.
#[derive(Debug)]
pub struct LoginThrottle {
    attempts: Mutex<HashMap<String, Vec<Instant>>>,
    window: Duration,
    max_attempts: usize,
}

impl Default for LoginThrottle {
    fn default() -> Self {
        Self::new(Duration::from_secs(15 * 60), 10)
    }
}

impl LoginThrottle {
    /// Creates a throttle with an explicit window and attempt budget.
    #[must_use]
    pub fn new(window: Duration, max_attempts: usize) -> Self {
        Self {
            attempts: Mutex::new(HashMap::new()),
            window,
            max_attempts,
        }
    }

    /// Records an attempt, returning whether it is allowed.
    pub fn check_and_record(&self, key: &str) -> bool {
        let key = key.trim().to_ascii_lowercase();
        let now = Instant::now();
        let mut attempts = self.attempts.lock().expect("throttle lock");
        let window = attempts.entry(key).or_default();
        window.retain(|at| now.duration_since(*at) < self.window);
        if window.len() >= self.max_attempts {
            return false;
        }
        window.push(now);
        true
    }

    /// Clears the record for an account after a successful login.
    pub fn clear(&self, key: &str) {
        self.attempts
            .lock()
            .expect("throttle lock")
            .remove(&key.trim().to_ascii_lowercase());
    }
}

#[cfg(test)]
mod tests {
    use super::LoginThrottle;
    use std::time::Duration;

    #[test]
    fn throttling_blocks_after_the_budget_and_clears_on_success() {
        let throttle = LoginThrottle::new(Duration::from_secs(60), 3);
        assert!(throttle.check_and_record("User@Example.com"));
        assert!(throttle.check_and_record("user@example.com"));
        assert!(throttle.check_and_record("user@example.com"));
        assert!(!throttle.check_and_record("user@example.com"));
        throttle.clear("user@example.com");
        assert!(throttle.check_and_record("user@example.com"));
        // A different account is unaffected.
        assert!(throttle.check_and_record("other@example.com"));
    }
}
