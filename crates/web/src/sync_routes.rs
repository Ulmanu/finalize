//! Sync protocol JSON endpoint and realtime invalidation stream.

use std::convert::Infallible;
use std::sync::Arc;

use axum::extract::{Extension, State};
use axum::http::StatusCode;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use momentum_app::realtime::{HintHub, HintSink, InvalidationHint};
use momentum_app::session::SessionState;
use momentum_app::sync::SyncRequest;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_stream::StreamExt;

use crate::server::AppState;

/// Sync protocol routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/sync", post(sync))
        .route("/sync/events", get(events))
}

struct ChannelSink {
    sender: tokio::sync::mpsc::UnboundedSender<Event>,
}

impl HintSink for ChannelSink {
    fn hint(&self, hint: InvalidationHint) {
        let data = format!("{{\"sequence\":{}}}", hint.sequence);
        let _ = self
            .sender
            .send(Event::default().event("invalidation").data(data));
    }
}

struct SubscriptionGuard {
    hub: Arc<HintHub>,
    id: u64,
}

impl Drop for SubscriptionGuard {
    fn drop(&mut self) {
        self.hub.unsubscribe(self.id);
    }
}

async fn events(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
) -> Response {
    let Some(account) = session.account() else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let Some(hub) = state.hints.clone() else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel::<Event>();
    let subscription = hub.subscribe(
        account.account_id,
        Arc::new(ChannelSink {
            sender: sender.clone(),
        }),
    );
    let guard = SubscriptionGuard {
        hub,
        id: subscription,
    };
    // An immediate hint covers reconnects: the client pulls through its authoritative cursor.
    let _ = sender.send(
        Event::default()
            .event("invalidation")
            .data("{\"reason\":\"connect\"}"),
    );
    let stream = UnboundedReceiverStream::new(receiver).map(move |event| {
        let _ = &guard;
        Ok::<Event, Infallible>(event)
    });
    Sse::new(stream)
        .keep_alive(KeepAlive::default())
        .into_response()
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
