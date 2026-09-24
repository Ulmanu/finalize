//! Ordering engine: independent scopes, gap-based insertion, deterministic maintenance.
//!
//! Ranks are integers with wide gaps; inserting between neighbors only assigns a new rank and
//! never renumbers the list. When a gap is exhausted the caller rebalances the scope, which
//! assigns evenly spaced ranks in the same visible order. Ranks are stored as zero-padded
//! fixed-width decimals so that `text` comparison in PostgreSQL matches numeric order.

use std::fmt;

use momentum_domain::model::ContextRef;
use momentum_domain::EntityId;
use thiserror::Error;

/// Spacing between adjacent ranks.
pub const STEP: i64 = 1024;

/// Width of the zero-padded storage encoding.
pub const ENCODED_WIDTH: usize = 19;

/// Ordering scope: a contextual list or the Today projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderingScope {
    /// A contextual list identified by its parent.
    Context(ContextRef),
    /// The Today projection's independent manual order.
    Today,
}

/// Ordering failures.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum OrderingError {
    /// No integer gap remains between the requested neighbors.
    #[error("rank space is exhausted at position {position} in scope {scope:?}")]
    Exhausted {
        /// Scope where the insertion was attempted.
        scope: OrderingScope,
        /// Zero-based position where the insertion was attempted.
        position: usize,
    },
    /// The requested index is outside the list.
    #[error("index {index} is out of bounds for a list of {len} items")]
    OutOfBounds {
        /// Requested index.
        index: usize,
        /// Current list length.
        len: usize,
    },
    /// The rank encoding could not be parsed.
    #[error("invalid rank encoding")]
    InvalidEncoding,
}

/// A gap-based ordering rank.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rank(i64);

impl Rank {
    /// The first rank of a list.
    #[must_use]
    pub const fn start() -> Self {
        Self(STEP)
    }

    /// Wraps a raw rank value.
    ///
    /// # Errors
    ///
    /// Returns [`OrderingError::OutOfBounds`] when the value is not positive.
    pub fn new(value: i64) -> Result<Self, OrderingError> {
        if value <= 0 {
            return Err(OrderingError::OutOfBounds { index: 0, len: 0 });
        }
        Ok(Self(value))
    }

    /// Returns the raw rank value.
    #[must_use]
    pub const fn value(self) -> i64 {
        self.0
    }

    /// Encodes the rank as a zero-padded fixed-width decimal string.
    #[must_use]
    pub fn encode(self) -> String {
        format!("{:0width$}", self.0, width = ENCODED_WIDTH)
    }

    /// Parses a stored rank encoding.
    ///
    /// # Errors
    ///
    /// Returns [`OrderingError::InvalidEncoding`] for malformed text.
    pub fn parse(value: &str) -> Result<Self, OrderingError> {
        let trimmed = value.trim();
        if trimmed.is_empty()
            || trimmed.len() > ENCODED_WIDTH
            || !trimmed.bytes().all(|b| b.is_ascii_digit())
        {
            return Err(OrderingError::InvalidEncoding);
        }
        trimmed
            .parse::<i64>()
            .map(Self)
            .map_err(|_| OrderingError::InvalidEncoding)
    }

    /// Computes a rank strictly between `lower` and `upper`.
    ///
    /// `None` means unbounded on that side.
    ///
    /// # Errors
    ///
    /// Returns [`OrderingError::Exhausted`] when no integer gap exists or the range would
    /// overflow.
    pub fn between(
        lower: Option<Rank>,
        upper: Option<Rank>,
        scope: OrderingScope,
        position: usize,
    ) -> Result<Self, OrderingError> {
        let exhausted = || OrderingError::Exhausted { scope, position };
        match (lower, upper) {
            (None, None) => Ok(Self::start()),
            (None, Some(upper)) => {
                let value = upper.0 / 2;
                if value <= 0 {
                    Err(exhausted())
                } else {
                    Ok(Self(value))
                }
            }
            (Some(lower), None) => lower.0.checked_add(STEP).map(Self).ok_or_else(exhausted),
            (Some(lower), Some(upper)) => {
                if lower.0 >= upper.0 {
                    return Err(exhausted());
                }
                let value = lower.0 + (upper.0 - lower.0) / 2;
                if value <= lower.0 || value >= upper.0 {
                    Err(exhausted())
                } else {
                    Ok(Self(value))
                }
            }
        }
    }

