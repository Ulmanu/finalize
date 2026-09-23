//! Session context for the application shell.
//!
//! Session resolution requires I/O (cookie validation against the session store), so it runs
//! in Axum middleware that inserts [`SessionState`] into the request extensions. The reactive
//! context reads the resolved state from `http::request::Parts`; hydration reads the JSON
//! bootstrap element rendered by the server.

use std::sync::Arc;

use async_trait::async_trait;
use leptos::prelude::*;
use momentum_app::auth::AuthService;
use momentum_app::session::{AccountSession, SessionState};

/// Name of the session cookie.
pub const SESSION_COOKIE_NAME: &str = "momentum_session";

/// DOM id of the JSON element that carries the server-resolved session into hydration.
pub const SESSION_BOOTSTRAP_ID: &str = "momentum-session-bootstrap";

/// Resolves the session state for an incoming request.
#[async_trait]
pub trait SessionResolver: Send + Sync + 'static {
    /// Resolves the session from request headers.
    async fn resolve(&self, headers: &http::HeaderMap) -> SessionState;
}

/// Resolver used when no authentication store is configured: every request is anonymous.
#[derive(Debug, Clone, Copy, Default)]
pub struct AnonymousSessionResolver;

#[async_trait]
impl SessionResolver for AnonymousSessionResolver {
    async fn resolve(&self, _headers: &http::HeaderMap) -> SessionState {
        SessionState::Anonymous
    }
}

/// Test/diagnostic resolver that returns a fixed session state.
#[derive(Debug, Clone)]
pub struct FixedSessionResolver(pub SessionState);

#[async_trait]
impl SessionResolver for FixedSessionResolver {
    async fn resolve(&self, _headers: &http::HeaderMap) -> SessionState {
        self.0.clone()
    }
}

/// Resolves sessions from the session cookie using the authentication service.
#[derive(Clone)]
pub struct CookieSessionResolver {
    auth: Arc<AuthService>,
}

impl std::fmt::Debug for CookieSessionResolver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CookieSessionResolver")
    }
}

impl CookieSessionResolver {
    /// Creates the resolver over an auth service.
    #[must_use]
    pub fn new(auth: Arc<AuthService>) -> Self {
        Self { auth }
    }
}

#[async_trait]
impl SessionResolver for CookieSessionResolver {
    async fn resolve(&self, headers: &http::HeaderMap) -> SessionState {
        let Some(token) = session_token_from_headers(headers) else {
            return SessionState::Anonymous;
        };
        match self.auth.authenticate_session(&token).await {
            Ok(account) => SessionState::Authenticated(AccountSession {
                account_id: account.id,
                email: account.email,
                display_name: account.display_name,
            }),
            Err(_) => SessionState::Anonymous,
        }
    }
}

/// Extracts the session token from a `Cookie` header.
#[must_use]
pub fn session_token_from_headers(headers: &http::HeaderMap) -> Option<String> {
    let cookies = headers.get(http::header::COOKIE)?.to_str().ok()?;
    cookies.split(';').find_map(|pair| {
        let (name, value) = pair.split_once('=')?;
        (name.trim() == SESSION_COOKIE_NAME).then(|| value.trim().to_string())
    })
}

/// Builds the `Set-Cookie` value for a new session.
#[must_use]
pub fn session_cookie(token: &str, max_age_seconds: i64, secure: bool) -> String {
    let mut cookie = format!(
        "{SESSION_COOKIE_NAME}={token}; Path=/; HttpOnly; SameSite=Lax; Max-Age={max_age_seconds}"
    );
    if secure {
        cookie.push_str("; Secure");
    }
    cookie
}

/// Builds the `Set-Cookie` value that clears the session cookie.
#[must_use]
pub fn clear_session_cookie(secure: bool) -> String {
    let mut cookie = format!("{SESSION_COOKIE_NAME}=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0");
    if secure {
        cookie.push_str("; Secure");
    }
    cookie
}

/// Resolves the session for the current reactive owner.
///
/// Under SSR the middleware-resolved state is read from request extensions. Under hydration
/// the state is read from the JSON bootstrap element rendered by the server. Every other
/// configuration is anonymous.
#[must_use]
pub fn resolve_session() -> SessionState {
    #[cfg(feature = "ssr")]
    {
        if let Some(parts) = use_context::<http::request::Parts>() {
            if let Some(session) = parts.extensions.get::<SessionState>() {
                return session.clone();
            }
        }
        SessionState::Anonymous
    }
    #[cfg(all(not(feature = "ssr"), feature = "hydrate", target_arch = "wasm32"))]
    {
        session_from_dom().unwrap_or(SessionState::Anonymous)
    }
    #[cfg(not(any(feature = "ssr", all(feature = "hydrate", target_arch = "wasm32"))))]
    {
        SessionState::Anonymous
    }
}

