//! Command-backed HTTP routes for tags.

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Extension, Form, Router};
use momentum_app::commands::ReorderDirection;
use momentum_app::session::SessionState;
use momentum_app::tags::TagService;
use momentum_domain::{AccountId, EntityId};
use serde::Deserialize;

use crate::server::AppState;

/// Tag command routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/tags", post(create_tag))
        .route("/tags/{tag_id}/rename", post(rename_tag))
        .route("/tags/{tag_id}/move", post(move_tag))
        .route("/tags/{tag_id}/move_up", post(move_tag_up))
        .route("/tags/{tag_id}/move_down", post(move_tag_down))
        .route("/tags/{tag_id}/delete", post(delete_tag))
        .route(
            "/entities/{entity_id}/tags/{tag_id}/assign",
            post(assign_tag),
        )
        .route(
            "/entities/{entity_id}/tags/{tag_id}/unassign",
            post(unassign_tag),
        )
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
) -> Option<(Arc<TagService>, AccountId)> {
    let service = state.tags.clone()?;
    let account = session.account()?;
    Some((service, account.account_id))
}

#[derive(Debug, Deserialize)]
struct TagForm {
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    parent: Option<String>,
    #[serde(default)]
    return_to: Option<String>,
}

fn target(form: &TagForm, fallback: &str) -> String {
    form.return_to
        .clone()
        .filter(|value| value.starts_with('/'))
        .unwrap_or_else(|| fallback.to_string())
}

fn parent_of(form: &TagForm) -> Option<EntityId> {
    form.parent
        .as_deref()
        .filter(|value| !value.is_empty())
        .and_then(|value| value.parse::<EntityId>().ok())
}

async fn create_tag(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Form(form): Form<TagForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let Some(title) = form.title.clone() else {
        return redirect("/tags?error=tag");
    };
    match service.create(account_id, &title, parent_of(&form)).await {
        Ok(_) => redirect(&target(&form, "/tags")),
        Err(_) => redirect("/tags?error=tag"),
    }
}

async fn rename_tag(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(tag_id): Path<String>,
    Form(form): Form<TagForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let Ok(id) = tag_id.parse::<EntityId>() else {
        return redirect("/tags?error=notfound");
    };
    let Some(title) = form.title.clone() else {
        return redirect("/tags?error=tag");
    };
    match service.rename(account_id, id, &title).await {
        Ok(_) => redirect(&target(&form, "/tags")),
        Err(_) => redirect("/tags?error=tag"),
    }
}

async fn move_tag(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(tag_id): Path<String>,
    Form(form): Form<TagForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let Ok(id) = tag_id.parse::<EntityId>() else {
        return redirect("/tags?error=notfound");
    };
    match service
        .move_to_parent(account_id, id, parent_of(&form))
        .await
    {
        Ok(_) => redirect(&target(&form, "/tags")),
        Err(_) => redirect("/tags?error=tag"),
    }
}

async fn move_tag_up(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(tag_id): Path<String>,
    Form(form): Form<TagForm>,
) -> Response {
    reorder(&state, &session, &tag_id, form, true).await
}

async fn move_tag_down(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(tag_id): Path<String>,
    Form(form): Form<TagForm>,
) -> Response {
    reorder(&state, &session, &tag_id, form, false).await
}

async fn reorder(
    state: &AppState,
    session: &SessionState,
    raw_id: &str,
    form: TagForm,
    up: bool,
) -> Response {
    let Some((service, account_id)) = service_and_account(state, session) else {
        return redirect("/login");
    };
    let Ok(id) = raw_id.parse::<EntityId>() else {
        return redirect("/tags?error=notfound");
    };
    let direction = if up {
        ReorderDirection::Up
    } else {
        ReorderDirection::Down
    };
    match service.reorder(account_id, id, direction).await {
        Ok(_) => redirect(&target(&form, "/tags")),
        Err(_) => redirect("/tags?error=tag"),
    }
}

async fn delete_tag(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path(tag_id): Path<String>,
    Form(form): Form<TagForm>,
) -> Response {
    let Some((service, account_id)) = service_and_account(&state, &session) else {
        return redirect("/login");
    };
    let Ok(id) = tag_id.parse::<EntityId>() else {
        return redirect("/tags?error=notfound");
    };
    match service.delete(account_id, id).await {
        Ok(_) => redirect(&target(&form, "/tags")),
        Err(_) => redirect("/tags?error=tag"),
    }
}

#[derive(Debug, Deserialize)]
struct AssignmentForm {
    #[serde(default)]
    return_to: Option<String>,
}

async fn assign_tag(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path((entity_id, tag_id)): Path<(String, String)>,
    Form(form): Form<AssignmentForm>,
) -> Response {
    assignment(&state, &session, &entity_id, &tag_id, form, true).await
}

async fn unassign_tag(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Path((entity_id, tag_id)): Path<(String, String)>,
    Form(form): Form<AssignmentForm>,
) -> Response {
    assignment(&state, &session, &entity_id, &tag_id, form, false).await
}

async fn assignment(
    state: &AppState,
    session: &SessionState,
    entity_id: &str,
    tag_id: &str,
    form: AssignmentForm,
    assign: bool,
) -> Response {
    let Some((service, account_id)) = service_and_account(state, session) else {
        return redirect("/login");
    };
    let (Ok(entity), Ok(tag)) = (entity_id.parse::<EntityId>(), tag_id.parse::<EntityId>()) else {
        return redirect("/inbox?error=notfound");
    };
    let fallback = format!("/todos/{entity}");
    let target = form
        .return_to
        .filter(|value| value.starts_with('/'))
        .unwrap_or(fallback);
    let result = if assign {
        service.assign(account_id, entity, tag).await
    } else {
        service.unassign(account_id, entity, tag).await
    };
    match result {
        Ok(_) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=tag")),
    }
}
