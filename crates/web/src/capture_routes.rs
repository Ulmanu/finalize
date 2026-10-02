//! Browser/PWA capture endpoints (share target and bookmark adapter boundary).
//!
//! The documented command URL is `GET/POST /capture` with `title`, `url`, and `text` parameters:
//! a PWA share target or an optional browser extension/bookmarklet posts here, and all domain
//! logic lives in the application layer.

use axum::extract::{Extension, Query, State};
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Form, Router};
use momentum_app::capture::build_capture;
use momentum_app::session::SessionState;
use serde::Deserialize;

use crate::server::AppState;

/// Capture routes.
pub fn routes() -> Router<AppState> {
    Router::new().route("/capture", get(capture_get).post(capture_post))
}

#[derive(Debug, Deserialize)]
struct CaptureQuery {
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    return_to: Option<String>,
}

fn redirect(to: &str) -> Response {
    let mut response = StatusCode::SEE_OTHER.into_response();
    if let Ok(location) = HeaderValue::from_str(to) {
        response.headers_mut().insert(header::LOCATION, location);
    }
    response
}

async fn capture_get(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Query(query): Query<CaptureQuery>,
) -> Response {
    capture(&state, &session, query).await
}

async fn capture_post(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Form(form): Form<CaptureQuery>,
) -> Response {
    capture(&state, &session, form).await
}

async fn capture(state: &AppState, session: &SessionState, form: CaptureQuery) -> Response {
    let Some(service) = state.todos.clone() else {
        return redirect("/login");
    };
    let Some(account) = session.account() else {
        return redirect("/login");
    };
    let target = form
        .return_to
        .as_deref()
        .filter(|value| value.starts_with('/'))
        .unwrap_or("/inbox")
        .to_string();
    let (title, notes) = build_capture(
        form.title.as_deref().unwrap_or_default(),
        form.url.as_deref(),
        form.text.as_deref(),
    );
    let notes = (!notes.is_empty()).then_some(notes);
    match service
        .create_in(
            account.account_id,
            &title,
            notes,
            momentum_domain::model::ContextRef::Inbox,
        )
        .await
    {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=capture")),
    }
}
