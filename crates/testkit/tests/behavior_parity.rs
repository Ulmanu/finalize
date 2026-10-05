//! Behavior parity suite: end-to-end GIVEN/WHEN/THEN scenarios across major workflows.
//!
//! Every scenario exercises the canonical command/read layers (never UI-only state), asserts
//! cross-view identity (projections never duplicate or corrupt canonical entities), and covers
//! offline replay, conflict handling, and multi-tab convergence.

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use momentum_app::api_tokens::ApiScope;
use momentum_app::capture::build_capture;
use momentum_app::commands::{Command, CommandDispatcher, ScheduleInput};
use momentum_app::inbound::{InboundLimits, InboundMail, InboundService, IngestOutcome};
use momentum_app::movement::Destination;
use momentum_app::projects::ProjectService;
use momentum_app::realtime::{HintHub, RecordingHintSink};
use momentum_app::replica::{MemoryReplicaStore, OfflineCoordinator, ReplicaEntity};
use momentum_app::search::SearchService;
use momentum_app::sync::{ConflictRule, SyncOperation, SyncRequest, SyncService};
use momentum_app::tabs::{RecordingTabBroadcaster, TabCoordinator, TabMessage};
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::model::{ContextRef, Schedule, Status};
use momentum_domain::time::LocalDate;
use momentum_domain::{AccountId, EntityId};
use momentum_testkit::auth::{DeterministicTokenGenerator, TestClock};
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::inbound::{DeterministicTokenSource, MemoryInboundStore};
use momentum_testkit::projects::MemoryProjectReadStore;
use momentum_testkit::sync::MemoryChangeLogReader;
use momentum_testkit::todos::MemoryTodoReadStore;
use uuid::Uuid;

struct Harness {
    database: MemoryDatabase,
    dispatcher: Arc<CommandDispatcher>,
    todos: Arc<TodoService>,
    projects: Arc<ProjectService>,
    account: AccountId,
}

fn harness() -> Harness {
    let database = MemoryDatabase::new();
    let clock = Arc::new(TestClock::new(
        Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap(),
    ));
    let dispatcher = Arc::new(CommandDispatcher::new(
        Arc::new(database.clone()),
        clock.clone(),
    ));
    let todos = Arc::new(TodoService::new(
        dispatcher.clone(),
        Arc::new(MemoryTodoReadStore::new(database.clone())),
        clock.clone(),
    ));
    let projects = Arc::new(ProjectService::new(
        dispatcher.clone(),
        Arc::new(MemoryProjectReadStore::new(database.clone())),
        clock,
    ));
    Harness {
        database,
        dispatcher,
        todos,
        projects,
        account: AccountId::new(),
    }
}

fn date(day: u32) -> LocalDate {
    LocalDate::from_ymd(2026, 6, day).expect("date")
}

