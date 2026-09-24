# Proposal

## Why

Notes are the second-largest text surface after titles. They need dependable multiline editing,
in-note navigation and replacement for long notes, and link handling that can never navigate to
an untrusted scheme.

## What Changes

- Add a pure Rust notes utility module: link detection, safe-URL policy, find/next/previous,
  and deterministic replace-all.
- Render a notes tools panel on to-do and project pages: detected safe links as anchors with
  `noopener noreferrer`, blocked schemes shown as inert text, find with next/previous
  traversal, and replace-all.
- Persist replacements through the existing authoritative note commands.

## Capabilities

### New Capabilities

- `features/notes-editor`: multiline editing behavior, find/replace, and safe link handling.

### Modified Capabilities

(none)

## Impact

- `crates/domain` (notes utilities), `crates/web` (notes panel, replacement routes).
- No schema change and no new commands: notes persist through `SetTodoNotes`/`SetProjectNotes`.
- Non-goals: Markdown rendering (change 19), rich-text storage, browser E2E harness.
