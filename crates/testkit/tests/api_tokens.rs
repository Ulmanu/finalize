//! Automation tokens: view-once creation, rotation, revocation, and authentication.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::api_tokens::{ApiScope, ApiTokenService};
use momentum_domain::AccountId;
use momentum_testkit::api_tokens::MemoryApiTokenStore;
use momentum_testkit::auth::DeterministicTokenGenerator;

fn service() -> ApiTokenService {
    ApiTokenService::new(
        Arc::new(MemoryApiTokenStore::new()),
        Arc::new(DeterministicTokenGenerator::new()),
    )
}

fn now() -> chrono::DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap()
}

#[tokio::test]
async fn tokens_are_view_once_and_authenticate_with_scopes() {
    let service = service();
    let account = AccountId::new();
    let created = service
        .create(account, "Shortcut", &[ApiScope::Write], now())
        .await
        .expect("create");
    assert_eq!(created.summary.name, "Shortcut");
    assert_eq!(created.summary.scopes, vec![ApiScope::Write]);
    assert!(!created.token.is_empty());

    // The list never exposes the plaintext, and only the hash is stored.
    let listed = service.list(account).await.expect("list");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, created.summary.id);
    assert!(listed[0].last_used_at.is_none());

    let principal = service
        .authenticate(&created.token, now())
        .await
        .expect("auth")
        .expect("principal");
    assert_eq!(principal.account_id, account);
    assert!(principal.has(ApiScope::Write));
    let touched = service.list(account).await.expect("list");
    assert!(touched[0].last_used_at.is_some());

    assert!(service
        .authenticate("not-a-token", now())
        .await
        .expect("auth")
        .is_none());
}

#[tokio::test]
async fn rotation_invalidates_the_previous_token_and_revocation_blocks_use() {
    let service = service();
    let account = AccountId::new();
    let created = service
        .create(account, "Automation", &[ApiScope::Read], now())
        .await
        .expect("create");

    let rotated = service
        .rotate(account, created.summary.id)
        .await
        .expect("rotate")
        .expect("rotated");
    assert_ne!(rotated.token, created.token);
    assert!(service
        .authenticate(&created.token, now())
        .await
        .expect("auth")
        .is_none());
    assert!(service
        .authenticate(&rotated.token, now())
        .await
        .expect("auth")
        .is_some());

    assert!(service
        .revoke(account, created.summary.id, now())
        .await
        .expect("revoke"));
    assert!(service
        .authenticate(&rotated.token, now())
        .await
        .expect("auth")
        .is_none());
    let listed = service.list(account).await.expect("list");
    assert!(listed[0].revoked_at.is_some());

    // A revoked token cannot be rotated again.
    assert!(service
        .rotate(account, created.summary.id)
        .await
        .expect("rotate")
        .is_none());
}
