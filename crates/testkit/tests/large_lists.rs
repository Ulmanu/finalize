//! Large-account benchmarks: list, search, reorder, and sync apply against declared budgets.
//!
//! Budgets are declared in `momentum_app::performance`; debug builds run roughly an order of
//! magnitude slower than release, so assertions use a documented debug factor while the raw
//! targets remain the production budgets.

use std::sync::Arc;
use std::time::Instant;

use chrono::{TimeZone, Utc};
use momentum_app::commands::{Command, CommandDispatcher};
use momentum_app::performance::budget;
use momentum_app::projects::ProjectService;
use momentum_app::search::SearchService;
use momentum_app::sync::{SyncOperation, SyncRequest, SyncService};
use momentum_app::todos::{TodoScope, TodoService};
use momentum_domain::AccountId;
use momentum_testkit::auth::TestClock;
use momentum_testkit::commands::MemoryDatabase;
use momentum_testkit::projects::MemoryProjectReadStore;
use momentum_testkit::sync::MemoryChangeLogReader;
use momentum_testkit::todos::MemoryTodoReadStore;
use uuid::Uuid;

const DATASET: usize = 2_000;
const DEBUG_FACTOR: u64 = 10;

fn debug_budget(operation: &str) -> u128 {
    let budget = budget(operation).expect("budget");
    u128::from(budget.target_ms) * u128::from(DEBUG_FACTOR)
}

struct Harness {
    database: MemoryDatabase,
    todos: Arc<TodoService>,
    projects: Arc<ProjectService>,
    dispatcher: Arc<CommandDispatcher>,
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
        todos,
        projects,
        dispatcher,
        account: AccountId::new(),
    }
}

async fn seed(h: &Harness, count: usize) -> Vec<momentum_domain::EntityId> {
    let mut ids = Vec::with_capacity(count);
    for index in 0..count {
        h.todos
            .create(h.account, &format!("Task {index:04}"), None)
            .await
            .expect("create");
    }
    let inbox = h
        .todos
        .list(h.account, TodoScope::Inbox)
        .await
        .expect("inbox");
    for todo in inbox {
        ids.push(todo.id);
    }
    ids
}

#[tokio::test]
async fn list_and_search_stay_within_budgets_for_thousands_of_items() {
    let h = harness();
    let ids = seed(&h, DATASET).await;
    assert_eq!(ids.len(), DATASET);

    let start = Instant::now();
    let listed = h
        .todos
        .list(h.account, TodoScope::AllOpen)
        .await
        .expect("list");
    let list_ms = start.elapsed().as_millis();
    assert_eq!(listed.len(), DATASET);
    assert!(
        list_ms <= debug_budget("initial_render"),
        "list took {list_ms}ms (budget {}ms x{DEBUG_FACTOR})",
        budget("initial_render").expect("budget").target_ms
    );

    let service = SearchService::new(h.todos.clone(), h.projects.clone(), None, None, None);
    let start = Instant::now();
    let hits = service
        .search(h.account, "task 0999")
        .await
        .expect("search");
    let search_ms = start.elapsed().as_millis();
    assert!(!hits.is_empty());
    assert!(
        search_ms <= debug_budget("search"),
        "search took {search_ms}ms (budget {}ms x{DEBUG_FACTOR})",
        budget("search").expect("budget").target_ms
    );
}

#[tokio::test]
async fn reorder_and_sync_apply_stay_within_budgets() {
    let h = harness();
    let ids = seed(&h, DATASET).await;

    let start = Instant::now();
    for (index, id) in ids.iter().take(100).enumerate() {
        h.todos
            .move_to_position(h.account, *id, index)
            .await
            .expect("reorder");
    }
    let reorder_ms = start.elapsed().as_millis();
    assert!(
        reorder_ms <= debug_budget("reorder"),
        "reorder took {reorder_ms}ms (budget {}ms x{DEBUG_FACTOR})",
        budget("reorder").expect("budget").target_ms
    );

    let sync = SyncService::new(
        h.dispatcher.clone(),
        h.todos.clone(),
        Arc::new(MemoryChangeLogReader::new(h.database.clone())),
        Arc::new(TestClock::new(
            Utc.with_ymd_and_hms(2026, 6, 10, 9, 0, 0).unwrap(),
        )),
    );
    let operations: Vec<SyncOperation> = ids
        .iter()
        .take(100)
        .map(|id| SyncOperation {
            operation_id: Uuid::now_v7(),
            command: Command::SetTodoTitle {
                id: *id,
                title: "Renamed".to_string(),
            },
        })
        .collect();
    let start = Instant::now();
    let response = sync
        .sync(
            h.account,
            SyncRequest {
                cursor: 0,
                operations,
            },
        )
        .await
        .expect("sync");
    let sync_ms = start.elapsed().as_millis();
    assert_eq!(response.acknowledged.len(), 100);
    assert!(
        sync_ms <= debug_budget("sync_apply"),
        "sync apply took {sync_ms}ms (budget {}ms x{DEBUG_FACTOR})",
        budget("sync_apply").expect("budget").target_ms
    );
}
