//! Browser push subscription routes.
//!
//! The hydrated client (with browser permission) registers its push subscription here; the
//! reminder scheduler fans notifications out to these endpoints.

use axum::extract::{Extension, State};
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Form, Router};
use momentum_app::reminders::PushSubscription;
use momentum_app::session::SessionState;
use serde::Deserialize;

use crate::server::AppState;

/// Push subscription routes.
pub fn routes() -> Router<AppState> {
    Router::new().route("/push/subscriptions", post(subscribe))
}

fn redirect(to: &str) -> Response {
    let mut response = StatusCode::SEE_OTHER.into_response();
    if let Ok(location) = HeaderValue::from_str(to) {
        response.headers_mut().insert(header::LOCATION, location);
    }
    response
}

#[derive(Debug, Deserialize)]
struct SubscribeForm {
    endpoint: String,
    p256dh: String,
    auth: String,
    #[serde(default)]
    return_to: Option<String>,
}

async fn subscribe(
    State(state): State<AppState>,
    Extension(session): Extension<SessionState>,
    Form(form): Form<SubscribeForm>,
) -> Response {
    let Some(account) = session.account() else {
        return redirect("/login");
    };
    let Some(store) = state.push_subscriptions.clone() else {
        return redirect("/settings?error=push");
    };
    let target = form
        .return_to
        .filter(|value| value.starts_with('/'))
        .unwrap_or_else(|| "/settings".to_string());
    let subscription = PushSubscription {
        endpoint: form.endpoint,
        p256dh: form.p256dh,
        auth: form.auth,
    };
    match store.add(account.account_id, subscription).await {
        Ok(()) => redirect(&target),
        Err(_) => redirect(&format!("{target}?error=push")),
    }
}
