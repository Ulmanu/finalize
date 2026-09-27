//! Multi-selection state machine for selectable list rows.
//!
//! Selection is presentation state: it never reaches persistence or the change log. The
//! machine is deterministic so the hydrated client and server-rendered bulk forms agree on
//! single, toggle, range, select-all, and clear semantics, and stale selections are pruned
//! against the currently visible rows after any list refresh.

use std::collections::BTreeSet;

use momentum_domain::EntityId;

/// Deterministic multi-selection state with an anchor for range selection.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Selection {
    anchor: Option<EntityId>,
    selected: BTreeSet<EntityId>,
}

impl Selection {
    /// Creates an empty selection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the anchor for range extension, when one exists.
    #[must_use]
    pub fn anchor(&self) -> Option<EntityId> {
        self.anchor
    }

    /// Returns the number of selected items.
    #[must_use]
    pub fn len(&self) -> usize {
        self.selected.len()
    }

    /// Returns whether nothing is selected.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.selected.is_empty()
    }

    /// Returns whether an item is selected.
    #[must_use]
    pub fn contains(&self, id: EntityId) -> bool {
        self.selected.contains(&id)
    }

    /// Returns the selected IDs in deterministic (ascending) order.
    #[must_use]
    pub fn ids(&self) -> Vec<EntityId> {
        self.selected.iter().copied().collect()
    }

    /// Selects exactly one item and anchors there.
    pub fn single(&mut self, id: EntityId) {
        self.selected.clear();
        self.selected.insert(id);
        self.anchor = Some(id);
    }

    /// Toggles an item; when it becomes selected the anchor moves to it.
    pub fn toggle(&mut self, id: EntityId) {
        if self.selected.remove(&id) {
            if self.anchor == Some(id) {
                self.anchor = self.selected.iter().next().copied();
            }
        } else {
            self.selected.insert(id);
            self.anchor = Some(id);
        }
    }

    /// Extends the selection from the anchor to `id` over the visible order, replacing any
    /// previous range while keeping the anchor stable.
    pub fn extend_to(&mut self, id: EntityId, visible: &[EntityId]) {
        let Some(anchor) = self.anchor else {
            self.single(id);
            return;
        };
        let (Some(from), Some(to)) = (position(visible, anchor), position(visible, id)) else {
            self.single(id);
            return;
        };
        let (start, end) = if from <= to { (from, to) } else { (to, from) };
        self.selected = visible[start..=end].iter().copied().collect();
    }

    /// Selects every currently visible item; the anchor stays when still visible.
    pub fn select_all(&mut self, visible: &[EntityId]) {
        self.selected = visible.iter().copied().collect();
        if !self.anchor.is_some_and(|id| self.selected.contains(&id)) {
            self.anchor = visible.first().copied();
        }
    }

    /// Clears the selection and the anchor.
    pub fn clear(&mut self) {
        self.selected.clear();
        self.anchor = None;
    }

    /// Prunes selection to items that remain visible, returning the IDs that were dropped.
    pub fn retain_visible(&mut self, visible: &[EntityId]) -> Vec<EntityId> {
        let visible: BTreeSet<EntityId> = visible.iter().copied().collect();
        let removed: Vec<EntityId> = self
            .selected
            .iter()
            .filter(|id| !visible.contains(id))
            .copied()
            .collect();
        self.selected.retain(|id| visible.contains(id));
        if self.anchor.is_some_and(|id| !visible.contains(&id)) {
            self.anchor = self.selected.iter().next().copied();
        }
        removed
    }
}

fn position(visible: &[EntityId], id: EntityId) -> Option<usize> {
    visible.iter().position(|candidate| *candidate == id)
}

#[cfg(test)]
mod tests {
    use super::Selection;
    use momentum_domain::EntityId;
    use uuid::Uuid;

    fn ids(values: &[u128]) -> Vec<EntityId> {
        values
            .iter()
            .copied()
            .map(|value| EntityId::from_uuid(Uuid::from_u128(value)))
            .collect()
    }

    #[test]
    fn toggle_anchor_and_deterministic_ids() {
        let [a, b, c] = <[EntityId; 3]>::try_from(ids(&[1, 2, 3])).expect("three ids");
        let mut selection = Selection::new();
        selection.single(a);
        assert_eq!(selection.ids(), vec![a]);
        assert_eq!(selection.anchor(), Some(a));
        selection.toggle(b);
        selection.toggle(c);
        assert_eq!(selection.ids(), vec![a, b, c]);
        assert_eq!(selection.anchor(), Some(c));
        selection.toggle(c);
        assert_eq!(selection.ids(), vec![a, b]);
        assert_eq!(selection.anchor(), Some(a));
    }

    #[test]
    fn range_selection_replaces_previous_range_and_keeps_anchor() {
        let visible = ids(&[1, 2, 3, 4, 5]);
        let [a, c, d] = <[EntityId; 3]>::try_from(ids(&[1, 3, 4])).expect("three ids");
        let mut selection = Selection::new();
        selection.single(c);
        selection.extend_to(a, &visible);
        assert_eq!(selection.ids(), ids(&[1, 2, 3]));
        selection.extend_to(d, &visible);
        assert_eq!(selection.ids(), ids(&[3, 4]));
        assert_eq!(selection.anchor(), Some(c));
    }

    #[test]
    fn extend_without_anchor_or_visibility_selects_single() {
        let visible = ids(&[1, 2]);
        let [a, b, hidden] = <[EntityId; 3]>::try_from(ids(&[1, 2, 9])).expect("three ids");
        let mut selection = Selection::new();
        selection.extend_to(a, &visible);
        assert_eq!(selection.ids(), vec![a]);
        selection.extend_to(hidden, &visible);
        assert_eq!(selection.ids(), vec![hidden]);
        selection.single(a);
        selection.extend_to(b, &[]);
        assert_eq!(selection.ids(), vec![b]);
    }

    #[test]
    fn select_all_then_clear() {
        let visible = ids(&[7, 8, 9]);
        let [a, b] = <[EntityId; 2]>::try_from(ids(&[7, 8])).expect("two ids");
        let mut selection = Selection::new();
        selection.single(a);
        selection.select_all(&visible);
        assert_eq!(selection.ids(), visible);
        assert_eq!(selection.anchor(), Some(a));
        selection.single(b);
        selection.select_all(&visible);
        assert_eq!(selection.ids(), visible);
        assert_eq!(selection.anchor(), Some(b));
        selection.clear();
        assert!(selection.is_empty());
        assert_eq!(selection.anchor(), None);
    }

    #[test]
    fn retain_visible_prunes_stale_ids_and_resets_anchor() {
        let visible = ids(&[1, 3]);
        let [a, b, c] = <[EntityId; 3]>::try_from(ids(&[1, 2, 3])).expect("three ids");
        let mut selection = Selection::new();
        selection.single(a);
        selection.toggle(b);
        selection.toggle(c);
        let removed = selection.retain_visible(&visible);
        assert_eq!(removed, vec![b]);
        assert_eq!(selection.ids(), vec![a, c]);
        assert_eq!(selection.anchor(), Some(c));
        selection.single(b);
        let removed = selection.retain_visible(&visible);
        assert_eq!(removed, vec![b]);
        assert!(selection.is_empty());
        assert_eq!(selection.anchor(), None);
    }
}
