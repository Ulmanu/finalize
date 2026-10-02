//! Inbound mail webhook and alias management routes.

use axum::extract::{Extension, State};
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use momentum_app::inbound::{InboundMail, IngestOutcome};
use momentum_app::session::SessionState;
use serde::{Deserialize, Serialize};

use crate::server::AppState;

/// Inbound routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/inbound/mail", post(webhook))
        .route("/settings/inbound/rotate", post(rotate))
        .route("/settings/inbound/disable", post(disable))
}

#[derive(Debug, Deserialize)]
struct MailPayload {
    to: String,
    #[serde(default)]
    subject: String,
    #[serde(default)]
    text: String,
    message_id: String,
}

#[derive(Debug, Serialize)]
struct MailResponse {
    status: &'static str,
}

fn redirect(to: &str) -> Response {
    let mut response = StatusCode::SEE_OTHER.into_response();
    if let Ok(location) = HeaderValue::from_str(to) {
        response.headers_mut().insert(header::LOCATION, location);
    }
    response
}

async fn webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<MailPayload>,
) -> Response {
    let Some(secret) = state.inbound_secret.clone() else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let provided = headers
        .get("x-inbound-secret")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    if provided != secret {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let Some(service) = state.inbound.clone() else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    // The local part of the recipient address is the alias token.
    let token = payload.to.split('@').next().unwrap_or_default().to_string();
    let mail = InboundMail {
        message_id: payload.message_id,
        subject: payload.subject,
        body: payload.text,
    };
    let now = chrono::Utc::now();
    match service.ingest(&token, &mail, now).await {
        Ok(IngestOutcome::Delivered { .. }) => (
            StatusCode::OK,
            Json(MailResponse {
                status: "delivered",
            }),
        )
            .into_response(),
        Ok(IngestOutcome::Duplicate) => (
            StatusCode::OK,
            Json(MailResponse {
                status: "duplicate",
            }),
        )
            .into_response(),
        Ok(IngestOutcome::UnknownToken) => (
            StatusCode::NOT_FOUND,
            Json(MailResponse {
                status: "unknown_token",
            }),
        )
            .into_response(),
        Ok(IngestOutcome::Disabled) => (
            StatusCode::FORBIDDEN,
            Json(MailResponse { status: "disabled" }),
        )
            .into_response(),
        Ok(IngestOutcome::RateLimited) => (
            StatusCode::TOO_MANY_REQUESTS,
            Json(MailResponse {
                status: "rate_limited",
            }),
        )
            .into_response(),
        Ok(IngestOutcome::TooLarge) => (
            StatusCode::PAYLOAD_TOO_LARGE,
            Json(MailResponse {
                status: "too_large",
            }),
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn rotate(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
) -> Response {
    let Some(service) = state.inbound.clone() else {
        return redirect("/settings?error=inbound");
    };
    let Some(account) = session.account() else {
        return redirect("/login");
    };
    match service.rotate(account.account_id).await {
        Ok(_) => redirect("/settings"),
        Err(_) => redirect("/settings?error=inbound"),
    }
}

async fn disable(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
) -> Response {
    let Some(service) = state.inbound.clone() else {
        return redirect("/settings?error=inbound");
    };
    let Some(account) = session.account() else {
        return redirect("/login");
    };
    match service.disable(account.account_id).await {
        Ok(_) => redirect("/settings"),
        Err(_) => redirect("/settings?error=inbound"),
    }
}
