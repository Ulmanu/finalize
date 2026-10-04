//! Structured request tracing and privacy-safe metrics exposition.

use std::sync::Arc;

use axum::extract::{Request, State};
use axum::http::{header, HeaderName, HeaderValue};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use momentum_app::metrics::Metrics;
use tracing::Instrument;
use uuid::Uuid;

use crate::server::AppState;

/// Correlation header echoed on every response.
pub const REQUEST_ID_HEADER: &str = "x-request-id";

/// Metrics routes.
pub fn routes() -> Router<AppState> {
    Router::new().route("/metrics", get(metrics))
}

/// Assigns/propagates a request id and traces the request with it.
pub async fn request_id(mut request: Request, next: Next) -> Response {
    let id = request
        .headers()
        .get(REQUEST_ID_HEADER)
        .and_then(|value| value.to_str().ok())
        .map(str::to_string)
        .unwrap_or_else(|| Uuid::now_v7().to_string());
    if let Ok(value) = HeaderValue::from_str(&id) {
        request
            .headers_mut()
            .insert(HeaderName::from_static(REQUEST_ID_HEADER), value.clone());
    }
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let span = tracing::info_span!(
        "http_request",
        request_id = %id,
        method = %method,
        path = %path,
    );
    let mut response = next.run(request).instrument(span).await;
    if let Ok(value) = HeaderValue::from_str(&id) {
        response
            .headers_mut()
            .insert(HeaderName::from_static(REQUEST_ID_HEADER), value);
    }
    response
}

/// Counts served responses (label-free, privacy-safe).
pub async fn count_requests(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    let response = next.run(request).await;
    state.metrics.increment("momentum_http_requests_total");
    response
}

async fn metrics(State(state): State<AppState>) -> Response {
    (
        [(
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/plain; version=0.0.4; charset=utf-8"),
        )],
        state.metrics.render(),
    )
        .into_response()
}

// Keeps Arc imported for state wiring in the server module.
#[allow(dead_code)]
fn _arc(metrics: Arc<Metrics>) -> Arc<Metrics> {
    metrics
}
