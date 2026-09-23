# Spec delta — domain/ordering

## ADDED Requirements

### Requirement: Independent ordering scopes

The system SHALL maintain independent manual ordering for contextual lists such as
Inbox/project/heading and derived lists such as Today.

#### Scenario: The same item holds different positions per scope

- **GIVEN** an entity placed in a contextual list and in Today
- **WHEN** its Today position changes
- **THEN** its contextual position is unchanged
- **AND** the two scopes store independent ranks

#### Scenario: Ordering another scope does not disturb this scope

- **GIVEN** two ordering scopes
- **WHEN** items are inserted, moved, or rebalanced in one scope
- **THEN** the other scope's ordered items are unchanged

### Requirement: Fractional insertion

Ordering SHALL support insertion between neighboring items without renumbering an entire list.

#### Scenario: Insert between neighbors

- **GIVEN** two adjacent items with a free gap between their ranks
- **WHEN** a new item is inserted between them
- **THEN** only the new item receives a new rank
- **AND** existing ranks are unchanged

#### Scenario: Append and prepend

- **GIVEN** a non-empty ordered list
- **WHEN** an item is appended or prepended
- **THEN** its rank sorts after or before every existing item respectively

### Requirement: Rank maintenance

The ordering engine SHALL detect exhausted rank space and rebalance a scope deterministically
without changing visible order.

#### Scenario: Exhausted gap triggers rebalance

- **GIVEN** two adjacent ranks with no integer gap between them
- **WHEN** an insertion is planned between them
- **THEN** the engine reports exhaustion instead of duplicating ranks

#### Scenario: Rebalance preserves visible order

- **GIVEN** an ordered list whose ranks are exhausted
- **WHEN** the scope is rebalanced
- **THEN** every item keeps its previous relative order
- **AND** the resulting ranks are strictly increasing with room for future insertions