    /// Computes evenly spaced ranks for `count` items, preserving order.
    #[must_use]
    pub fn rebalance(count: usize) -> Vec<Rank> {
        (0..count)
            .map(|index| {
                #[allow(clippy::cast_possible_wrap)]
                Self(STEP.saturating_mul(index as i64 + 1))
            })
            .collect()
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.encode())
    }
}

/// An entity together with its rank in one scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderedItem {
    /// Entity identity.
    pub entity_id: EntityId,
    /// Rank within the scope.
    pub rank: Rank,
}

/// Persistable outcome of an ordering operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderingChange {
    /// Rank assigned to the inserted or moved entity.
    pub rank: Rank,
    /// Existing items whose ranks changed during a rebalance.
    pub rebalanced: Vec<OrderedItem>,
}

/// One scope's ordered items.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeOrder {
    scope: OrderingScope,
    items: Vec<OrderedItem>,
}

impl ScopeOrder {
    /// Creates an empty order for a scope.
    #[must_use]
    pub fn new(scope: OrderingScope) -> Self {
        Self {
            scope,
            items: Vec::new(),
        }
    }

    /// Creates an order from existing items (sorted by rank).
    #[must_use]
    pub fn from_items(scope: OrderingScope, mut items: Vec<OrderedItem>) -> Self {
        items.sort_by_key(|item| item.rank);
        Self { scope, items }
    }

    /// Returns the scope.
    #[must_use]
    pub const fn scope(&self) -> OrderingScope {
        self.scope
    }

    /// Returns the ordered items.
    #[must_use]
    pub fn items(&self) -> &[OrderedItem] {
        &self.items
    }

    /// Returns the ranks in visible order.
    #[must_use]
    pub fn ranks(&self) -> Vec<Rank> {
        self.items.iter().map(|item| item.rank).collect()
    }

    /// Mutable access to the ordered items (used by adapters assembling stored order).
    pub fn items_mut(&mut self) -> &mut Vec<OrderedItem> {
        &mut self.items
    }

    /// Sorts items by rank in place.
    pub fn sort_by_rank(&mut self) {
        self.items.sort_by_key(|item| item.rank);
    }

    /// Returns the position of an entity, if present.
    #[must_use]
    pub fn position_of(&self, entity_id: EntityId) -> Option<usize> {
        self.items
            .iter()
            .position(|item| item.entity_id == entity_id)
    }

    /// Plans insertion of an entity at `index`, rebalancing when the gap is exhausted.
    ///
    /// # Errors
    ///
    /// Returns [`OrderingError::OutOfBounds`] when `index > len`.
    pub fn insert_at(
        &mut self,
        entity_id: EntityId,
        index: usize,
    ) -> Result<OrderingChange, OrderingError> {
        if index > self.items.len() {
            return Err(OrderingError::OutOfBounds {
                index,
                len: self.items.len(),
            });
        }
        let lower = index
            .checked_sub(1)
            .and_then(|i| self.items.get(i))
            .map(|item| item.rank);
        let upper = self.items.get(index).map(|item| item.rank);

        let rank = match Rank::between(lower, upper, self.scope, index) {
            Ok(rank) => rank,
            Err(OrderingError::Exhausted { .. }) => {
                let rebalanced = self.rebalance_in_place();
                let lower = index
                    .checked_sub(1)
                    .and_then(|i| self.items.get(i))
                    .map(|item| item.rank);
                let upper = self.items.get(index).map(|item| item.rank);
                let rank = Rank::between(lower, upper, self.scope, index)?;
                self.items.insert(index, OrderedItem { entity_id, rank });
                return Ok(OrderingChange { rank, rebalanced });
            }
            Err(error) => return Err(error),
        };

        self.items.insert(index, OrderedItem { entity_id, rank });
        Ok(OrderingChange {
            rank,
            rebalanced: Vec::new(),
        })
    }

    /// Plans appending an entity at the end.
    ///
    /// # Errors
    ///
    /// Propagates insertion errors.
    pub fn append(&mut self, entity_id: EntityId) -> Result<OrderingChange, OrderingError> {
        self.insert_at(entity_id, self.items.len())
    }

