# Proposal

## Why

Sharing and printing need deterministic render models that never depend on interactive markup, and
print output must be a dedicated document rather than the shell printed unchanged; all untrusted
text must stay escaped and scoped.

## What Changes

- Add `momentum_app::share`: `escape_markdown`, `escape_html`, and deterministic models
  `share_todo_markdown`, `share_project_markdown`, `share_list_markdown`, plus dedicated
  print-only HTML documents `print_list_html`/`print_project_html` with their own minimal styles.
- Add routes: `GET /share/todos/{id}`, `/share/projects/{id}`, `/share/lists/{list}` returning
  `text/markdown`; `GET /print/lists/{list}` and `/print/projects/{id}` returning print documents;
  list names come from a fixed allow-list and reads use open scopes (no trashed content).
- Add `@media print` rules so printing any interactive page never prints shell chrome or action
  controls.

## Capabilities

### New Capabilities

- `features/sharing-and-printing`: share render models, print layouts, content safety.

### Modified Capabilities

(none)

## Impact

- `crates/app` (share module + unit tests), `crates/web` (share/print routes, print CSS, tests).
  No schema change.
- Non-goals: public/anonymous share links (session required), PDF generation, social embeds.