/// Reads the session bootstrap element rendered into the SSR document.
#[cfg(all(feature = "hydrate", target_arch = "wasm32"))]
#[must_use]
pub fn session_from_dom() -> Option<SessionState> {
    let document = leptos::web_sys::window()?.document()?;
    let element = document.get_element_by_id(SESSION_BOOTSTRAP_ID)?;
    serde_json::from_str(&element.text_content()?).ok()
}

/// Provides the session signal used by routes and layouts.
///
/// Returns the signal so callers (and tests) can hold onto it.
#[must_use]
pub fn provide_session() -> RwSignal<SessionState> {
    let signal = RwSignal::new(resolve_session());
    provide_context(signal);
    signal
}

/// Returns the session signal from context.
///
/// # Panics
///
/// Panics when called outside the application root, which always provides the signal.
#[must_use]
pub fn use_session() -> RwSignal<SessionState> {
    use_context::<RwSignal<SessionState>>().expect("session context is provided by App")
}

/// Returns the authenticated account summary, if any.
#[must_use]
pub fn use_account() -> Option<AccountSession> {
    use_session().get().account().cloned()
}

/// Renders the session bootstrap element consumed during hydration.
#[component]
pub fn SessionBootstrap(session: SessionState) -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let json = json_for_script(&session);
        view! {
            <script
                type="application/json"
                id=SESSION_BOOTSTRAP_ID
                inner_html=json
            ></script>
        }
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = session;
        view! { <span hidden=true data-role="session-bootstrap"></span> }
    }
}

/// Serializes a session state for embedding in an HTML document.
///
/// `<` is escaped so that account data can never terminate the surrounding element.
#[cfg(feature = "ssr")]
#[must_use]
pub fn json_for_script(session: &SessionState) -> String {
    serde_json::to_string(session)
        .unwrap_or_else(|_| "{\"state\":\"anonymous\"}".to_string())
        .replace('<', "\\u003c")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anonymous_resolver_returns_anonymous() {
        let resolver = AnonymousSessionResolver;
        let state = futures_lite_block_on(resolver.resolve(&http::HeaderMap::new()));
        assert_eq!(state, SessionState::Anonymous);
    }

    fn futures_lite_block_on<F: std::future::Future>(future: F) -> F::Output {
        use std::task::{Context, Poll, Waker};

        let mut future = Box::pin(future);
        let mut context = Context::from_waker(Waker::noop());
        loop {
            match future.as_mut().poll(&mut context) {
                Poll::Ready(output) => return output,
                Poll::Pending => std::thread::yield_now(),
            }
        }
    }

    #[test]
    fn cookie_parsing_finds_the_session_token() {
        let mut headers = http::HeaderMap::new();
        headers.insert(
            http::header::COOKIE,
            http::HeaderValue::from_static("theme=dark; momentum_session=abc123; other=1"),
        );
        assert_eq!(
            session_token_from_headers(&headers).as_deref(),
            Some("abc123")
        );
    }

    #[test]
    fn cookie_parsing_returns_none_without_session() {
        let mut headers = http::HeaderMap::new();
        headers.insert(
            http::header::COOKIE,
            http::HeaderValue::from_static("theme=dark"),
        );
        assert!(session_token_from_headers(&headers).is_none());
    }

    #[test]
    fn session_cookie_is_hardened() {
        let cookie = session_cookie("token-value", 3600, true);
        assert!(cookie.contains("HttpOnly"));
        assert!(cookie.contains("SameSite=Lax"));
        assert!(cookie.contains("Secure"));
        assert!(cookie.contains("Max-Age=3600"));
        let cleared = clear_session_cookie(false);
        assert!(cleared.contains("Max-Age=0"));
        assert!(!cleared.contains("Secure"));
    }

    #[cfg(feature = "ssr")]
    #[test]
    fn bootstrap_json_escapes_angle_brackets() {
        let account = AccountSession::new(
            momentum_domain::AccountId::new(),
            "</script><script>alert(1)</script>",
        );
        let json = json_for_script(&SessionState::Authenticated(account));
        assert!(
            !json.contains('<'),
            "angle brackets must be escaped: {json}"
        );
        assert!(json.contains("\\u003c"));
    }
}
