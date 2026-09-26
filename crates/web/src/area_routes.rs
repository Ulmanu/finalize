//! Command-backed HTTP routes for the area lifecycle.

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Extension, Form, Router};
use momentum_app::areas::AreaService;
use momentum_app::commands::AreaDeletionPolicy;
use momentum_app::session::SessionState;
use momentum_domain::{AccountId, EntityId};
use serde::Deserialize;

use crate::server::AppState;

/// Area command routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/areas", post(create_area))
        .route("/areas/{area_id}/rename", post(rename_area))
        .route("/areas/{area_id}/delete", post(delete_area))
        .route("/areas/{area_id}/restore", post(restore_area))
        .route("/areas/{area_id}/purge", post(purge_area))
}

fn redirect(to: &str) -> Response {
    let mut response = StatusCode::SEE_OTHER.into_response();
    if let Ok(location) = HeaderValue::from_str(to) {
        response.headers_mut().insert(header::LOCATION, location);
    }
    response
}

fn service_and_account(
    state: &AppState,
    session: &SessionState,
) -> Option<(Arc<AreaService>, AccountId)> {
    let service = state.areas.clone()?;
    let account = session.account()?;
    Some((service, account.account_id))
}

#[derive(Debug, Deserialize)]
struct CreateAreaForm {
    title: String,
}

async fn create_area(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Form(form): Form<CreateAreaForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    match service.create(account_id, &form.title).await {
        Ok(_) => redirect("/areas"),
        Err(_) => redirect("/areas?error=create"),
    }
}

#[derive(Debug, Deserialize)]
struct RenameAreaForm {
    title: String,
}

async fn rename_area(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(area_id): Path<String>,
    Form(form): Form<RenameAreaForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let Ok(id) = area_id.parse::<EntityId>() else {
        return redirect("/areas?error=notfound");
    };
    match service.rename(account_id, id, &form.title).await {
        Ok(_) => redirect(&format!("/areas/{id}")),
        Err(_) => redirect(&format!("/areas/{id}?error=rename")),
    }
}

#[derive(Debug, Deserialize)]
struct DeleteAreaForm {
    #[serde(default)]
    policy: Option<String>,
}

async fn delete_area(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(area_id): Path<String>,
    Form(form): Form<DeleteAreaForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let Ok(id) = area_id.parse::<EntityId>() else {
        return redirect("/areas?error=notfound");
    };
    let policy = match form.policy.as_deref() {
        Some("trash") => AreaDeletionPolicy::TrashContents,
        _ => AreaDeletionPolicy::Detach,
    };
    match service.delete(account_id, id, policy).await {
        Ok(_) => redirect("/areas"),
        Err(_) => redirect(&format!("/areas/{id}?error=delete")),
    }
}

#[derive(Debug, Deserialize)]
struct AreaTrashForm {
    #[serde(default)]
    return_to: Option<String>,
}

async fn restore_area(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(area_id): Path<String>,
    Form(form): Form<AreaTrashForm>,
) -> Response {
    area_trash_action(&state, &session, &area_id, form, true).await
}

async fn purge_area(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(area_id): Path<String>,
    Form(form): Form<AreaTrashForm>,
) -> Response {
    area_trash_action(&state, &session, &area_id, form, false).await
}

async fn area_trash_action(
    state: &AppState,
    session: &SessionState,
    raw_id: &str,
    form: AreaTrashForm,
    restore: bool,
) -> Response {
    let Some((service, account_id)) = service_and_account(state, session) else {
        return redirect("/login");
    };
    let Ok(id) = raw_id.parse::<EntityId>() else {
        return redirect("/trash?error=notfound");
    };
    let target = form
        .return_to
        .filter(|value| value.starts_with('/'))
        .unwrap_or_else(|| "/trash".to_string());
    let result = if restore {
        service.restore(account_id, id).await
    } else {
        service.purge(account_id, id).await
    };
    match result {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=trash")),
    }
}
