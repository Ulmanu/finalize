//! Performance budgets for large accounts.
//!
//! Budgets are declared once so benchmarks and tests measure the same operations with the same
//! targets. Server-measurable budgets (list, search, reorder, sync apply) are enforced by
//! testkit benchmarks against thousands of items; client-only operations (scroll
//! responsiveness, replica startup) are documented here and measured by the hydrated client.

/// One measured operation with its budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PerformanceBudget {
    /// Stable operation id.
    pub operation: &'static str,
    /// Human-readable description.
    pub description: &'static str,
    /// Target duration in milliseconds.
    pub target_ms: u64,
    /// Dataset size the budget assumes.
    pub dataset: usize,
    /// Whether the operation is measured server-side by the workspace.
    pub server_measured: bool,
}

const REGISTRY: &[PerformanceBudget] = &[
    PerformanceBudget {
        operation: "initial_render",
        description: "SSR render of a large list page",
        target_ms: 500,
        dataset: 2_000,
        server_measured: true,
    },
    PerformanceBudget {
        operation: "search",
        description: "Deep search over a large account",
        target_ms: 750,
        dataset: 2_000,
        server_measured: true,
    },
    PerformanceBudget {
        operation: "reorder",
        description: "One hundred positional reorders",
        target_ms: 1_000,
        dataset: 2_000,
        server_measured: true,
    },
    PerformanceBudget {
        operation: "sync_apply",
        description: "Applying a batch of one hundred operations",
        target_ms: 1_000,
        dataset: 100,
        server_measured: true,
    },
    PerformanceBudget {
        operation: "scroll",
        description: "Scroll responsiveness in the hydrated client",
        target_ms: 16,
        dataset: 10_000,
        server_measured: false,
    },
    PerformanceBudget {
        operation: "replica_startup",
        description: "Local replica startup in the hydrated client",
        target_ms: 250,
        dataset: 10_000,
        server_measured: false,
    },
];

/// Every declared budget.
#[must_use]
pub const fn registry() -> &'static [PerformanceBudget] {
    REGISTRY
}

/// Looks up a budget by operation id.
#[must_use]
pub fn budget(operation: &str) -> Option<&'static PerformanceBudget> {
    REGISTRY.iter().find(|budget| budget.operation == operation)
}

/// The default page size for windowed list reads.
pub const DEFAULT_PAGE_LIMIT: usize = 100;

/// The largest accepted page size for windowed list reads.
pub const MAX_PAGE_LIMIT: usize = 500;

/// Applies offset/limit windowing to a materialized list, clamping the limit.
#[must_use]
pub fn window<T>(items: Vec<T>, offset: usize, limit: usize) -> Vec<T> {
    let limit = limit.clamp(1, MAX_PAGE_LIMIT);
    items.into_iter().skip(offset).take(limit).collect()
}

#[cfg(test)]
mod tests {
    use super::{budget, registry, window, MAX_PAGE_LIMIT};

    #[test]
    fn budgets_are_unique_positive_and_cover_every_operation() {
        let mut operations: Vec<&str> = registry().iter().map(|budget| budget.operation).collect();
        let total = operations.len();
        operations.sort_unstable();
        operations.dedup();
        assert_eq!(operations.len(), total, "duplicate budget operations");
        for budget in registry() {
            assert!(budget.target_ms > 0, "{}", budget.operation);
            assert!(budget.dataset > 0, "{}", budget.operation);
            assert!(!budget.description.is_empty());
        }
        for operation in [
            "initial_render",
            "search",
            "reorder",
            "sync_apply",
            "scroll",
            "replica_startup",
        ] {
            assert!(budget(operation).is_some(), "missing {operation}");
        }
        assert!(registry().iter().any(|budget| budget.server_measured));
        assert!(registry().iter().any(|budget| !budget.server_measured));
    }

    #[test]
    fn windowing_clamps_and_slices_deterministically() {
        let items: Vec<usize> = (0..1_000).collect();
        let page = window(items.clone(), 10, 5);
        assert_eq!(page, vec![10, 11, 12, 13, 14]);
        let clamped = window(items.clone(), 0, MAX_PAGE_LIMIT + 1_000);
        assert_eq!(clamped.len(), MAX_PAGE_LIMIT);
        let beyond = window(items, 5_000, 10);
        assert!(beyond.is_empty());
    }
}
