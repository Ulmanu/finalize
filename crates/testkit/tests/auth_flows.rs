//! Account lifecycle tests over the in-memory auth adapters.

use std::sync::Arc;

use chrono::{Duration, TimeZone, Utc};
use momentum_app::auth::{AuthError, AuthService};
use momentum_domain::ids::SessionId;
use momentum_testkit::auth::{
    DeterministicHasher, DeterministicTokenGenerator, MemoryAuthStore, TestClock,
};

struct Harness {
    service: AuthService,
    store: Arc<MemoryAuthStore>,
    clock: Arc<TestClock>,
}

fn harness() -> Harness {
    let store = Arc::new(MemoryAuthStore::new());
    let clock = Arc::new(TestClock::new(
        Utc.with_ymd_and_hms(2026, 1, 1, 9, 0, 0).unwrap(),
    ));
    let service = AuthService::new(
        store.clone(),
        store.clone(),
        store.clone(),
        Arc::new(DeterministicHasher),
        Arc::new(DeterministicTokenGenerator::new()),
        clock.clone(),
    );
    Harness {
        service,
        store,
        clock,
    }
}

async fn registered(harness: &Harness) -> (momentum_domain::AccountId, String) {
    let outcome = harness
        .service
        .register("person@example.com", "correct horse battery", None)
        .await
        .expect("register");
    (outcome.account_id, outcome.verification_token)
}

#[tokio::test]
async fn registration_verification_and_login_flow() {
    let h = harness();
    let (account_id, token) = registered(&h).await;

    let account = h.store.account_by_id(account_id).expect("account exists");
    assert!(!account.is_verified());
    assert_eq!(
        account.password_hash.as_deref(),
        Some("test-hash:correct horse battery")
    );

    assert_eq!(
        h.service.verify_email(&token).await.expect("verify"),
        account_id
    );
    assert!(h
        .store
        .account_by_id(account_id)
        .expect("account")
        .is_verified());

    let login = h
        .service
        .login("Person@Example.com", "correct horse battery")
        .await
        .expect("login");
    assert_eq!(login.account.id, account_id);
    assert!(login.token.starts_with("test-token-"));

    let authenticated = h
        .service
        .authenticate_session(&login.token)
        .await
        .expect("session valid");
    assert_eq!(authenticated.id, account_id);
}

#[tokio::test]
async fn duplicate_email_is_rejected() {
    let h = harness();
    registered(&h).await;
    let error = h
        .service
        .register("PERSON@example.com", "another good password", None)
        .await
        .expect_err("duplicate email");
    assert!(matches!(error, AuthError::EmailTaken));
}

#[tokio::test]
async fn login_requires_verification() {
    let h = harness();
    registered(&h).await;
    let error = h
        .service
        .login("person@example.com", "correct horse battery")
        .await
        .expect_err("unverified");
    assert!(matches!(error, AuthError::EmailNotVerified));
}

#[tokio::test]
async fn unknown_email_and_wrong_password_are_uniform() {
    let h = harness();
    let (_, token) = registered(&h).await;
    h.service.verify_email(&token).await.expect("verify");

    let unknown = h
        .service
        .login("nobody@example.com", "correct horse battery")
        .await
        .expect_err("unknown email");
    let wrong = h
        .service
        .login("person@example.com", "wrong password value")
        .await
        .expect_err("wrong password");
    assert!(matches!(unknown, AuthError::InvalidCredentials));
    assert!(matches!(wrong, AuthError::InvalidCredentials));
}

#[tokio::test]
async fn logout_revokes_the_session() {
    let h = harness();
    let (_, token) = registered(&h).await;
    h.service.verify_email(&token).await.expect("verify");
    let login = h
        .service
        .login("person@example.com", "correct horse battery")
        .await
        .expect("login");

    h.service.logout(&login.token).await.expect("logout");
    let error = h
        .service
        .authenticate_session(&login.token)
        .await
        .expect_err("revoked session");
    assert!(matches!(error, AuthError::InvalidSession));
}

#[tokio::test]
async fn expired_session_is_rejected() {
    let h = harness();
    let (_, token) = registered(&h).await;
    h.service.verify_email(&token).await.expect("verify");
    let login = h
        .service
        .login("person@example.com", "correct horse battery")
        .await
        .expect("login");

    h.clock.advance(Duration::days(31));
    let error = h
        .service
        .authenticate_session(&login.token)
        .await
        .expect_err("expired session");
    assert!(matches!(error, AuthError::InvalidSession));
}

#[tokio::test]
async fn verification_tokens_are_single_use() {
    let h = harness();
    let (_, token) = registered(&h).await;
    h.service.verify_email(&token).await.expect("first use");
    let error = h
        .service
        .verify_email(&token)
        .await
        .expect_err("second use");
    assert!(matches!(error, AuthError::InvalidToken));
}

