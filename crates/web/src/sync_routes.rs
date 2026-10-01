//! Sync protocol JSON endpoint.

use axum::extract::{Extension, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use momentum_app::session::SessionState;
use momentum_app::sync::SyncRequest;

use crate::server::AppState;

/// Sync protocol routes.
pub fn routes() -> Router<AppState> {
    Router::new().route("/sync", post(sync))
}

async fn sync(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Json(request): Json<SyncRequest>,
) -> Response {
    let Some(account) = session.account() else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let Some(service) = state.sync.clone() else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    match service.sync(account.account_id, request).await {
        Ok(response) => Json(response).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
