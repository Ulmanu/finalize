//! HTTP routes for account authentication.
//!
//! Forms post to these handlers directly (progressive enhancement, no handwritten script).
//! Successful login sets an `HttpOnly` session cookie and redirects; credential failures are
//! uniform and never reveal whether an email address exists.

use std::sync::Arc;

use axum::extract::{Query, State};
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Extension, Form, Router};
use momentum_app::auth::{AuthError, AuthService};
use momentum_app::session::SessionState;
use serde::Deserialize;

use crate::server::AppState;
use crate::session::{clear_session_cookie, session_cookie, session_token_from_headers};

/// Authentication behavior shared by the handlers.
pub struct AuthContext {
    /// Account authentication service.
    pub service: Arc<AuthService>,
    /// Whether session cookies include the `Secure` attribute (HTTPS deployments).
    pub secure_cookies: bool,
}

impl std::fmt::Debug for AuthContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("AuthContext")
    }
}

impl AuthContext {
    /// Creates the context.
    #[must_use]
    pub fn new(service: Arc<AuthService>, secure_cookies: bool) -> Self {
        Self {
            service,
            secure_cookies,
        }
    }
}

/// Authentication route handlers.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/verify", get(verify_email))
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
        .route("/auth/password-reset", post(request_password_reset))
        .route("/auth/password-reset/confirm", post(confirm_password_reset))
        .route("/auth/password-change", post(change_password))
}

fn redirect(to: &str) -> Response {
    let mut response = StatusCode::SEE_OTHER.into_response();
    if let Ok(location) = HeaderValue::from_str(to) {
        response.headers_mut().insert(header::LOCATION, location);
    }
    response
}

fn redirect_with_cookie(to: &str, cookie: &str) -> Response {
    let mut response = redirect(to);
    if let Ok(value) = HeaderValue::from_str(cookie) {
        response.headers_mut().insert(header::SET_COOKIE, value);
    }
    response
}

fn auth_context(state: &AppState) -> Option<Arc<AuthContext>> {
    state.auth.clone()
}

#[derive(Debug, Deserialize)]
struct RegisterForm {
    email: String,
    password: String,
    #[serde(default)]
    display_name: Option<String>,
}

async fn register(State(state): State<AppState>, Form(form): Form<RegisterForm>) -> Response {
    let Some(auth) = auth_context(&state) else {
        return redirect("/login?error=unavailable");
    };

    match auth
        .service
        .register(&form.email, &form.password, form.display_name)
        .await
    {
        Ok(outcome) => {
            // Email delivery is not configured in this build; the token is recorded for
            // operators so verification can still be completed. Recorded as a live gap.
            tracing::info!(
                account = %outcome.account_id,
                "registration created; email delivery is not configured, verification token issued"
            );
            redirect("/login?registered=1")
        }
        Err(AuthError::EmailTaken) => redirect("/signup?error=taken"),
        Err(AuthError::InvalidEmail(_) | AuthError::WeakPassword(_)) => {
            redirect("/signup?error=invalid")
        }
        Err(error) => {
            tracing::warn!(%error, "registration failed");
            redirect("/signup?error=unavailable")
        }
    }
}

#[derive(Debug, Deserialize)]
struct VerifyQuery {
    token: String,
}

async fn verify_email(State(state): State<AppState>, Query(query): Query<VerifyQuery>) -> Response {
    let Some(auth) = auth_context(&state) else {
        return redirect("/login?error=unavailable");
    };
    match auth.service.verify_email(&query.token).await {
        Ok(_) => redirect("/login?verified=1"),
        Err(_) => redirect("/login?error=verify"),
    }
}

#[derive(Debug, Deserialize)]
struct LoginForm {
    email: String,
    password: String,
}

async fn login(State(state): State<AppState>, Form(form): Form<LoginForm>) -> Response {
    let Some(auth) = auth_context(&state) else {
        return redirect("/login?error=unavailable");
    };
    if !state.login_throttle.check_and_record(&form.email) {
        return redirect("/login?error=throttled");
    }

    match auth.service.login(&form.email, &form.password).await {
        Ok(outcome) => {
            state.login_throttle.clear(&form.email);
            let max_age = (outcome.expires_at - chrono::Utc::now())
                .num_seconds()
                .max(0);
            let cookie = session_cookie(&outcome.token, max_age, auth.secure_cookies);
            redirect_with_cookie("/today", &cookie)
        }
        Err(AuthError::EmailNotVerified) => redirect("/login?error=unverified"),
        Err(_) => redirect("/login?error=invalid"),
    }
}

async fn logout(State(state): State<AppState>, headers: axum::http::HeaderMap) -> Response {
    let Some(auth) = auth_context(&state) else {
        return redirect("/");
    };
    if let Some(token) = session_token_from_headers(&headers) {
        if let Err(error) = auth.service.logout(&token).await {
            tracing::debug!(%error, "logout with inactive session");
        }
    }
    redirect_with_cookie("/", &clear_session_cookie(auth.secure_cookies))
}

#[derive(Debug, Deserialize)]
struct EmailForm {
    email: String,
}

async fn request_password_reset(
    State(state): State<AppState>,
    Form(form): Form<EmailForm>,
) -> Response {
    let Some(auth) = auth_context(&state) else {
        return redirect("/login?error=unavailable");
    };
    match auth.service.request_password_reset(&form.email).await {
        Ok(Some(outcome)) => {
            tracing::info!(
                "password reset requested; email delivery is not configured, reset token issued"
            );
            let _ = outcome;
        }
        Ok(None) => {}
        Err(error) => {
            tracing::warn!(%error, "password reset request failed");
        }
    }
    redirect("/login?reset=sent")
}

#[derive(Debug, Deserialize)]
struct ResetForm {
    token: String,
    password: String,
}

async fn confirm_password_reset(
    State(state): State<AppState>,
    Form(form): Form<ResetForm>,
) -> Response {
    let Some(auth) = auth_context(&state) else {
        return redirect("/login?error=unavailable");
    };
    match auth
        .service
        .reset_password(&form.token, &form.password)
        .await
    {
        Ok(_) => redirect("/login?reset=done"),
        Err(_) => redirect("/login?error=reset"),
    }
}

#[derive(Debug, Deserialize)]
struct ChangePasswordForm {
    current_password: String,
    new_password: String,
}

async fn change_password(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Form(form): Form<ChangePasswordForm>,
) -> Response {
    let Some(auth) = auth_context(&state) else {
        return redirect("/login?error=unavailable");
    };
    let Some(account) = session.account() else {
        return redirect("/login");
    };

    match auth
        .service
        .change_password(
            account.account_id,
            &form.current_password,
            &form.new_password,
        )
        .await
    {
        Ok(()) => redirect_with_cookie(
            "/login?password=changed",
            &clear_session_cookie(auth.secure_cookies),
        ),
        Err(AuthError::InvalidCredentials) => redirect("/settings?error=password"),
        Err(_) => redirect("/settings?error=unavailable"),
    }
}
