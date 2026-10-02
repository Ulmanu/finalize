//! Inbound mail: alias lifecycle, deterministic mapping, and abuse protection.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::commands::CommandDispatcher;
use momentum_app::inbound::{InboundLimits, InboundMail, InboundService, IngestOutcome};
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::AccountId;
use momentum_testkit::auth::TestClock;
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::inbound::{DeterministicTokenSource, MemoryInboundStore};
use momentum_testkit::todos::MemoryTodoReadStore;

struct Harness {
    service: InboundService,
    todos: Arc<TodoService>,
    account: AccountId,
}

fn harness() -> Harness {
    let database = MemoryDatabase::new();
    let now = Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap();
    let clock = Arc::new(TestClock::new(now));
    let dispatcher = Arc::new(CommandDispatcher::new(
        Arc::new(database.clone()),
        clock.clone(),
    ));
    let todos = Arc::new(TodoService::new(
        dispatcher,
        Arc::new(MemoryTodoReadStore::new(database)),
        clock,
    ));
    let service = InboundService::new(
        Arc::new(MemoryInboundStore::new()),
        Arc::new(MemoryInboundStore::new()),
        todos.clone(),
        Arc::new(DeterministicTokenSource::new()),
        InboundLimits {
            per_hour: 3,
            max_subject: 100,
            max_body: 200,
        },
    );
    Harness {
        service,
        todos,
        account: AccountId::new(),
    }
}

fn now() -> chrono::DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap()
}

fn mail(id: &str, subject: &str, body: &str) -> InboundMail {
    InboundMail {
        message_id: id.to_string(),
        subject: subject.to_string(),
        body: body.to_string(),
    }
}

async fn inbox_titles(h: &Harness) -> Vec<String> {
    h.todos
        .list(h.account, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .map(|todo| todo.title)
        .collect()
}

#[tokio::test]
async fn alias_lifecycle_rotates_and_disables() {
    let h = harness();
    let alias = h.service.ensure_alias(h.account).await.expect("alias");
    assert_eq!(alias.token.len(), 32);
    assert!(alias.address().ends_with("@inbox.momentum.example"));
    assert_eq!(
        h.service.ensure_alias(h.account).await.expect("alias"),
        alias
    );

    let rotated = h.service.rotate(h.account).await.expect("rotate");
    assert_ne!(rotated.token, alias.token);
    let stale = h
        .service
        .ingest(&alias.token, &mail("m1", "Old token", ""), now())
        .await
        .expect("ingest");
    assert_eq!(stale, IngestOutcome::UnknownToken);

    h.service.disable(h.account).await.expect("disable");
    let disabled = h
        .service
        .ingest(&rotated.token, &mail("m2", "Disabled", ""), now())
        .await
        .expect("ingest");
    assert_eq!(disabled, IngestOutcome::Disabled);
    assert!(inbox_titles(&h).await.is_empty());
}

#[tokio::test]
async fn ingest_maps_mail_and_deduplicates_message_ids() {
    let h = harness();
    let alias = h.service.ensure_alias(h.account).await.expect("alias");

    let first = h
        .service
        .ingest(
            &alias.token,
            &mail(
                "m1",
                "  Weekly report ",
                "First line\n> quoted\nSecond line",
            ),
            now(),
        )
        .await
        .expect("ingest");
    assert!(matches!(first, IngestOutcome::Delivered { .. }));
    let created = h
        .todos
        .list(h.account, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .next()
        .expect("created");
    assert_eq!(created.title, "Weekly report");
    assert_eq!(created.notes, "First line Second line");

    let duplicate = h
        .service
        .ingest(
            &alias.token,
            &mail("m1", "Weekly report", "Different body"),
            now(),
        )
        .await
        .expect("ingest");
    assert_eq!(duplicate, IngestOutcome::Duplicate);
    assert_eq!(inbox_titles(&h).await.len(), 1);

    let second = h
        .service
        .ingest(&alias.token, &mail("m2", "Another", "Body"), now())
        .await
        .expect("ingest");
    assert!(matches!(second, IngestOutcome::Delivered { .. }));
    assert_eq!(inbox_titles(&h).await.len(), 2);
}

#[tokio::test]
async fn oversized_and_rate_limited_messages_are_rejected_without_items() {
    let h = harness();
    let alias = h.service.ensure_alias(h.account).await.expect("alias");

    let oversized = h
        .service
        .ingest(&alias.token, &mail("big", &"s".repeat(101), "body"), now())
        .await
        .expect("ingest");
    assert_eq!(oversized, IngestOutcome::TooLarge);
    assert!(inbox_titles(&h).await.is_empty());

    for index in 0..3 {
        let outcome = h
            .service
            .ingest(
                &alias.token,
                &mail(&format!("ok{index}"), "Subject", "Body"),
                now(),
            )
            .await
            .expect("ingest");
        assert!(matches!(outcome, IngestOutcome::Delivered { .. }));
    }
    let limited = h
        .service
        .ingest(&alias.token, &mail("over", "Subject", "Body"), now())
        .await
        .expect("ingest");
    assert_eq!(limited, IngestOutcome::RateLimited);
    assert_eq!(inbox_titles(&h).await.len(), 3);

    // The window slides: an hour later the account may ingest again.
    let later = now() + chrono::Duration::hours(1);
    let recovered = h
        .service
        .ingest(&alias.token, &mail("later", "Subject", "Body"), later)
        .await
        .expect("ingest");
    assert!(matches!(recovered, IngestOutcome::Delivered { .. }));
}