#[tokio::test]
async fn password_reset_revokes_sessions_and_uses_new_password() {
    let h = harness();
    let (account_id, token) = registered(&h).await;
    h.service.verify_email(&token).await.expect("verify");
    let login = h
        .service
        .login("person@example.com", "correct horse battery")
        .await
        .expect("login");

    let reset = h
        .service
        .request_password_reset("person@example.com")
        .await
        .expect("reset request")
        .expect("token issued");

    h.service
        .reset_password(&reset.reset_token, "a brand new password")
        .await
        .expect("reset");

    assert!(h.service.authenticate_session(&login.token).await.is_err());

    let old = h
        .service
        .login("person@example.com", "correct horse battery")
        .await
        .expect_err("old password");
    assert!(matches!(old, AuthError::InvalidCredentials));

    h.service
        .login("person@example.com", "a brand new password")
        .await
        .expect("new password");

    assert_eq!(
        h.store.account_by_id(account_id).expect("account").id,
        account_id
    );
}

#[tokio::test]
async fn password_reset_for_unknown_email_is_silent() {
    let h = harness();
    let outcome = h
        .service
        .request_password_reset("nobody@example.com")
        .await
        .expect("no error");
    assert!(outcome.is_none());
}

#[tokio::test]
async fn expired_reset_token_is_rejected() {
    let h = harness();
    let (_, token) = registered(&h).await;
    h.service.verify_email(&token).await.expect("verify");
    let reset = h
        .service
        .request_password_reset("person@example.com")
        .await
        .expect("reset request")
        .expect("token issued");

    h.clock.advance(Duration::hours(2));
    let error = h
        .service
        .reset_password(&reset.reset_token, "a brand new password")
        .await
        .expect_err("expired token");
    assert!(matches!(error, AuthError::InvalidToken));
}

#[tokio::test]
async fn change_password_requires_current_password_and_revokes_sessions() {
    let h = harness();
    let (account_id, token) = registered(&h).await;
    h.service.verify_email(&token).await.expect("verify");
    let login = h
        .service
        .login("person@example.com", "correct horse battery")
        .await
        .expect("login");

    let wrong = h
        .service
        .change_password(account_id, "wrong password", "new strong password")
        .await
        .expect_err("wrong current");
    assert!(matches!(wrong, AuthError::InvalidCredentials));

    h.service
        .change_password(account_id, "correct horse battery", "new strong password")
        .await
        .expect("change");
    assert!(h.service.authenticate_session(&login.token).await.is_err());
    h.service
        .login("person@example.com", "new strong password")
        .await
        .expect("new password works");
}

#[tokio::test]
async fn revoke_session_only_affects_owned_sessions() {
    let h = harness();
    let (account_id, token) = registered(&h).await;
    h.service.verify_email(&token).await.expect("verify");
    let first = h
        .service
        .login("person@example.com", "correct horse battery")
        .await
        .expect("first login");
    let second = h
        .service
        .login("person@example.com", "correct horse battery")
        .await
        .expect("second login");

    h.service
        .revoke_session(account_id, first.session_id)
        .await
        .expect("revoke first");
    assert!(h.service.authenticate_session(&first.token).await.is_err());
    assert!(h.service.authenticate_session(&second.token).await.is_ok());

    let other_account = momentum_domain::AccountId::new();
    let error = h
        .service
        .revoke_session(other_account, second.session_id)
        .await
        .expect_err("foreign session");
    assert!(matches!(error, AuthError::InvalidSession));
    assert!(h.service.authenticate_session(&second.token).await.is_ok());
}

#[tokio::test]
async fn account_deletion_blocks_authentication() {
    let h = harness();
    let (account_id, token) = registered(&h).await;
    h.service.verify_email(&token).await.expect("verify");
    let login = h
        .service
        .login("person@example.com", "correct horse battery")
        .await
        .expect("login");

    h.service
        .delete_account(account_id, "correct horse battery")
        .await
        .expect("delete");

    assert!(h.service.authenticate_session(&login.token).await.is_err());
    let error = h
        .service
        .login("person@example.com", "correct horse battery")
        .await
        .expect_err("deleted account");
    assert!(matches!(error, AuthError::InvalidCredentials));
}

#[tokio::test]
async fn weak_passwords_and_invalid_emails_are_rejected() {
    let h = harness();
    assert!(matches!(
        h.service
            .register("not-an-email", "correct horse battery", None)
            .await,
        Err(AuthError::InvalidEmail(_))
    ));
    assert!(matches!(
        h.service
            .register("person@example.com", "short", None)
            .await,
        Err(AuthError::WeakPassword(_))
    ));
    assert!(matches!(
        h.service
            .register("person@example.com", "person@example.com", None)
            .await,
        Err(AuthError::WeakPassword(_))
    ));
}

#[tokio::test]
async fn session_ids_are_canonical() {
    let h = harness();
    let (_, token) = registered(&h).await;
    h.service.verify_email(&token).await.expect("verify");
    let login = h
        .service
        .login("person@example.com", "correct horse battery")
        .await
        .expect("login");
    let session_id: SessionId = login.session_id;
    assert_eq!(session_id.to_string().len(), 36);
}