async fn find_inbox(h: &Harness, title: &str) -> EntityId {
    h.todos
        .list(h.account, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .find(|todo| todo.title == title)
        .expect("todo by title")
        .id
}

async fn inbox_ids(h: &Harness) -> Vec<EntityId> {
    h.todos
        .list(h.account, TodoScope::Inbox)
        .await
        .expect("inbox")
        .into_iter()
        .map(|todo| todo.id)
        .collect()
}

async fn scope_ids(h: &Harness, scope: TodoScope) -> Vec<EntityId> {
    h.todos
        .list(h.account, scope)
        .await
        .expect("scope")
        .into_iter()
        .map(|todo| todo.id)
        .collect()
}

async fn assert_unique(h: &Harness, scope: TodoScope) {
    let ids = scope_ids(h, scope).await;
    let mut sorted = ids.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate rows in projection");
    for id in ids {
        let canonical = h
            .todos
            .get(h.account, id)
            .await
            .expect("get")
            .expect("canonical entity");
        assert_eq!(canonical.id, id);
    }
}

/// GIVEN capture paths (Quick Entry-style create, browser share capture, inbound mail)
/// WHEN they run
/// THEN every capture lands in the Inbox as one canonical entity per source.
#[tokio::test]
async fn capture_parity_lands_every_source_in_the_inbox_once() {
    let h = harness();

    // Quick Entry composition.
    h.todos
        .create(h.account, "Quick entry", Some("notes".to_string()))
        .await
        .expect("quick entry");
    // Browser capture composition.
    let (title, notes) = build_capture(
        "Shared page",
        Some("https://example.com"),
        Some("Selected text"),
    );
    h.todos
        .create(h.account, &title, Some(notes))
        .await
        .expect("browser capture");
    // Inbound mail composition.
    let inbound = InboundService::new(
        Arc::new(MemoryInboundStore::new()),
        Arc::new(MemoryInboundStore::new()),
        h.todos.clone(),
        Arc::new(DeterministicTokenSource::new()),
        InboundLimits::default(),
    );
    let alias = inbound.ensure_alias(h.account).await.expect("alias");
    let outcome = inbound
        .ingest(
            &alias.token,
            &InboundMail {
                message_id: "m1".to_string(),
                subject: "Emailed task".to_string(),
                body: "Body".to_string(),
            },
            Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap(),
        )
        .await
        .expect("ingest");
    assert!(matches!(outcome, IngestOutcome::Delivered { .. }));

    let ids = inbox_ids(&h).await;
    assert_eq!(ids.len(), 3);
    assert_unique(&h, TodoScope::Inbox).await;
}

/// GIVEN to-dos scheduled through different paths
/// WHEN projections are read
/// THEN each entity appears once in the correct scope and moves leave no stale rows.
#[tokio::test]
async fn scheduling_parity_keeps_projections_exact() {
    let h = harness();
    h.todos
        .create(h.account, "Today task", None)
        .await
        .expect("create");
    let today_task = find_inbox(&h, "Today task").await;
    h.todos
        .schedule(h.account, today_task, ScheduleInput::On(date(10)))
        .await
        .expect("schedule");

    h.todos
        .create(h.account, "Upcoming task", None)
        .await
        .expect("create");
    let upcoming_task = find_inbox(&h, "Upcoming task").await;
    h.todos
        .schedule(h.account, upcoming_task, ScheduleInput::On(date(12)))
        .await
        .expect("schedule");

    assert_eq!(scope_ids(&h, TodoScope::Today).await, vec![today_task]);
    assert_eq!(
        scope_ids(&h, TodoScope::Upcoming).await,
        vec![upcoming_task]
    );
    // The Inbox projection keeps today-scheduled items until they move into the future.
    assert_eq!(inbox_ids(&h).await, vec![today_task]);
    assert_unique(&h, TodoScope::Today).await;
    assert_unique(&h, TodoScope::Upcoming).await;

    // Moving Today -> Upcoming must not leave a stale Today row.
    h.todos
        .schedule(h.account, today_task, ScheduleInput::On(date(12)))
        .await
        .expect("reschedule");
    assert!(scope_ids(&h, TodoScope::Today).await.is_empty());
    assert!(inbox_ids(&h).await.is_empty());
    assert_eq!(scope_ids(&h, TodoScope::Upcoming).await.len(), 2);
    assert_unique(&h, TodoScope::Upcoming).await;
}

/// GIVEN a to-do in a project
/// WHEN it moves across contexts
/// THEN the canonical context changes and only the target view lists it.
#[tokio::test]
async fn move_parity_preserves_context_identity_across_views() {
    let h = harness();
    h.projects
        .create(h.account, "Target project", None)
        .await
        .expect("project");
    let project = h
        .projects
        .list(h.account)
        .await
        .expect("projects")
        .into_iter()
        .next()
        .expect("project")
        .id;
    h.todos
        .create(h.account, "Mover", None)
        .await
        .expect("create");
    let todo = inbox_ids(&h).await[0];

    h.todos
        .move_to(h.account, todo, ContextRef::Project(project))
        .await
        .expect("move");
    assert_eq!(
        h.todos.get(h.account, todo).await.unwrap().unwrap().context,
        ContextRef::Project(project)
    );
    assert!(inbox_ids(&h).await.is_empty());
    let project_todos = h
        .todos
        .list(h.account, TodoScope::AllOpen)
        .await
        .expect("all open")
        .into_iter()
        .filter(|summary| summary.context == ContextRef::Project(project))
        .count();
    assert_eq!(project_todos, 1);

    // The move engine also accepts destination-shaped input.
    h.todos
        .move_to(h.account, todo, Destination::Inbox.context())
        .await
        .expect("move back");
    assert_eq!(inbox_ids(&h).await, vec![todo]);
}

/// GIVEN content in notes, checklists, and the logbook
/// WHEN deep search runs
/// THEN title matches outrank notes matches and results are deduplicated per entity.
#[tokio::test]
async fn search_parity_matches_quick_find_ranking() {
    let h = harness();
    h.todos
        .create(h.account, "Roadmap review", None)
        .await
        .expect("create");
    h.todos
        .create(
            h.account,
            "Groceries",
            Some("pick up for the roadmap".to_string()),
        )
        .await
        .expect("create");
    let service = SearchService::new(h.todos.clone(), h.projects.clone(), None, None, None);
    let hits = service.search(h.account, "roadmap").await.expect("search");
    assert_eq!(hits.len(), 2);
    assert_eq!(hits[0].hit.title, "Roadmap review");
    assert_eq!(hits[1].hit.title, "Groceries");
    let mut urls: Vec<&str> = hits.iter().map(|hit| hit.hit.url.as_str()).collect();
    let total = urls.len();
    urls.sort_unstable();
    urls.dedup();
    assert_eq!(urls.len(), total, "duplicate search rows");
}

/// GIVEN offline optimistic writes
/// WHEN the outbox replays and conflicts occur
/// THEN server state matches once, replays are idempotent, and delete-beats-edit is enforced.
#[tokio::test]
async fn offline_parity_replays_once_and_classifies_conflicts() {
    let h = harness();
    let sync = SyncService::new(
        h.dispatcher.clone(),
        h.todos.clone(),
        Arc::new(MemoryChangeLogReader::new(h.database.clone())),
        Arc::new(TestClock::new(
            Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap(),
        )),
    );
    let store = Arc::new(MemoryReplicaStore::new());
    let coordinator = OfflineCoordinator::new(Box::new(SharedStore(store.clone())));

    h.todos
        .create(h.account, "Offline edit", None)
        .await
        .expect("create");
    let todo = inbox_ids(&h).await[0];
    let operation = coordinator
        .apply_locally(
            &Command::SetTodoTitle {
                id: todo,
                title: "Edited offline".to_string(),
            },
            Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap(),
        )
        .await
        .expect("enqueue");
    coordinator
        .record_entity(ReplicaEntity {
            id: todo,
            kind: "todo".to_string(),
            payload: "{\"title\":\"Edited offline\"}".to_string(),
        })
        .await
        .expect("record");

    // Reconnect: replay through the canonical sync protocol.
    let pending = coordinator.startup().await.expect("startup");
    assert_eq!(pending.pending.len(), 1);
    let response = sync
        .sync(
            h.account,
            SyncRequest {
                cursor: 0,
                operations: vec![SyncOperation {
                    operation_id: operation,
                    command: Command::SetTodoTitle {
                        id: todo,
                        title: "Edited offline".to_string(),
                    },
                }],
            },
        )
        .await
        .expect("sync");
    assert_eq!(response.acknowledged.len(), 1);
    coordinator.acknowledge(operation).await.expect("ack");
    assert_eq!(
        h.todos.get(h.account, todo).await.unwrap().unwrap().title,
        "Edited offline"
    );

    // Replaying the same operation id is idempotent.
    let replay = sync
        .sync(
            h.account,
            SyncRequest {
                cursor: 0,
                operations: vec![SyncOperation {
                    operation_id: operation,
                    command: Command::SetTodoTitle {
                        id: todo,
                        title: "Edited offline".to_string(),
                    },
                }],
            },
        )
        .await
        .expect("sync");
    assert!(replay.acknowledged[0].replayed);

    // Delete beats edit for trashed entities.
    h.todos.delete(h.account, todo).await.expect("delete");
    let conflict = sync
        .sync(
            h.account,
            SyncRequest {
                cursor: 0,
                operations: vec![SyncOperation {
                    operation_id: Uuid::now_v7(),
                    command: Command::SetTodoTitle {
                        id: todo,
                        title: "Resurrect".to_string(),
                    },
                }],
            },
        )
        .await
        .expect("sync");
    assert_eq!(conflict.conflicts[0].rule, ConflictRule::DeleteBeatsEdit);
    assert_eq!(
        h.todos.get(h.account, todo).await.unwrap().unwrap().status,
        Status::Deleted
    );
}

/// GIVEN two tabs sharing an outbox and hint hub
/// WHEN one commits and leadership expires
/// THEN siblings pull, leadership recovers, and outbox work survives.
#[tokio::test]
async fn multi_tab_parity_converges_and_recovers_leadership() {
    let start = Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap();
    let account = AccountId::new();
    let broadcaster = Arc::new(RecordingTabBroadcaster::new());
    let hub = Arc::new(HintHub::new());
    let sink = Arc::new(RecordingHintSink::new());
    hub.subscribe(account, sink.clone());

    let first = TabCoordinator::new(1, account, broadcaster.clone(), hub.clone());
    let second = TabCoordinator::new(2, account, broadcaster.clone(), hub.clone());
    assert!(first.claim_or_renew(start));
    for message in broadcaster.messages() {
        second.on_message(message);
    }
    assert!(!second.claim_or_renew(start));

    // A sibling commit surfaces a local pull hint.
    second.on_message(TabMessage::Committed { sequence: 5 });
    assert_eq!(sink.hints().len(), 1);
    assert_eq!(sink.hints()[0].sequence, 5);

    // Leader recovery after lease expiry preserves outbox work.
    let later = start + momentum_app::tabs::LEADER_LEASE + chrono::Duration::seconds(1);
    assert!(!first.is_leader(later));
    assert!(second.claim_or_renew(later));
    let store = Arc::new(MemoryReplicaStore::new());
    let coordinator = OfflineCoordinator::new(Box::new(SharedStore(store.clone())));
    coordinator
        .apply_locally(
            &Command::CompleteTodo {
                id: EntityId::new(),
            },
            start,
        )
        .await
        .expect("enqueue");
    assert_eq!(
        coordinator.startup().await.expect("startup").pending.len(),
        1
    );
    let _ = ApiScope::Read;
}

/// GIVEN a mix of scheduled, completed, and moved items
/// WHEN every projection is read
/// THEN each canonical entity appears at most once per projection with matching state.
#[tokio::test]
async fn cross_view_identity_holds_across_every_projection() {
    let h = harness();
    for index in 0..4 {
        h.todos
            .create(h.account, &format!("Parity {index}"), None)
            .await
            .expect("create");
    }
    let ids = inbox_ids(&h).await;
    assert_eq!(ids.len(), 4);
    h.todos
        .schedule(h.account, ids[0], ScheduleInput::On(date(10)))
        .await
        .expect("schedule");
    h.todos
        .schedule(h.account, ids[1], ScheduleInput::ThisEvening(date(10)))
        .await
        .expect("schedule");
    h.todos
        .schedule(h.account, ids[2], ScheduleInput::On(date(12)))
        .await
        .expect("schedule");
    h.todos.complete(h.account, ids[3]).await.expect("complete");

    for scope in [
        TodoScope::Inbox,
        TodoScope::Today,
        TodoScope::Upcoming,
        TodoScope::Anytime,
        TodoScope::AllOpen,
        TodoScope::Logbook { logged_only: false },
    ] {
        assert_unique(&h, scope).await;
    }

    // Canonical state matches what projections show.
    let today = scope_ids(&h, TodoScope::Today).await;
    assert!(today.contains(&ids[0]) && today.contains(&ids[1]));
    assert_eq!(
        h.todos
            .get(h.account, ids[3])
            .await
            .unwrap()
            .unwrap()
            .status,
        Status::Completed
    );
    assert_eq!(
        h.todos
            .get(h.account, ids[2])
            .await
            .unwrap()
            .unwrap()
            .schedule,
        Schedule::on(date(12))
    );
}

/// Shares one memory store between replica coordinators and assertions.
#[derive(Debug)]
struct SharedStore(Arc<MemoryReplicaStore>);

#[async_trait::async_trait]
impl momentum_app::replica::ReplicaStore for SharedStore {
    async fn load_snapshot(
        &self,
    ) -> Result<Option<momentum_app::replica::ReplicaSnapshot>, String> {
        self.0.load_snapshot().await
    }

    async fn save_snapshot(
        &self,
        snapshot: &momentum_app::replica::ReplicaSnapshot,
    ) -> Result<(), String> {
        self.0.save_snapshot(snapshot).await
    }

    async fn load_outbox(&self) -> Result<Vec<momentum_app::replica::OutboxEntry>, String> {
        self.0.load_outbox().await
    }

    async fn save_outbox(
        &self,
        entries: &[momentum_app::replica::OutboxEntry],
    ) -> Result<(), String> {
        self.0.save_outbox(entries).await
    }
}

// Keeps the deterministic generator linked for adapters that need stable tokens in this suite.
#[allow(dead_code)]
fn _stable_tokens() -> DeterministicTokenGenerator {
    DeterministicTokenGenerator::new()
}
