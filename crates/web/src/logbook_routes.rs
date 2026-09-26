//! Logbook history, logging policy, and explicit logging routes.

use axum::extract::{Path, State};
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Extension, Form, Router};
use momentum_app::session::SessionState;
use momentum_domain::model::LoggingPolicy;
use momentum_domain::EntityId;
use serde::Deserialize;

use crate::server::AppState;

/// Logbook command routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/logbook/policy", post(set_policy))
        .route("/todos/{todo_id}/log", post(log_todo))
        .route("/projects/{project_id}/log", post(log_project))
}

fn redirect(to: &str) -> Response {
    let mut response = StatusCode::SEE_OTHER.into_response();
    if let Ok(location) = HeaderValue::from_str(to) {
        response.headers_mut().insert(header::LOCATION, location);
    }
    response
}

#[derive(Debug, Deserialize)]
struct PolicyForm {
    policy: String,
    #[serde(default)]
    return_to: Option<String>,
}

async fn set_policy(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Form(form): Form<PolicyForm>,
) -> Response {
    let Some(service) = state.logbook.clone() else {
        return redirect("/login");
    };
    let Some(account) = session.account() else {
        return redirect("/login");
    };
    let target = form
        .return_to
        .filter(|value| value.starts_with('/'))
        .unwrap_or_else(|| "/logbook".to_string());
    match service
        .set_policy(account.account_id, LoggingPolicy::parse(&form.policy))
        .await
    {
        Ok(()) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=policy")),
    }
}

#[derive(Debug, Deserialize)]
struct LogForm {
    #[serde(default)]
    return_to: Option<String>,
}

async fn log_todo(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(todo_id): Path<String>,
    Form(form): Form<LogForm>,
) -> Response {
    let Some(service) = state.logbook.clone() else {
        return redirect("/login");
    };
    let Some(account) = session.account() else {
        return redirect("/login");
    };
    let Ok(id) = todo_id.parse::<EntityId>() else {
        return redirect("/logbook?error=notfound");
    };
    let target = form
        .return_to
        .filter(|value| value.starts_with('/'))
        .unwrap_or_else(|| "/logbook".to_string());
    match service.log_todo(account.account_id, id).await {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=log")),
    }
}

async fn log_project(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(project_id): Path<String>,
    Form(form): Form<LogForm>,
) -> Response {
    let Some(service) = state.logbook.clone() else {
        return redirect("/login");
    };
    let Some(account) = session.account() else {
        return redirect("/login");
    };
    let Ok(id) = project_id.parse::<EntityId>() else {
        return redirect("/logbook?error=notfound");
    };
    let target = form
        .return_to
        .filter(|value| value.starts_with('/'))
        .unwrap_or_else(|| "/logbook".to_string());
    match service.log_project(account.account_id, id).await {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=log")),
    }
}
