# Proposal

## Why

Moves are the most reused mutation in a task manager: the editor Move dialog, drag-and-drop,
keyboard commands, and automation must all validate destinations and preserve state the same
way. Today each surface validates ad hoc, and headings cannot change projects at all.

## What Changes

- Add one authoritative movement engine defining valid destinations for to-dos, projects, and
  headings plus destination existence/ownership/alive checks.
- Route `MoveTodo`, `MoveProject`, and a new `MoveHeading` command through the engine.
- Headings can move between projects, taking their child to-dos with them.
- Surface the heading move control on the project detail page.

## Capabilities

### New Capabilities

- `features/movement`: destination validation, move preservation, and adapter reuse.

### Modified Capabilities

(none)

## Impact

- `crates/app` (`movement` module, `MoveHeading` command/event, engine-backed handlers),
  `crates/infra` (change-log event name), `crates/testkit`, `crates/web` (heading move form and
  route). No schema change.
- Non-goals: drag-and-drop pointer interactions (32), cross-account moves, bulk moves.
