//! Privacy-safe operational metrics.
//!
//! Counters are declared once with descriptions and carry no labels, so no task titles, notes,
//! email bodies, tokens, or account identifiers can ever appear in metrics output. Services
//! increment counters through an optional `Arc<Metrics>`.

use std::collections::BTreeMap;
use std::sync::Mutex;

/// One declared counter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CounterDefinition {
    /// Stable metric name.
    pub name: &'static str,
    /// Human-readable description.
    pub description: &'static str,
}

const COUNTERS: &[CounterDefinition] = &[
    CounterDefinition {
        name: "momentum_http_requests_total",
        description: "HTTP responses served",
    },
    CounterDefinition {
        name: "momentum_sync_operations_total",
        description: "Sync operations acknowledged",
    },
    CounterDefinition {
        name: "momentum_sync_conflicts_total",
        description: "Sync operations classified as conflicts",
    },
    CounterDefinition {
        name: "momentum_reminders_delivered_total",
        description: "Reminders delivered successfully",
    },
    CounterDefinition {
        name: "momentum_reminders_failed_total",
        description: "Reminder deliveries that failed",
    },
    CounterDefinition {
        name: "momentum_recurrence_instances_total",
        description: "Recurrence instances materialized",
    },
    CounterDefinition {
        name: "momentum_inbound_delivered_total",
        description: "Inbound emails that created items",
    },
    CounterDefinition {
        name: "momentum_inbound_rejected_total",
        description: "Inbound emails rejected by limits or state",
    },
    CounterDefinition {
        name: "momentum_calendar_success_total",
        description: "Calendar provider reads that succeeded",
    },
    CounterDefinition {
        name: "momentum_calendar_failure_total",
        description: "Calendar provider reads that degraded",
    },
    CounterDefinition {
        name: "momentum_push_sent_total",
        description: "Push notifications sent to subscriptions",
    },
];

/// Every declared counter.
#[must_use]
pub const fn registry() -> &'static [CounterDefinition] {
    COUNTERS
}

/// Whether a name is a declared counter.
#[must_use]
pub fn is_declared(name: &str) -> bool {
    COUNTERS.iter().any(|counter| counter.name == name)
}

/// In-process counter store.
#[derive(Debug, Default)]
pub struct Metrics {
    counters: Mutex<BTreeMap<&'static str, u64>>,
}

impl Metrics {
    /// Creates an empty store.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Increments a declared counter (unknown names are ignored).
    pub fn increment(&self, name: &'static str) {
        if !is_declared(name) {
            return;
        }
        *self
            .counters
            .lock()
            .expect("metrics lock")
            .entry(name)
            .or_insert(0) += 1;
    }

    /// Increments by a delta.
    pub fn add(&self, name: &'static str, delta: u64) {
        if !is_declared(name) {
            return;
        }
        *self
            .counters
            .lock()
            .expect("metrics lock")
            .entry(name)
            .or_insert(0) += delta;
    }

    /// The current value of a counter.
    #[must_use]
    pub fn value(&self, name: &str) -> u64 {
        self.counters
            .lock()
            .expect("metrics lock")
            .get(name)
            .copied()
            .unwrap_or(0)
    }

    /// Renders the Prometheus-style text exposition (names, descriptions, values only).
    #[must_use]
    pub fn render(&self) -> String {
        let counters = self.counters.lock().expect("metrics lock");
        let mut out = String::new();
        for definition in COUNTERS {
            out.push_str(&format!(
                "# HELP {} {}\n# TYPE {} counter\n{} {}\n",
                definition.name,
                definition.description,
                definition.name,
                definition.name,
                counters.get(definition.name).copied().unwrap_or(0)
            ));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::{is_declared, registry, Metrics};

    #[test]
    fn registry_is_unique_and_descriptive() {
        let mut names: Vec<&str> = registry().iter().map(|counter| counter.name).collect();
        let total = names.len();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), total, "duplicate counter names");
        for counter in registry() {
            assert!(counter.name.starts_with("momentum_"));
            assert!(!counter.description.is_empty());
        }
    }

    #[test]
    fn counters_increment_render_and_ignore_unknown_names() {
        let metrics = Metrics::new();
        metrics.increment("momentum_http_requests_total");
        metrics.add("momentum_sync_operations_total", 3);
        metrics.increment("unknown_metric");
        assert_eq!(metrics.value("momentum_http_requests_total"), 1);
        assert_eq!(metrics.value("momentum_sync_operations_total"), 3);
        assert_eq!(metrics.value("unknown_metric"), 0);
        assert!(!is_declared("unknown_metric"));

        let rendered = metrics.render();
        assert!(
            rendered.contains("momentum_http_requests_total 1"),
            "{rendered}"
        );
        assert!(
            rendered.contains("momentum_sync_operations_total 3"),
            "{rendered}"
        );
        // Privacy: the exposition contains only names, descriptions, and values.
        assert!(!rendered.contains('@'), "{rendered}");
        assert!(!rendered.contains("token"), "{rendered}");
    }
}