    /// Plans moving an existing entity to `to` (post-removal index).
    ///
    /// # Errors
    ///
    /// Returns [`OrderingError::OutOfBounds`] for invalid indices.
    pub fn move_item(
        &mut self,
        entity_id: EntityId,
        to: usize,
    ) -> Result<OrderingChange, OrderingError> {
        let Some(from) = self.position_of(entity_id) else {
            return Err(OrderingError::OutOfBounds {
                index: to,
                len: self.items.len(),
            });
        };
        if to >= self.items.len() {
            return Err(OrderingError::OutOfBounds {
                index: to,
                len: self.items.len(),
            });
        }
        if from == to {
            return Ok(OrderingChange {
                rank: self.items[from].rank,
                rebalanced: Vec::new(),
            });
        }

        let removed = self.items.remove(from);
        let result = self.insert_at(entity_id, to);
        if result.is_err() {
            self.items.insert(from, removed);
        }
        result
    }

    /// Rebalances every rank, preserving visible order.
    pub fn rebalance(&mut self) -> Vec<OrderedItem> {
        self.rebalance_in_place()
    }

    fn rebalance_in_place(&mut self) -> Vec<OrderedItem> {
        let ranks = Rank::rebalance(self.items.len());
        let mut changed = Vec::new();
        for (item, rank) in self.items.iter_mut().zip(ranks) {
            if item.rank != rank {
                item.rank = rank;
                changed.push(*item);
            }
        }
        changed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id() -> EntityId {
        EntityId::new()
    }

    fn scope() -> OrderingScope {
        OrderingScope::Context(ContextRef::Inbox)
    }

    fn order_with(count: usize) -> (ScopeOrder, Vec<EntityId>) {
        let mut order = ScopeOrder::new(scope());
        let ids: Vec<EntityId> = (0..count).map(|_| id()).collect();
        for entity in &ids {
            order.append(*entity).expect("append");
        }
        (order, ids)
    }

    #[test]
    fn encoding_preserves_numeric_order_lexicographically() {
        let low = Rank::new(5).expect("rank");
        let high = Rank::new(12345).expect("rank");
        assert!(
            low.encode() < high.encode(),
            "{} !< {}",
            low.encode(),
            high.encode()
        );
        assert_eq!(Rank::parse(&low.encode()).expect("parse"), low);
        assert_eq!(Rank::parse(&high.encode()).expect("parse"), high);
        assert!(Rank::parse("abc").is_err());
        assert!(Rank::parse("").is_err());
    }

    #[test]
    fn first_insert_uses_start_rank() {
        let mut order = ScopeOrder::new(scope());
        let change = order.insert_at(id(), 0).expect("insert");
        assert_eq!(change.rank, Rank::start());
        assert!(change.rebalanced.is_empty());
    }

    #[test]
    fn insert_between_neighbors_only_changes_the_new_item() {
        let (mut order, ids) = order_with(2);
        let before = order.ranks();
        let change = order.insert_at(id(), 1).expect("insert middle");
        assert!(change.rebalanced.is_empty());
        assert!(change.rank > before[0]);
        assert!(change.rank < before[1]);
        assert_eq!(order.items()[0].entity_id, ids[0]);
        assert_eq!(order.items()[2].entity_id, ids[1]);
        assert_eq!(order.ranks()[0], before[0]);
        assert_eq!(order.ranks()[2], before[1]);
    }

    #[test]
    fn append_and_prepend_sort_at_the_ends() {
        let (mut order, _) = order_with(2);
        let before_append = *order.ranks().last().unwrap();
        let appended = order.append(id()).expect("append");
        assert!(appended.rank > before_append);

        let before_prepend = order.ranks()[0];
        let prepended = order.insert_at(id(), 0).expect("prepend");
        assert!(prepended.rank < before_prepend);
        assert_eq!(order.items()[0].rank, prepended.rank);
    }

    #[test]
    fn move_reorders_without_touching_other_scopes() {
        let (mut order, ids) = order_with(3);
        let other_scope = OrderingScope::Today;
        let mut other = ScopeOrder::new(other_scope);
        other.append(ids[0]).expect("append today");
        let today_rank = other.items()[0].rank;

        order.move_item(ids[2], 0).expect("move");
        assert_eq!(order.items()[0].entity_id, ids[2]);
        assert_eq!(order.items()[1].entity_id, ids[0]);
        assert_eq!(order.items()[2].entity_id, ids[1]);
        assert_eq!(other.items()[0].rank, today_rank, "Today must be untouched");
    }

    #[test]
    fn same_entity_can_hold_different_ranks_per_scope() {
        let entity = id();
        let mut context = ScopeOrder::new(scope());
        let mut today = ScopeOrder::new(OrderingScope::Today);
        context.append(entity).expect("context");
        today.append(id()).expect("filler");
        today.append(entity).expect("today");
        let context_rank = context.items()[0].rank;
        let today_rank = today.items()[1].rank;
        assert_ne!(context_rank, today_rank);
        assert_eq!(context.position_of(entity), Some(0));
        assert_eq!(today.position_of(entity), Some(1));
    }

    #[test]
    fn exhausted_gap_reports_exhaustion_instead_of_duplicating() {
        let first = Rank::new(10).expect("rank");
        let second = Rank::new(11).expect("rank");
        let error = Rank::between(Some(first), Some(second), scope(), 1).expect_err("exhausted");
        assert!(matches!(
            error,
            OrderingError::Exhausted { position: 1, .. }
        ));
    }

    #[test]
    fn exhausted_insertion_triggers_rebalance_and_succeeds() {
        let mut order = ScopeOrder::new(scope());
        let a = id();
        let b = id();
        order.items.push(OrderedItem {
            entity_id: a,
            rank: Rank::new(10).expect("rank"),
        });
        order.items.push(OrderedItem {
            entity_id: b,
            rank: Rank::new(11).expect("rank"),
        });

        let change = order.insert_at(id(), 1).expect("insert after rebalance");
        assert!(!change.rebalanced.is_empty(), "rebalance must be reported");
        assert_eq!(order.items().len(), 3);
        assert_eq!(order.items()[0].entity_id, a);
        assert_eq!(order.items()[2].entity_id, b);
        let ranks = order.ranks();
        assert!(ranks.windows(2).all(|pair| pair[0] < pair[1]));
    }

    #[test]
    fn repeated_middle_inserts_stay_ordered_and_bounded() {
        let mut order = ScopeOrder::new(scope());
        let first = id();
        let last = id();
        order.append(first).expect("append");
        order.append(last).expect("append");

        for _ in 0..200 {
            let change = order.insert_at(id(), 1).expect("insert");
            assert!(change.rank > order.ranks()[0]);
            assert!(change.rank < *order.ranks().last().unwrap());
        }
        let ranks = order.ranks();
        assert!(ranks.windows(2).all(|pair| pair[0] < pair[1]));
        assert_eq!(ranks.len(), 202);
    }

    #[test]
    fn rebalance_preserves_visible_order_and_leaves_room() {
        let mut order = ScopeOrder::new(scope());
        let ids: Vec<EntityId> = (0..4).map(|_| id()).collect();
        for (offset, entity) in ids.iter().enumerate() {
            let value = 10 + i64::try_from(offset).expect("offset");
            order.items.push(OrderedItem {
                entity_id: *entity,
                rank: Rank::new(value).expect("rank"),
            });
        }

        let before: Vec<EntityId> = order.items().iter().map(|item| item.entity_id).collect();
        let changed = order.rebalance();
        let after: Vec<EntityId> = order.items().iter().map(|item| item.entity_id).collect();
        assert_eq!(before, after);
        assert_eq!(before, ids);
        assert_eq!(changed.len(), 4);

        let ranks = order.ranks();
        assert!(ranks
            .windows(2)
            .all(|pair| pair[1].value() - pair[0].value() >= 2));
    }

    #[test]
    fn out_of_bounds_indices_are_rejected() {
        let (mut order, _) = order_with(1);
        assert!(matches!(
            order.insert_at(id(), 5),
            Err(OrderingError::OutOfBounds { .. })
        ));
        assert!(matches!(
            order.move_item(id(), 3),
            Err(OrderingError::OutOfBounds { .. })
        ));
        assert!(matches!(
            order.move_item(order.items()[0].entity_id, 9),
            Err(OrderingError::OutOfBounds { .. })
        ));
    }

    #[test]
    fn from_items_sorts_by_rank() {
        let a = id();
        let b = id();
        let order = ScopeOrder::from_items(
            scope(),
            vec![
                OrderedItem {
                    entity_id: b,
                    rank: Rank::new(200).expect("rank"),
                },
                OrderedItem {
                    entity_id: a,
                    rank: Rank::new(100).expect("rank"),
                },
            ],
        );
        assert_eq!(order.items()[0].entity_id, a);
        assert_eq!(order.items()[1].entity_id, b);
    }
}
