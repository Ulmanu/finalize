//! Session context for the application shell.
//!
//! The shell needs to know whether a request is authenticated before choosing the public or
//! authenticated layout. Authentication itself (cookies, sessions, credentials) arrives in
//! the account-authentication change; this module defines the injection point
//! ([`SessionResolver`]) and the reactive context consumed by routes.

use std::sync::Arc;

use leptos::prelude::*;
use momentum_app::session::{AccountSession, SessionState};

/// DOM id of the JSON element that carries the server-resolved session into hydration.
pub const SESSION_BOOTSTRAP_ID: &str = "momentum-session-bootstrap";

/// Resolves the session state for an incoming request.
///
/// The account-authentication change provides the cookie-backed implementation; tests provide
/// deterministic resolvers. The default resolver treats every request as anonymous.
pub trait SessionResolver: Send + Sync + 'static {
    /// Resolves the session from request headers.
    fn resolve(&self, headers: &http::HeaderMap) -> SessionState;
}

/// Resolver used until authentication exists: every request is anonymous.
#[derive(Debug, Clone, Copy, Default)]
pub struct AnonymousSessionResolver;

impl SessionResolver for AnonymousSessionResolver {
    fn resolve(&self, _headers: &http::HeaderMap) -> SessionState {
        SessionState::Anonymous
    }
}

/// Test/diagnostic resolver that returns a fixed session state.
#[derive(Debug, Clone)]
pub struct FixedSessionResolver(pub SessionState);

impl SessionResolver for FixedSessionResolver {
    fn resolve(&self, _headers: &http::HeaderMap) -> SessionState {
        self.0.clone()
    }
}

/// Resolves the session for the current reactive owner.
///
/// Under SSR the request parts and the active resolver are read from context. Under hydration
/// the state is read from the JSON bootstrap element rendered by the server. Every other
/// configuration is anonymous.
#[must_use]
pub fn resolve_session() -> SessionState {
    #[cfg(feature = "ssr")]
    {
        let resolver = use_context::<Arc<dyn SessionResolver>>();
        let parts = use_context::<http::request::Parts>();
        if let (Some(resolver), Some(parts)) = (resolver, parts) {
            return resolver.resolve(&parts.headers);
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
        assert_eq!(
            resolver.resolve(&http::HeaderMap::new()),
            SessionState::Anonymous
        );
    }

    #[test]
    fn fixed_resolver_returns_configured_state() {
        let account = AccountSession::new(momentum_domain::AccountId::new(), "a@example.com");
        let resolver = FixedSessionResolver(SessionState::Authenticated(account.clone()));
        assert_eq!(
            resolver.resolve(&http::HeaderMap::new()),
            SessionState::Authenticated(account)
        );
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
