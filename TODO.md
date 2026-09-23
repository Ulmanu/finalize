# TODO — Autonomous Things-style Full-Stack Rust Web App

> **This file is the entire execution plan.**
>
> The coding agent must read this file from top to bottom and then execute it autonomously.
> Do not ask the user for intermediate approval. Do not stop between numbered changes.
> Do not pre-create all OpenSpec changes. Materialize **one change at a time**, complete it,
> validate it, verify it, archive it, commit it, then move to the next change.
>
> Target: a web application reproducing the observable workflow, information architecture,
> interaction quality, and major functionality of Things, implemented as a full-stack Rust app.
> Use original branding and original assets; do not ship Cultured Code trademarks, logos,
> marketing illustrations, or proprietary icons.

## Web-verified planning basis

This plan was cross-checked against current official documentation before generation:

- OpenSpec's built-in `spec-driven` workflow uses `proposal.md`, delta specs, `design.md`,
  and `tasks.md`, with the dependency flow `proposal → specs/design → tasks → apply`.
- OpenSpec project configuration supports `schema`, shared `context`, artifact-specific `rules`,
  and operation guidance for `apply` and `archive`.
- Things' official support documentation confirms the core model around Today, Upcoming,
  Anytime, Someday, projects, areas, headings, tags, checklists, repeating items, reminders,
  calendar events, Quick Find, natural-language dates, keyboard interaction, gestures,
  Markdown notes, Magic Plus, and related navigation/automation features.
- Today is explicitly a cross-app filter/projection rather than a storage folder; This Evening
  is a separate Today section. This plan treats all built-in date lists as projections.

---

# PART 0 — AUTONOMOUS AGENT INSTRUCTIONS

# Autonomous Overnight Agent — Things-style Full-Stack Rust Web App

You are the autonomous implementation agent for this repository.

This is an unattended sequential run. Do not wait for user approval between changes. Do not ask whether to continue. You own proposal, implementation, validation, verification, archive, and Git commit for every change in the queue.

## Primary objective

Perform bootstrap step `00`, then execute implementation changes `01` through `74` in order. Each change must finish its entire lifecycle before the next change starts.

For each implementation change:

1. Create the OpenSpec proposal and all planning artifacts.
2. Review the artifacts yourself against the current repository, current live specs, this roadmap, and prior archived changes.
3. Run strict non-interactive OpenSpec validation.
4. Repair proposal/spec/design/task problems automatically until validation passes.
5. Apply the change completely.
6. Implement every legitimate task.
7. Add or update tests.
8. Run all quality gates relevant to the affected code.
9. Diagnose and repair failures; never weaken a requirement just to make tests pass.
10. Run implementation verification against the OpenSpec artifacts when the installed OpenSpec workflow provides `/opsx:verify`. If that command is unavailable, perform the same verification manually: check completeness, correctness, and coherence against proposal/specs/design/tasks and record that fallback in the overnight report.
11. Resolve all critical implementation/spec mismatches and all actionable warnings.
12. Strictly validate the change again.
13. Archive it non-interactively.
14. Strictly validate the resulting live specs and archived task completeness.
15. Inspect the complete Git diff for accidental or unrelated changes.
16. Commit the completed archived change.
17. Record the result and commit SHA in `OVERNIGHT_REPORT.md`.
18. Continue immediately to the next change.

Never archive an incomplete change just to increase the completion count.


## Mandatory per-change blueprint

Before proposing numbered change `NN-name`, read:

- `openspec-blueprints/NN-name/proposal-seed.md`
- `openspec-blueprints/NN-name/spec-seed.md`
- `openspec-blueprints/NN-name/design-seed.md`
- `openspec-blueprints/NN-name/tasks-seed.md`

These files contain the intended detailed scope, requirements/scenarios, design constraints, and implementation checklist. They are planning seeds, not files to copy blindly.

The agent MUST reconcile each seed with:
1. the repository as it exists after the previous archived change;
2. the current canonical `openspec/specs/`;
3. `openspec instructions <artifact> --change <name>`;
4. any requirement that was already introduced/modified by an earlier change.

Use `ADDED`, `MODIFIED`, `REMOVED`, or `RENAMED` correctly. Do not leave an `ADDED` requirement in the real delta if the canonical spec already contains that requirement; convert the delta appropriately.

Do not pre-create all 74 active changes. Only materialize the current change into `openspec/changes/`.

## OpenSpec lifecycle

For change `<name>`:

```text
PROPOSE
→ STRICT VALIDATE
→ APPLY
→ TEST
→ VERIFY
→ FIX
→ STRICT VALIDATE
→ ARCHIVE
→ VALIDATE LIVE SPECS + ARCHIVED TASKS
→ COMMIT
→ NEXT CHANGE
```

Use the installed OpenSpec agent command for proposal:

```text
/opsx:propose <name>
```

Strict validation:

```bash
openspec validate <name> --strict --no-interactive
```

Apply:

```text
/opsx:apply <name>
```

Verification when available:

```text
/opsx:verify <name>
```

Archive:

```bash
openspec archive <name> --yes
```

Post-archive checks:

```bash
openspec validate --specs --strict --no-interactive
openspec validate --archived --strict --no-interactive
```

Do not use archive options that bypass validation.

## No user interaction

Do not ask the user questions during the unattended run.

When an implementation decision is required:

1. inspect the current code;
2. inspect current live OpenSpec specs;
3. inspect previously archived changes;
4. inspect the project roadmap;
5. choose the narrowest option consistent with those sources;
6. record non-obvious decisions in `design.md` and the overnight report;
7. continue.

If requirements are ambiguous, prefer:
- established architecture;
- the documented observable behavior of Things used as the parity reference;
- domain consistency over UI hacks;
- the narrowest implementation satisfying the current proposal.

A missing live credential is normally not a blocker. Implement the provider-neutral adapter, configuration, deterministic fake/mock, and tests that do not require the secret. Record the unverified live integration and continue.

Never invent credentials.

## Hard blockers

Only treat a situation as blocked when work is genuinely impossible without unavailable external information or when continuing would require destructive/unsafe behavior.

For a hard blocker:
- do not archive the incomplete change;
- do not mark incomplete tasks done;
- leave the worktree consistent;
- record `BLOCKED` in `OVERNIGHT_REPORT.md`;
- record cause, commands attempted, remaining work, and dependency impact;
- continue with later changes only when they do not depend on the blocker.

## Git policy

Work on an isolated branch, preferably:

```text
agent/things-web-overnight
```

If the user's current working tree has pre-existing uncommitted changes, create/use an isolated worktree or preserve those changes without touching them.

Never:
- reset or discard pre-existing user work;
- force-push;
- rewrite published history;
- merge into main/master;
- deploy production without an explicit existing repository instruction;
- delete production data;
- commit secrets.

Each numbered implementation change should end in one normal completion commit after archive and validation.

Commit style:

```text
feat(01): bootstrap full-stack Rust workspace
feat(22): add Today projection
test(71): add visual parity suite
fix(74): complete parity hardening
```

Do not commit knowingly failing code.

## Rust-only source policy

Application/business logic is Rust.

Allowed:
- Rust source;
- Cargo manifests;
- configuration/data files;
- generated HTML from Rust views;
- CSS/styling resources;
- browser manifests;
- WASM output;
- JavaScript glue automatically generated by Rust/WASM tooling.

Forbidden:
- hand-written JavaScript application logic;
- hand-written TypeScript;
- React/Vue/Svelte/Next.js application layers;
- Node business/backend logic.

Shared domain/application behavior must remain independent of Leptos, Axum, PostgreSQL, and browser APIs unless it is intentionally inside an adapter/infrastructure crate.

## Architectural invariants

1. Built-in Things lists are projections, not storage folders.
2. A Todo has one canonical identity.
3. A Todo may simultaneously appear in its context, Today, search, and tag-filtered projections.
4. Projection ordering must not accidentally change contextual ordering.
5. UI components do not own authoritative business rules.
6. Mutations go through the application command layer.
7. Offline mutations are optimistic and eventually synchronized.
8. Start date, deadline, and reminder are distinct concepts.
9. User-local dates are modeled as dates, not fake UTC midnight instants.
10. External integrations are behind provider-neutral interfaces.
11. Sync operations are idempotent.
12. Recurrence materialization is idempotent and concurrency-safe.
13. Conflict handling must not silently destroy meaningful state.
14. Every persisted schema change gets a migration.
15. User-visible behavior gets tests.
16. Desktop/tablet/mobile behavior is explicit where it differs.
17. Accessibility is part of feature completion.
18. Visual parity uses original app assets/branding, never Cultured Code logos or proprietary marketing/icon assets.

## Quality gates

Run what exists and what is relevant. At minimum, when available:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

If configured:

```bash
cargo nextest run --workspace --all-features
```

Also run:
- WASM/web build checks;
- migration tests when persistence changes;
- browser integration/E2E tests when UI behavior changes;
- accessibility tests/checks when UI changes;
- visual regression once change 71 exists;
- strict OpenSpec validation.

A task is complete only after its acceptance condition passes.

## Failure recovery

For a failing gate:
1. read the failure;
2. identify root cause;
3. fix it;
4. rerun the smallest affected test;
5. rerun the complete required gate;
6. continue.

Never:
- delete/disable a test merely because it exposes a real defect;
- remove a requirement because it is difficult;
- globally suppress Clippy to hide errors;
- introduce broad production `unwrap()`/`expect()` use just to finish faster.

## Things parity policy

Reproduce observable workflow, information architecture, interaction behavior, responsiveness, and design quality.

Do not copy:
- Cultured Code trademarks;
- product logo;
- copyrighted marketing illustrations;
- proprietary icon assets.

Use original equivalent branding/assets.

Where an Apple-native feature cannot literally exist in a browser, implement the specified web equivalent.

## Overnight report

Maintain `OVERNIGHT_REPORT.md`.

For every change record:
- number;
- OpenSpec change name;
- start/end time;
- proposal validation result;
- implementation result;
- tests executed;
- verification result;
- archive result;
- commit SHA;
- important design decisions;
- deferred environmental/live-integration checks;
- status: `COMPLETE`, `BLOCKED`, or `SKIPPED-DUE-TO-DEPENDENCY`.

At run end include:
- total changes complete;
- total blocked;
- total commits;
- test status;
- OpenSpec validation state;
- branch;
- HEAD SHA;
- known follow-ups;
- how to start the app;
- how to run tests;
- how to inspect the implementation.

## No time-based stopping

Do not stop merely because an arbitrary number of changes has been completed.

Continue while the execution environment permits safe actionable work.

It is better to finish 25 changes correctly than archive 74 incomplete changes.

## Bootstrap 00

Before change 01:

- inspect repository and existing OpenSpec state;
- create/confirm isolated branch/worktree;
- install or confirm project OpenSpec structure without destroying existing configuration;
- confirm `openspec/config.yaml` uses the project rules in this bundle;
- determine whether `/opsx:verify` exists;
- create/initialize `OVERNIGHT_REPORT.md`;
- record tool versions;
- run a baseline build/test/validation and record existing failures separately;
- freeze the observable parity reference and document any known platform-specific web substitutions;
- make one bootstrap commit only if bootstrap files actually changed.

Then execute changes 01 through 74 sequentially from `THINGS_WEB_ROADMAP.md`.


---

# PART 1 — BOOTSTRAP 00

# 00 — Autonomous bootstrap

This is a setup phase, not one of the 74 implementation proposals.

The coding agent must:

1. Inspect the repository, current branch, worktree state, existing OpenSpec files, Rust toolchain, and existing test commands.
2. Preserve all pre-existing user changes.
3. Create or switch to an isolated `agent/things-web-overnight` branch/worktree when safe.
4. Ensure `openspec/config.yaml` contains this bundle's project rules without discarding repository-specific context that should be retained.
5. Detect whether OpenSpec's `/opsx:verify` workflow is already installed. If not, do not block the run; use explicit manual implementation-vs-artifact verification and record that fallback.
6. Record OpenSpec and Rust versions in `OVERNIGHT_REPORT.md`.
7. Run baseline build/tests/OpenSpec validation. Record pre-existing failures distinctly so later changes are not blamed for them.
8. Freeze the parity policy:
   - observable Things-style behavior and structure are reference targets;
   - Apple-native-only integrations receive web equivalents;
   - Cultured Code branding/proprietary assets are not copied.
9. Confirm the Rust-only application-source policy.
10. Commit bootstrap files only if they changed and only after reviewing the bootstrap diff.
11. Begin change 01 immediately.

Bootstrap must not pre-create all 74 active OpenSpec changes.


---

# PART 2 — MASTER IMPLEMENTATION QUEUE

# Things-style Full-Stack Rust Web App — OpenSpec Roadmap

## Bootstrap 00

`00` is a one-time autonomous setup step, not an implementation proposal.

Deliver:
- parity/reference policy;
- Rust-only source policy;
- OpenSpec project rules;
- isolated Git branch/worktree;
- baseline test/validation status;
- overnight report;
- original-branding/asset policy;
- web substitutions for Apple-native-only integrations.

## Implementation queue

Run strictly in numerical order. Each change is proposed, strictly validated, applied, tested, verified, repaired, archived, post-archive validated, and committed before the next change begins.

| # | OpenSpec change | Required completed outcome |
|---:|---|---|
| 01 | `bootstrap-fullstack-rust-workspace` | Cargo workspace, frontend/backend/shared crates, CI-quality commands, Leptos/Axum foundation, WASM build |
| 02 | `add-web-application-shell` | Routing, SSR/hydration shell, authenticated/public layouts, error/loading boundaries |
| 03 | `add-things-design-system` | Central typography, spacing, radius, surface, row, sidebar, icon and motion primitives |
| 04 | `add-responsive-layout-foundation` | Desktop, tablet and mobile application shells with parity-oriented navigation |
| 05 | `add-persistence-foundation` | PostgreSQL model, Rust migrations, IDs, timestamps, tombstones/version fields |
| 06 | `add-account-authentication` | Registration, verification, login/logout, sessions, reset, revocation and account deletion |
| 07 | `add-task-domain-core` | Canonical Todo/Project/Area/Heading/Checklist/Tag types and domain invariants |
| 08 | `add-fractional-ordering-engine` | Stable independent ordering across Inbox, project, Today, headings and sidebar scopes |
| 09 | `add-command-transaction-layer` | Typed commands, validation, domain transactions, events, projection invalidation and sync hooks |
| 10 | `add-undo-redo-engine` | Reversible mutation model and grouped undo semantics |
| 11 | `add-todo-crud` | Create, edit, duplicate, move, complete, cancel, reopen and delete lifecycle |
| 12 | `add-inbox` | Capture workflow, Inbox projection, badge, ordering, processing and movement |
| 13 | `add-areas` | Area creation, ordering, loose tasks, project grouping and tagging |
| 14 | `add-projects` | Project lifecycle, membership, notes, schedule, deadline, ordering and status |
| 15 | `add-project-headings` | Create/edit/reorder/archive headings and move associated child tasks |
| 16 | `add-checklists` | Ordered embedded checklist editing, completion, paste expansion and reordering |
| 17 | `add-tags` | Tag CRUD, hierarchy, assignment, inherited context and multi-tag filtering |
| 18 | `add-notes-editor` | Multiline notes, links, selection, find/replace and editing behavior |
| 19 | `add-things-markdown` | Things-style Markdown subset with syntax-visible rich rendering |
| 20 | `add-scheduling-model` | Anytime/Someday/on-date/Today/This Evening scheduling semantics |
| 21 | `add-deadlines` | Independent deadline model, overdue/due-today/future states and sorting |
| 22 | `add-today-projection` | Derived Today view, This Evening, manual ordering and correct membership |
| 23 | `add-upcoming-projection` | Date-grouped future agenda including rescheduling behavior |
| 24 | `add-anytime-projection` | Correct currently-actionable projection and exclusions |
| 25 | `add-someday-projection` | Deferred/inactive tasks and projects with correct transitions |
| 26 | `add-completion-and-cancellation` | Distinct completed/canceled semantics, project handling and reopening |
| 27 | `add-logbook` | Completion history, date grouping, canceled items, projects and logging behavior |
| 28 | `add-trash` | Recoverable deletion, restore, permanent deletion and empty-trash flow |
| 29 | `add-special-lists` | Tomorrow, Deadlines, Repeating, All Projects and Logged Projects |
| 30 | `add-project-progress` | Derived project progress rings and Today completion progress |
| 31 | `add-move-engine` | One authoritative movement engine for tasks/projects/headings and destinations |
| 32 | `add-drag-and-drop` | Reorder/move/date-reschedule behavior across desktop and touch input |
| 33 | `add-multi-selection` | Range/toggle/select-all state plus bulk commands and multi-item movement |
| 34 | `add-magic-plus` | Context-aware Things-style creation control and insertion positioning |
| 35 | `add-expanded-item-editor` | Smooth row-to-expanded editor with title, notes and optional metadata controls |
| 36 | `add-date-pickers` | When/deadline/reminder date interfaces and quick destinations |
| 37 | `add-natural-language-dates` | Deterministic Rust date parser with tested supported expressions |
| 38 | `add-reminders` | Reminder persistence, scheduler, browser Web Push and routing |
| 39 | `add-reminder-snoozing` | Snooze durations, tomorrow behavior and scheduler integration |
| 40 | `add-recurrence-model` | Regular and after-completion recurrence domain including end conditions |
| 41 | `add-recurrence-materializer` | Concurrent-safe, idempotent generation of recurrence instances |
| 42 | `add-recurrence-exceptions` | Early completion, one-time schedule overrides and future-rule editing |
| 43 | `add-quick-find` | Type-to-search navigation across entities, tags, lists and settings |
| 44 | `add-deep-search` | Search notes/checklists/logged content with ranking and normalization |
| 45 | `add-keyboard-command-system` | Central command/shortcut registry covering navigation and task operations |
| 46 | `add-type-travel` | Typing outside editors immediately invokes navigation/search behavior |
| 47 | `add-mobile-gestures` | Swipe, pull, long-press, drag and touch multi-selection state machines |
| 48 | `add-calendar-integration` | Provider-neutral calendar interface plus initial web calendar adapters |
| 49 | `add-offline-local-replica` | IndexedDB local replica, optimistic writes, outbox and offline startup |
| 50 | `add-cloud-sync-protocol` | Incremental push/pull, cursors, operation IDs, versions and conflict rules |
| 51 | `add-realtime-sync` | WebSocket/SSE invalidation hints feeding the canonical sync protocol |
| 52 | `add-multitab-coordination` | BroadcastChannel/local leadership and synchronized browser-tab replicas |
| 53 | `add-quick-entry` | Keyboard-invoked capture overlay that saves without leaving current context |
| 54 | `add-browser-capture` | PWA/share/browser capture of title, URL and selected text |
| 55 | `add-mail-to-inbox` | Secure inbound email address translating mail to Inbox tasks |
| 56 | `add-command-urls` | Web equivalents for add/update/show/search automation URLs |
| 57 | `add-automation-api` | Scoped personal API tokens and REST automation over application commands |
| 58 | `add-pwa` | Installability, manifest, offline application shell, shortcuts and update behavior |
| 59 | `add-web-companion-views` | Compact Today/progress/quick-add views equivalent to native companion surfaces |
| 60 | `add-multiple-window-support` | Popout list/project/area windows sharing synchronized local state |
| 61 | `add-import-export` | Versioned JSON backups plus CSV/Markdown/import adapters |
| 62 | `add-sharing-and-printing` | Deterministic share and print render models |
| 63 | `add-settings` | General, appearance, Today, calendar, notifications, account, sync and data settings |
| 64 | `add-theme-and-text-sizing` | Light/dark/system themes and accessibility-aware text sizing |
| 65 | `add-localization` | Locale, week start, hour format, timezone and parser-locale infrastructure |
| 66 | `add-accessibility` | Complete keyboard access, semantics, screen-reader behavior, focus and reduced motion |
| 67 | `add-animation-system` | Centralized insertion/removal/editor/popover/drag/navigation motion primitives |
| 68 | `add-large-list-performance` | Benchmarks and optimizations for thousands to tens of thousands of items |
| 69 | `add-security-hardening` | Argon2id, secure sessions, CSRF/CSP, rate limiting, isolation and API authorization |
| 70 | `add-observability` | Structured tracing, request/sync/scheduler/integration diagnostics and privacy-safe metrics |
| 71 | `add-visual-parity-suite` | Fixed-viewport screenshot baselines and Rust-driven visual regression checks |
| 72 | `add-behavior-parity-suite` | End-to-end GIVEN/WHEN/THEN parity scenarios across all major workflows |
| 73 | `add-production-deployment` | Reproducible deployment, health/readiness, backups, migrations and worker topology |
| 74 | `things-web-parity-hardening` | Full audit against parity matrix; eliminate remaining implementation/spec/test gaps |

## Dependency rule

Default dependency is sequential:

```text
01 → 02 → 03 → … → 74
```

Do not pre-create all 74 active OpenSpec changes. Change `N+1` must be planned against the code and canonical specs left by archived change `N`.

If a change is blocked, later changes may continue only if they genuinely do not depend on the blocked capability.

## Completion definition

A numbered change is `COMPLETE` only when:
- all required OpenSpec planning artifacts exist;
- strict change validation passes;
- all legitimate tasks are checked;
- implementation-specific tests pass;
- implementation is verified against the artifacts;
- the change is archived;
- live spec validation passes;
- archived task completeness validation passes;
- the resulting diff is reviewed;
- the change has a normal Git commit;
- the report records the commit SHA.

Never archive solely to increase the completed-count metric.

---

# PART 3 — GLOBAL OPEN SPEC RULES

The following rules apply to **every** numbered change.

## Required lifecycle

For `NN-change-name`:

```text
READ THIS CHANGE'S FULL SECTION
        ↓
INSPECT CURRENT REPOSITORY + CURRENT CANONICAL OPENSPEC SPECS
        ↓
CREATE CURRENT REAL OPENSPEC CHANGE ONLY
        ↓
/opsx:propose NN-change-name
        ↓
RECONCILE PROPOSAL / DELTA SPECS / DESIGN / TASKS WITH THIS TODO
        ↓
openspec validate NN-change-name --strict --no-interactive
        ↓
FIX UNTIL PASS
        ↓
/opsx:apply NN-change-name
        ↓
IMPLEMENT EVERY TASK
        ↓
RUN AFFECTED TESTS
        ↓
RUN QUALITY GATES
        ↓
/opsx:verify NN-change-name  (when installed)
        ↓
MANUAL ARTIFACT↔IMPLEMENTATION VERIFICATION IF VERIFY IS UNAVAILABLE
        ↓
FIX ALL CRITICAL/ACTIONABLE FINDINGS
        ↓
openspec validate NN-change-name --strict --no-interactive
        ↓
openspec archive NN-change-name --yes
        ↓
openspec validate --specs --strict --no-interactive
        ↓
openspec validate --archived --strict --no-interactive
        ↓
REVIEW COMPLETE GIT DIFF
        ↓
UPDATE OVERNIGHT_REPORT.md
        ↓
COMMIT
        ↓
NEXT NUMBER
```

## Never do this

- Never archive incomplete work.
- Never mark tasks complete without acceptance evidence.
- Never skip strict validation because a proposal is inconvenient.
- Never create all 74 live changes up front.
- Never implement feature-local business rules directly in Leptos components.
- Never add handwritten JS/TS application logic.
- Never use future roadmap changes to excuse an incomplete current proposal.
- Never overwrite pre-existing user work.
- Never force-push, reset user work, or merge to main/master.
- Never hide failing tests.
- Never use a provider credential that is not already available.
- Never claim live external integration was tested if only mocks were available.

## Canonical architecture invariants

1. One canonical Todo identity.
2. Built-in date lists are projections, not folders.
3. Context order and projection order are independent.
4. Start/schedule, deadline, and reminder are separate domain concepts.
5. Date-only values remain local dates, not fake UTC-midnight instants.
6. Authoritative mutations use the application command layer.
7. UI reads query/projection state and dispatches commands.
8. PostgreSQL and browser persistence are adapters behind domain/application layers.
9. Offline mutations are optimistic and durable in an outbox.
10. Sync and recurrence operations are idempotent.
11. External integrations use provider-neutral interfaces.
12. Authorization is account-scoped at every boundary.
13. Migrations are Rust-managed and tested.
14. Accessibility is part of feature completion.
15. Original product branding/assets must be used.

## Standard acceptance gates

Where applicable, run:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

If configured:

```bash
cargo nextest run --workspace --all-features
```

Also run relevant:
- WASM/web build;
- migrations;
- persistence integration tests;
- browser E2E tests;
- accessibility tests;
- sync concurrency/idempotency tests;
- visual regression after its harness exists;
- production build after deployment support exists.

---

# PART 4 — ALL 74 DETAILED OPENSPEC BLUEPRINTS

**Important:** Every section below contains the intended `proposal`, delta `spec`,
`design`, and `tasks` content for that change. The agent must adapt the delta keyword
(`ADDED`, `MODIFIED`, `REMOVED`, `RENAMED`) to the canonical specs that exist at that
moment. Do not blindly use `ADDED` when an earlier archived change already introduced
the same capability.



---

# 01 — `bootstrap-fullstack-rust-workspace`

**Required completed outcome:** Cargo workspace, frontend/backend/shared crates, CI-quality commands, Leptos/Axum foundation, WASM build

## Proposal blueprint

## Why

This change delivers **Cargo workspace, frontend/backend/shared crates, CI-quality commands, Leptos/Axum foundation, WASM build** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Rust workspace topology: The system SHALL provide a Cargo workspace separating web UI, domain/application logic, persistence/infrastructure, and test support so dependency direction can be enforced.
- Full-stack Rust execution path: The web application SHALL use Rust for server logic and Rust-to-WASM for client application logic, with no handwritten JavaScript or TypeScript business/application layer.
- Baseline quality gates: The repository SHALL expose repeatable formatting, linting, test, and web-build commands suitable for unattended validation.

## Dependencies

- Bootstrap `00` only.

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Cargo workspace, frontend/backend/shared crates, CI-quality commands, Leptos/Axum foundation, WASM build.

## ADDED Requirements

### Requirement: Rust workspace topology

The system SHALL provide a Cargo workspace separating web UI, domain/application logic, persistence/infrastructure, and test support so dependency direction can be enforced.

#### Scenario: Rust workspace topology — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the workspace is built from a clean checkout
- **THEN** all workspace crates compile with no forbidden dependency cycles

#### Scenario: Rust workspace topology — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Full-stack Rust execution path

The web application SHALL use Rust for server logic and Rust-to-WASM for client application logic, with no handwritten JavaScript or TypeScript business/application layer.

#### Scenario: Full-stack Rust execution path — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the web app is built for server and browser targets
- **THEN** the server binary and WASM client are produced from Rust sources

#### Scenario: Full-stack Rust execution path — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Baseline quality gates

The repository SHALL expose repeatable formatting, linting, test, and web-build commands suitable for unattended validation.

#### Scenario: Baseline quality gates — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the autonomous agent runs the baseline quality suite
- **THEN** every configured gate exits successfully or a pre-existing failure is recorded before feature work begins

#### Scenario: Baseline quality gates — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Cargo workspace, frontend/backend/shared crates, CI-quality commands, Leptos/Axum foundation, WASM build** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Rust workspace topology`
- `Full-stack Rust execution path`
- `Baseline quality gates`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Rust workspace topology` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Full-stack Rust execution path` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Baseline quality gates` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Rust workspace topology`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Full-stack Rust execution path`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Baseline quality gates`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify bootstrap-fullstack-rust-workspace` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate bootstrap-fullstack-rust-workspace --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive bootstrap-fullstack-rust-workspace --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 02 — `add-web-application-shell`

**Required completed outcome:** Routing, SSR/hydration shell, authenticated/public layouts, error/loading boundaries

## Proposal blueprint

## Why

This change delivers **Routing, SSR/hydration shell, authenticated/public layouts, error/loading boundaries** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Route-addressable application shell: The system SHALL expose stable routes for Inbox, Today, Upcoming, Anytime, Someday, Logbook, areas, projects, and individual to-dos.
- SSR and hydration continuity: The application shell SHALL render server-side and hydrate into interactive Rust/WASM behavior without replacing canonical route or session state.
- Application boundary handling: The shell SHALL provide explicit loading, not-found, unauthorized, and recoverable error boundaries.

## Dependencies

- `bootstrap-fullstack-rust-workspace`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Routing, SSR/hydration shell, authenticated/public layouts, error/loading boundaries.

## ADDED Requirements

### Requirement: Route-addressable application shell

The system SHALL expose stable routes for Inbox, Today, Upcoming, Anytime, Someday, Logbook, areas, projects, and individual to-dos.

#### Scenario: Route-addressable application shell — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user opens or refreshes a supported application URL
- **THEN** the same logical destination renders without relying on transient client state

#### Scenario: Route-addressable application shell — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: SSR and hydration continuity

The application shell SHALL render server-side and hydrate into interactive Rust/WASM behavior without replacing canonical route or session state.

#### Scenario: SSR and hydration continuity — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** an authenticated route is loaded directly
- **THEN** server-rendered content becomes interactive without duplicating or losing the active destination

#### Scenario: SSR and hydration continuity — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Application boundary handling

The shell SHALL provide explicit loading, not-found, unauthorized, and recoverable error boundaries.

#### Scenario: Application boundary handling — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a route load or data dependency fails
- **THEN** the user receives the appropriate non-destructive state and can recover or navigate elsewhere

#### Scenario: Application boundary handling — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Routing, SSR/hydration shell, authenticated/public layouts, error/loading boundaries** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Route-addressable application shell`
- `SSR and hydration continuity`
- `Application boundary handling`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Route-addressable application shell` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `SSR and hydration continuity` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Application boundary handling` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Route-addressable application shell`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `SSR and hydration continuity`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Application boundary handling`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-web-application-shell` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-web-application-shell --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-web-application-shell --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 03 — `add-things-design-system`

**Required completed outcome:** Central typography, spacing, radius, surface, row, sidebar, icon and motion primitives

## Proposal blueprint

## Why

This change delivers **Central typography, spacing, radius, surface, row, sidebar, icon and motion primitives** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Central visual tokens: The UI SHALL define centralized typography, spacing, radius, surface, shadow, icon, row, sidebar, and motion tokens instead of duplicating literal values across features.
- Reusable Things-style primitives: The UI SHALL provide reusable task rows, project rows, area/sidebar rows, check controls, progress rings, section headings, popovers, context menus, date controls, and add controls.
- Original asset policy: The product SHALL reproduce the intended visual structure with original branding and original icons/assets rather than redistributing Cultured Code proprietary assets.

## Dependencies

- `add-web-application-shell`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Central typography, spacing, radius, surface, row, sidebar, icon and motion primitives.

## ADDED Requirements

### Requirement: Central visual tokens

The UI SHALL define centralized typography, spacing, radius, surface, shadow, icon, row, sidebar, and motion tokens instead of duplicating literal values across features.

#### Scenario: Central visual tokens — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a component adopts the design system
- **THEN** its visual geometry is derived from shared tokens

#### Scenario: Central visual tokens — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Reusable Things-style primitives

The UI SHALL provide reusable task rows, project rows, area/sidebar rows, check controls, progress rings, section headings, popovers, context menus, date controls, and add controls.

#### Scenario: Reusable Things-style primitives — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a feature needs a standard interaction surface
- **THEN** it composes the shared primitive rather than implementing a one-off visual copy

#### Scenario: Reusable Things-style primitives — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Original asset policy

The product SHALL reproduce the intended visual structure with original branding and original icons/assets rather than redistributing Cultured Code proprietary assets.

#### Scenario: Original asset policy — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a visual primitive requires an icon or branded asset
- **THEN** the implementation uses a project-owned equivalent

#### Scenario: Original asset policy — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Central typography, spacing, radius, surface, row, sidebar, icon and motion primitives** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Central visual tokens`
- `Reusable Things-style primitives`
- `Original asset policy`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Central visual tokens` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Reusable Things-style primitives` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Original asset policy` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Central visual tokens`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Reusable Things-style primitives`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Original asset policy`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-things-design-system` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-things-design-system --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-things-design-system --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 04 — `add-responsive-layout-foundation`

**Required completed outcome:** Desktop, tablet and mobile application shells with parity-oriented navigation

## Proposal blueprint

## Why

This change delivers **Desktop, tablet and mobile application shells with parity-oriented navigation** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Desktop layout: The application SHALL provide a desktop layout with persistent navigation and content suitable for wide screens.
- Tablet layout: The application SHALL provide a tablet layout that preserves full functionality while allowing navigation to collapse or overlay.
- Mobile layout: The application SHALL provide a single-column mobile navigation model with touch-sized controls and full feature reachability.

## Dependencies

- `add-things-design-system`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Desktop, tablet and mobile application shells with parity-oriented navigation.

## ADDED Requirements

### Requirement: Desktop layout

The application SHALL provide a desktop layout with persistent navigation and content suitable for wide screens.

#### Scenario: Desktop layout — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the viewport meets the desktop breakpoint
- **THEN** sidebar and primary list/editor surfaces remain simultaneously usable

#### Scenario: Desktop layout — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Tablet layout

The application SHALL provide a tablet layout that preserves full functionality while allowing navigation to collapse or overlay.

#### Scenario: Tablet layout — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the viewport enters the tablet range
- **THEN** the same domain actions remain accessible without desktop-only assumptions

#### Scenario: Tablet layout — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Mobile layout

The application SHALL provide a single-column mobile navigation model with touch-sized controls and full feature reachability.

#### Scenario: Mobile layout — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the viewport enters the mobile range
- **THEN** lists, editors, navigation, and creation workflows remain usable without horizontal overflow

#### Scenario: Mobile layout — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Desktop, tablet and mobile application shells with parity-oriented navigation** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Desktop layout`
- `Tablet layout`
- `Mobile layout`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Desktop layout` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Tablet layout` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Mobile layout` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Desktop layout`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Tablet layout`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Mobile layout`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-responsive-layout-foundation` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-responsive-layout-foundation --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-responsive-layout-foundation --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 05 — `add-persistence-foundation`

**Required completed outcome:** PostgreSQL model, Rust migrations, IDs, timestamps, tombstones/version fields

## Proposal blueprint

## Why

This change delivers **PostgreSQL model, Rust migrations, IDs, timestamps, tombstones/version fields** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Canonical persistence schema: The system SHALL persist accounts, entities, relationships/placements, recurrence data, reminders, preferences, sync metadata, and tombstones in PostgreSQL using Rust-managed migrations.
- Stable identifiers and versions: Persisted user entities SHALL use stable client-generatable identifiers and optimistic version metadata suitable for offline creation and synchronization.
- Migration safety: Every persistence change SHALL have a forward migration and an explicit rollback or documented irreversibility strategy tested against an isolated database.

## Dependencies

- `add-responsive-layout-foundation`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

PostgreSQL model, Rust migrations, IDs, timestamps, tombstones/version fields.

## ADDED Requirements

### Requirement: Canonical persistence schema

The system SHALL persist accounts, entities, relationships/placements, recurrence data, reminders, preferences, sync metadata, and tombstones in PostgreSQL using Rust-managed migrations.

#### Scenario: Canonical persistence schema — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a fresh database is migrated
- **THEN** the complete foundational schema is created deterministically

#### Scenario: Canonical persistence schema — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Stable identifiers and versions

Persisted user entities SHALL use stable client-generatable identifiers and optimistic version metadata suitable for offline creation and synchronization.

#### Scenario: Stable identifiers and versions — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a client creates an entity while offline
- **THEN** the identifier remains valid when later synchronized to the server

#### Scenario: Stable identifiers and versions — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Migration safety

Every persistence change SHALL have a forward migration and an explicit rollback or documented irreversibility strategy tested against an isolated database.

#### Scenario: Migration safety — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a migration suite runs
- **THEN** schema transitions complete without corrupting existing foundational data

#### Scenario: Migration safety — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **PostgreSQL model, Rust migrations, IDs, timestamps, tombstones/version fields** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Canonical persistence schema`
- `Stable identifiers and versions`
- `Migration safety`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Canonical persistence schema` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Stable identifiers and versions` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Migration safety` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Canonical persistence schema`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Stable identifiers and versions`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Migration safety`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-persistence-foundation` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-persistence-foundation --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-persistence-foundation --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 06 — `add-account-authentication`

**Required completed outcome:** Registration, verification, login/logout, sessions, reset, revocation and account deletion

## Proposal blueprint

## Why

This change delivers **Registration, verification, login/logout, sessions, reset, revocation and account deletion** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Account lifecycle: The system SHALL support registration, verification, login, logout, password reset/change, session revocation, and account deletion.
- Secure password and session handling: Passwords SHALL be stored with Argon2id and authenticated browser sessions SHALL use secure server-validated session state.
- Account isolation: All authenticated application queries and commands SHALL be scoped to the owning account.

## Dependencies

- `add-persistence-foundation`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Registration, verification, login/logout, sessions, reset, revocation and account deletion.

## ADDED Requirements

### Requirement: Account lifecycle

The system SHALL support registration, verification, login, logout, password reset/change, session revocation, and account deletion.

#### Scenario: Account lifecycle — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user follows a supported account lifecycle path
- **THEN** the account and session state transitions are enforced server-side

#### Scenario: Account lifecycle — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Secure password and session handling

Passwords SHALL be stored with Argon2id and authenticated browser sessions SHALL use secure server-validated session state.

#### Scenario: Secure password and session handling — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** valid credentials are submitted
- **THEN** a secure session is issued without exposing password material or reusable session secrets to client script

#### Scenario: Secure password and session handling — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Account isolation

All authenticated application queries and commands SHALL be scoped to the owning account.

#### Scenario: Account isolation — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** one account references another account's entity identifier
- **THEN** the operation is rejected without disclosing the foreign entity

#### Scenario: Account isolation — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Registration, verification, login/logout, sessions, reset, revocation and account deletion** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Account lifecycle`
- `Secure password and session handling`
- `Account isolation`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Account lifecycle` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Secure password and session handling` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Account isolation` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Account lifecycle`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Secure password and session handling`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Account isolation`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-account-authentication` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-account-authentication --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-account-authentication --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 07 — `add-task-domain-core`

**Required completed outcome:** Canonical Todo/Project/Area/Heading/Checklist/Tag types and domain invariants

## Proposal blueprint

## Why

This change delivers **Canonical Todo/Project/Area/Heading/Checklist/Tag types and domain invariants** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Canonical task model: The domain SHALL model Todo, Project, Area, Heading, ChecklistItem, Tag, schedule, deadline, reminder, and status concepts without coupling them to UI or database frameworks.
- Distinct temporal concepts: Start/schedule state, deadline, and reminder SHALL be represented as separate domain concepts.
- Project/context membership: A to-do SHALL have one canonical identity and a well-defined contextual parent while remaining eligible for derived list projections.

## Dependencies

- `add-account-authentication`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Canonical Todo/Project/Area/Heading/Checklist/Tag types and domain invariants.

## ADDED Requirements

### Requirement: Canonical task model

The domain SHALL model Todo, Project, Area, Heading, ChecklistItem, Tag, schedule, deadline, reminder, and status concepts without coupling them to UI or database frameworks.

#### Scenario: Canonical task model — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** domain tests construct and mutate task entities
- **THEN** all invariants can be evaluated in pure Rust

#### Scenario: Canonical task model — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Distinct temporal concepts

Start/schedule state, deadline, and reminder SHALL be represented as separate domain concepts.

#### Scenario: Distinct temporal concepts — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a to-do has both a start date and a deadline
- **THEN** changing one does not implicitly rewrite the other

#### Scenario: Distinct temporal concepts — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Project/context membership

A to-do SHALL have one canonical identity and a well-defined contextual parent while remaining eligible for derived list projections.

#### Scenario: Project/context membership — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the same to-do appears in a project and Today
- **THEN** both views resolve to the same canonical entity

#### Scenario: Project/context membership — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Canonical Todo/Project/Area/Heading/Checklist/Tag types and domain invariants** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Canonical task model`
- `Distinct temporal concepts`
- `Project/context membership`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Canonical task model` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Distinct temporal concepts` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Project/context membership` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Canonical task model`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Distinct temporal concepts`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Project/context membership`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-task-domain-core` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-task-domain-core --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-task-domain-core --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 08 — `add-fractional-ordering-engine`

**Required completed outcome:** Stable independent ordering across Inbox, project, Today, headings and sidebar scopes

## Proposal blueprint

## Why

This change delivers **Stable independent ordering across Inbox, project, Today, headings and sidebar scopes** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Independent ordering scopes: The system SHALL maintain independent manual ordering for contextual lists such as Inbox/project/heading and derived lists such as Today.
- Fractional insertion: Ordering SHALL support insertion between neighboring items without renumbering an entire list.
- Rank maintenance: The ordering engine SHALL detect exhausted rank space and rebalance a scope deterministically without changing visible order.

## Dependencies

- `add-task-domain-core`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Stable independent ordering across Inbox, project, Today, headings and sidebar scopes.

## ADDED Requirements

### Requirement: Independent ordering scopes

The system SHALL maintain independent manual ordering for contextual lists such as Inbox/project/heading and derived lists such as Today.

#### Scenario: Independent ordering scopes — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user reorders a project task inside Today
- **THEN** the project-local order is unchanged

#### Scenario: Independent ordering scopes — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Fractional insertion

Ordering SHALL support insertion between neighboring items without renumbering an entire list.

#### Scenario: Fractional insertion — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** an item is dropped between two adjacent items
- **THEN** a stable rank is assigned with bounded writes

#### Scenario: Fractional insertion — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Rank maintenance

The ordering engine SHALL detect exhausted rank space and rebalance a scope deterministically without changing visible order.

#### Scenario: Rank maintenance — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** rank density exceeds the configured threshold
- **THEN** the scope is rebalanced and all clients continue to observe the same order

#### Scenario: Rank maintenance — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Stable independent ordering across Inbox, project, Today, headings and sidebar scopes** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Independent ordering scopes`
- `Fractional insertion`
- `Rank maintenance`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Independent ordering scopes` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Fractional insertion` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Rank maintenance` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Independent ordering scopes`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Fractional insertion`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Rank maintenance`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-fractional-ordering-engine` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-fractional-ordering-engine --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-fractional-ordering-engine --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 09 — `add-command-transaction-layer`

**Required completed outcome:** Typed commands, validation, domain transactions, events, projection invalidation and sync hooks

## Proposal blueprint

## Why

This change delivers **Typed commands, validation, domain transactions, events, projection invalidation and sync hooks** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Typed application commands: All authoritative mutations SHALL enter through typed application commands rather than direct UI or persistence writes.
- Atomic command transaction: A command SHALL update domain state, persistence, change-log metadata, and projection invalidation atomically where persistence is involved.
- Deterministic command errors: Validation, authorization, conflict, and not-found failures SHALL be represented as typed application errors consumable by web, sync, and automation adapters.

## Dependencies

- `add-fractional-ordering-engine`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Typed commands, validation, domain transactions, events, projection invalidation and sync hooks.

## ADDED Requirements

### Requirement: Typed application commands

All authoritative mutations SHALL enter through typed application commands rather than direct UI or persistence writes.

#### Scenario: Typed application commands — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user edits or moves an entity
- **THEN** the mutation is validated and executed through the command layer

#### Scenario: Typed application commands — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Atomic command transaction

A command SHALL update domain state, persistence, change-log metadata, and projection invalidation atomically where persistence is involved.

#### Scenario: Atomic command transaction — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a persisted command succeeds
- **THEN** all related writes commit together

#### Scenario: Atomic command transaction — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Deterministic command errors

Validation, authorization, conflict, and not-found failures SHALL be represented as typed application errors consumable by web, sync, and automation adapters.

#### Scenario: Deterministic command errors — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a command cannot be applied
- **THEN** callers receive a stable error category without parsing infrastructure-specific strings

#### Scenario: Deterministic command errors — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Typed commands, validation, domain transactions, events, projection invalidation and sync hooks** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Typed application commands`
- `Atomic command transaction`
- `Deterministic command errors`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Typed application commands` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Atomic command transaction` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Deterministic command errors` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Typed application commands`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Atomic command transaction`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Deterministic command errors`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-command-transaction-layer` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-command-transaction-layer --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-command-transaction-layer --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 10 — `add-undo-redo-engine`

**Required completed outcome:** Reversible mutation model and grouped undo semantics

## Proposal blueprint

## Why

This change delivers **Reversible mutation model and grouped undo semantics** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Reversible mutation records: User-visible destructive or state-changing commands SHALL produce enough inverse information to support undo within the active history window.
- Grouped undo: Related multi-entity operations SHALL be undoable as a single group.
- Redo consistency: Undo and redo SHALL reuse domain commands/invariants rather than bypassing validation with raw database snapshots.

## Dependencies

- `add-command-transaction-layer`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Reversible mutation model and grouped undo semantics.

## ADDED Requirements

### Requirement: Reversible mutation records

User-visible destructive or state-changing commands SHALL produce enough inverse information to support undo within the active history window.

#### Scenario: Reversible mutation records — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a reversible command completes
- **THEN** an undo record is created without rereading lost state from external systems

#### Scenario: Reversible mutation records — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Grouped undo

Related multi-entity operations SHALL be undoable as a single group.

#### Scenario: Grouped undo — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a bulk move changes several to-dos
- **THEN** one undo action restores the complete prior state

#### Scenario: Grouped undo — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Redo consistency

Undo and redo SHALL reuse domain commands/invariants rather than bypassing validation with raw database snapshots.

#### Scenario: Redo consistency — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user redoes an undone action
- **THEN** the action passes through the same authoritative mutation rules

#### Scenario: Redo consistency — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Reversible mutation model and grouped undo semantics** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Reversible mutation records`
- `Grouped undo`
- `Redo consistency`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Reversible mutation records` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Grouped undo` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Redo consistency` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Reversible mutation records`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Grouped undo`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Redo consistency`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-undo-redo-engine` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-undo-redo-engine --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-undo-redo-engine --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 11 — `add-todo-crud`

**Required completed outcome:** Create, edit, duplicate, move, complete, cancel, reopen and delete lifecycle

## Proposal blueprint

## Why

This change delivers **Create, edit, duplicate, move, complete, cancel, reopen and delete lifecycle** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Todo lifecycle: Users SHALL be able to create, edit title/notes, duplicate, move, complete, cancel, reopen, and delete a to-do.
- Metadata preservation: Operations that do not semantically alter schedule, deadline, tags, checklist, or context SHALL preserve those fields.
- Optimistic concurrency: Todo updates SHALL detect stale versions instead of silently overwriting newer state.

## Dependencies

- `add-undo-redo-engine`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Create, edit, duplicate, move, complete, cancel, reopen and delete lifecycle.

## ADDED Requirements

### Requirement: Todo lifecycle

Users SHALL be able to create, edit title/notes, duplicate, move, complete, cancel, reopen, and delete a to-do.

#### Scenario: Todo lifecycle — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a supported lifecycle action is invoked
- **THEN** the canonical to-do transitions to the expected state

#### Scenario: Todo lifecycle — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Metadata preservation

Operations that do not semantically alter schedule, deadline, tags, checklist, or context SHALL preserve those fields.

#### Scenario: Metadata preservation — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user edits only the title
- **THEN** unrelated task metadata remains unchanged

#### Scenario: Metadata preservation — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Optimistic concurrency

Todo updates SHALL detect stale versions instead of silently overwriting newer state.

#### Scenario: Optimistic concurrency — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** two clients update the same version concurrently
- **THEN** the second conflicting write receives a resolvable conflict outcome

#### Scenario: Optimistic concurrency — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Create, edit, duplicate, move, complete, cancel, reopen and delete lifecycle** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Todo lifecycle`
- `Metadata preservation`
- `Optimistic concurrency`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Todo lifecycle` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Metadata preservation` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Optimistic concurrency` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Todo lifecycle`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Metadata preservation`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Optimistic concurrency`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-todo-crud` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-todo-crud --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-todo-crud --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 12 — `add-inbox`

**Required completed outcome:** Capture workflow, Inbox projection, badge, ordering, processing and movement

## Proposal blueprint

## Why

This change delivers **Capture workflow, Inbox projection, badge, ordering, processing and movement** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Inbox capture: A newly captured unfiled to-do SHALL appear in Inbox until it is assigned to another context or otherwise processed.
- Inbox ordering and badge: Inbox SHALL support manual ordering and an accurate open-item count/badge.
- Inbox processing: Inbox items SHALL support scheduling, tagging, assignment to project/area, completion, and deletion through the shared command system.

## Dependencies

- `add-todo-crud`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Capture workflow, Inbox projection, badge, ordering, processing and movement.

## ADDED Requirements

### Requirement: Inbox capture

A newly captured unfiled to-do SHALL appear in Inbox until it is assigned to another context or otherwise processed.

#### Scenario: Inbox capture — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user creates an unfiled task
- **THEN** the task is visible in Inbox immediately

#### Scenario: Inbox capture — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Inbox ordering and badge

Inbox SHALL support manual ordering and an accurate open-item count/badge.

#### Scenario: Inbox ordering and badge — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** items are inserted, moved, completed, or filed
- **THEN** the order and badge update consistently

#### Scenario: Inbox ordering and badge — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Inbox processing

Inbox items SHALL support scheduling, tagging, assignment to project/area, completion, and deletion through the shared command system.

#### Scenario: Inbox processing — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user processes an Inbox item
- **THEN** the item leaves or remains in Inbox according to its resulting context/state

#### Scenario: Inbox processing — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Capture workflow, Inbox projection, badge, ordering, processing and movement** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Inbox capture`
- `Inbox ordering and badge`
- `Inbox processing`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Inbox capture` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Inbox ordering and badge` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Inbox processing` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Inbox capture`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Inbox ordering and badge`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Inbox processing`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-inbox` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-inbox --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-inbox --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 13 — `add-areas`

**Required completed outcome:** Area creation, ordering, loose tasks, project grouping and tagging

## Proposal blueprint

## Why

This change delivers **Area creation, ordering, loose tasks, project grouping and tagging** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Area lifecycle: Users SHALL create, rename, reorder, tag, and delete Areas representing ongoing responsibilities.
- Area contents: An Area SHALL contain projects and loose to-dos without forcing loose items into synthetic projects.
- Area deletion policy: Deleting an Area SHALL require an explicit deterministic policy for contained projects/to-dos and SHALL never silently orphan or destroy them.

## Dependencies

- `add-inbox`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Area creation, ordering, loose tasks, project grouping and tagging.

## ADDED Requirements

### Requirement: Area lifecycle

Users SHALL create, rename, reorder, tag, and delete Areas representing ongoing responsibilities.

#### Scenario: Area lifecycle — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** an Area command succeeds
- **THEN** the sidebar/context model reflects the new area state

#### Scenario: Area lifecycle — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Area contents

An Area SHALL contain projects and loose to-dos without forcing loose items into synthetic projects.

#### Scenario: Area contents — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a to-do is moved directly into an Area
- **THEN** it appears under that Area while retaining canonical identity

#### Scenario: Area contents — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Area deletion policy

Deleting an Area SHALL require an explicit deterministic policy for contained projects/to-dos and SHALL never silently orphan or destroy them.

#### Scenario: Area deletion policy — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a non-empty Area is deleted
- **THEN** the configured move/delete confirmation path is applied atomically

#### Scenario: Area deletion policy — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Area creation, ordering, loose tasks, project grouping and tagging** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Area lifecycle`
- `Area contents`
- `Area deletion policy`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Area lifecycle` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Area contents` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Area deletion policy` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Area lifecycle`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Area contents`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Area deletion policy`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-areas` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-areas --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-areas --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 14 — `add-projects`

**Required completed outcome:** Project lifecycle, membership, notes, schedule, deadline, ordering and status

## Proposal blueprint

## Why

This change delivers **Project lifecycle, membership, notes, schedule, deadline, ordering and status** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Project lifecycle: Users SHALL create, edit, schedule, deadline, complete, cancel, reopen, move, and delete projects.
- Project child ordering: Projects SHALL own an ordered sequence of headings and/or direct to-dos while keeping each child's canonical identity.
- Project temporal participation: Projects SHALL participate in schedule/deadline/logbook projections without copying those values to every child.

## Dependencies

- `add-areas`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Project lifecycle, membership, notes, schedule, deadline, ordering and status.

## ADDED Requirements

### Requirement: Project lifecycle

Users SHALL create, edit, schedule, deadline, complete, cancel, reopen, move, and delete projects.

#### Scenario: Project lifecycle — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a supported project action is invoked
- **THEN** project state changes consistently with domain invariants

#### Scenario: Project lifecycle — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Project child ordering

Projects SHALL own an ordered sequence of headings and/or direct to-dos while keeping each child's canonical identity.

#### Scenario: Project child ordering — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** children are reordered
- **THEN** the project view reflects the new contextual order

#### Scenario: Project child ordering — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Project temporal participation

Projects SHALL participate in schedule/deadline/logbook projections without copying those values to every child.

#### Scenario: Project temporal participation — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a project is scheduled for a future date
- **THEN** the project changes projection membership while child schedules remain unchanged

#### Scenario: Project temporal participation — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Project lifecycle, membership, notes, schedule, deadline, ordering and status** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Project lifecycle`
- `Project child ordering`
- `Project temporal participation`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Project lifecycle` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Project child ordering` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Project temporal participation` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Project lifecycle`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Project child ordering`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Project temporal participation`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-projects` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-projects --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-projects --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 15 — `add-project-headings`

**Required completed outcome:** Create/edit/reorder/archive headings and move associated child tasks

## Proposal blueprint

## Why

This change delivers **Create/edit/reorder/archive headings and move associated child tasks** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Heading lifecycle: Project headings SHALL support create, rename, reorder, archive/restore, and conversion into a project where supported by the product model.
- Heading child movement: Moving a heading SHALL move the heading together with the to-dos assigned under it.
- Heading scope: Headings SHALL exist only within projects and SHALL not become independent global sidebar containers.

## Dependencies

- `add-projects`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Create/edit/reorder/archive headings and move associated child tasks.

## ADDED Requirements

### Requirement: Heading lifecycle

Project headings SHALL support create, rename, reorder, archive/restore, and conversion into a project where supported by the product model.

#### Scenario: Heading lifecycle — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a heading command is executed
- **THEN** the containing project preserves deterministic child structure

#### Scenario: Heading lifecycle — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Heading child movement

Moving a heading SHALL move the heading together with the to-dos assigned under it.

#### Scenario: Heading child movement — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a heading is reordered or transferred within project structure
- **THEN** its assigned child group moves as one unit

#### Scenario: Heading child movement — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Heading scope

Headings SHALL exist only within projects and SHALL not become independent global sidebar containers.

#### Scenario: Heading scope — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a heading is referenced outside its project context
- **THEN** the domain rejects invalid standalone placement

#### Scenario: Heading scope — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Create/edit/reorder/archive headings and move associated child tasks** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Heading lifecycle`
- `Heading child movement`
- `Heading scope`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Heading lifecycle` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Heading child movement` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Heading scope` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Heading lifecycle`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Heading child movement`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Heading scope`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-project-headings` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-project-headings --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-project-headings --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 16 — `add-checklists`

**Required completed outcome:** Ordered embedded checklist editing, completion, paste expansion and reordering

## Proposal blueprint

## Why

This change delivers **Ordered embedded checklist editing, completion, paste expansion and reordering** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Checklist editing: A to-do SHALL support an ordered checklist whose items can be added, edited, completed, uncompleted, reordered, and removed.
- Bulk paste expansion: Pasting multi-line checklist content SHALL create one checklist item per non-empty parsed line using deterministic normalization.
- Template/copy semantics: Checklist state SHALL be compatible with recurrence templates so future copies can inherit structure without inheriting completion state improperly.

## Dependencies

- `add-project-headings`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Ordered embedded checklist editing, completion, paste expansion and reordering.

## ADDED Requirements

### Requirement: Checklist editing

A to-do SHALL support an ordered checklist whose items can be added, edited, completed, uncompleted, reordered, and removed.

#### Scenario: Checklist editing — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a checklist item is changed
- **THEN** the parent to-do remains the same canonical entity and checklist order/state updates

#### Scenario: Checklist editing — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Bulk paste expansion

Pasting multi-line checklist content SHALL create one checklist item per non-empty parsed line using deterministic normalization.

#### Scenario: Bulk paste expansion — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** multiple lines are pasted into checklist creation
- **THEN** the expected ordered checklist items are produced

#### Scenario: Bulk paste expansion — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Template/copy semantics

Checklist state SHALL be compatible with recurrence templates so future copies can inherit structure without inheriting completion state improperly.

#### Scenario: Template/copy semantics — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a recurring item copy is materialized
- **THEN** it receives checklist content with copy-appropriate completion state

#### Scenario: Template/copy semantics — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Ordered embedded checklist editing, completion, paste expansion and reordering** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Checklist editing`
- `Bulk paste expansion`
- `Template/copy semantics`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Checklist editing` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Bulk paste expansion` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Template/copy semantics` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Checklist editing`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Bulk paste expansion`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Template/copy semantics`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-checklists` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-checklists --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-checklists --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 17 — `add-tags`

**Required completed outcome:** Tag CRUD, hierarchy, assignment, inherited context and multi-tag filtering

## Proposal blueprint

## Why

This change delivers **Tag CRUD, hierarchy, assignment, inherited context and multi-tag filtering** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Tag hierarchy: Users SHALL create, rename, delete, reorder, and nest tags.
- Tag assignment and filtering: Open entities SHALL support tag assignment/removal and list filtering by selected tags.
- Inherited context tags: Area/project tags SHALL be available as inherited filtering context without duplicating all inherited relationships onto every child entity.

## Dependencies

- `add-checklists`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Tag CRUD, hierarchy, assignment, inherited context and multi-tag filtering.

## ADDED Requirements

### Requirement: Tag hierarchy

Users SHALL create, rename, delete, reorder, and nest tags.

#### Scenario: Tag hierarchy — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a nested tag hierarchy is edited
- **THEN** cycles are prevented and deterministic hierarchy is preserved

#### Scenario: Tag hierarchy — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Tag assignment and filtering

Open entities SHALL support tag assignment/removal and list filtering by selected tags.

#### Scenario: Tag assignment and filtering — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user filters a list by tags
- **THEN** only items satisfying the defined filter semantics are shown without changing entity membership

#### Scenario: Tag assignment and filtering — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Inherited context tags

Area/project tags SHALL be available as inherited filtering context without duplicating all inherited relationships onto every child entity.

#### Scenario: Inherited context tags — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a child item is evaluated for a parent tag filter
- **THEN** the inherited tag contributes to matching without synthetic direct assignment

#### Scenario: Inherited context tags — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Tag CRUD, hierarchy, assignment, inherited context and multi-tag filtering** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Tag hierarchy`
- `Tag assignment and filtering`
- `Inherited context tags`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Tag hierarchy` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Tag assignment and filtering` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Inherited context tags` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Tag hierarchy`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Tag assignment and filtering`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Inherited context tags`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-tags` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-tags --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-tags --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 18 — `add-notes-editor`

**Required completed outcome:** Multiline notes, links, selection, find/replace and editing behavior

## Proposal blueprint

## Why

This change delivers **Multiline notes, links, selection, find/replace and editing behavior** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Rich plain-text notes editor: A to-do/project note SHALL support multiline editing, selection, clipboard operations, undo/redo, and link interaction.
- Find and replace: The note editor SHALL support in-note find, next/previous, and replace operations for long notes.
- Safe link handling: Detected links SHALL open through safe browser navigation rules and SHALL not execute untrusted script URLs.

## Dependencies

- `add-tags`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Multiline notes, links, selection, find/replace and editing behavior.

## ADDED Requirements

### Requirement: Rich plain-text notes editor

A to-do/project note SHALL support multiline editing, selection, clipboard operations, undo/redo, and link interaction.

#### Scenario: Rich plain-text notes editor — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user edits a note
- **THEN** text changes persist without corrupting surrounding task metadata

#### Scenario: Rich plain-text notes editor — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Find and replace

The note editor SHALL support in-note find, next/previous, and replace operations for long notes.

#### Scenario: Find and replace — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a search term exists in the open note
- **THEN** matching ranges can be traversed and replaced deterministically

#### Scenario: Find and replace — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Safe link handling

Detected links SHALL open through safe browser navigation rules and SHALL not execute untrusted script URLs.

#### Scenario: Safe link handling — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a note contains a link
- **THEN** supported URLs are navigable and unsafe schemes are rejected

#### Scenario: Safe link handling — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Multiline notes, links, selection, find/replace and editing behavior** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Rich plain-text notes editor`
- `Find and replace`
- `Safe link handling`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Rich plain-text notes editor` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Find and replace` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Safe link handling` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Rich plain-text notes editor`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Find and replace`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Safe link handling`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-notes-editor` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-notes-editor --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-notes-editor --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 19 — `add-things-markdown`

**Required completed outcome:** Things-style Markdown subset with syntax-visible rich rendering

## Proposal blueprint

## Why

This change delivers **Things-style Markdown subset with syntax-visible rich rendering** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Supported Markdown subset: Notes SHALL render the selected Things-style Markdown subset including emphasis, strong text, highlight, headings, lists, quotes, code, and links.
- Syntax-visible editing: Markdown punctuation SHALL remain part of the editable text while formatting is visually represented.
- Lossless storage: Markdown rendering SHALL be derived from stored source text rather than storing a destructive rich-text representation.

## Dependencies

- `add-notes-editor`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Things-style Markdown subset with syntax-visible rich rendering.

## ADDED Requirements

### Requirement: Supported Markdown subset

Notes SHALL render the selected Things-style Markdown subset including emphasis, strong text, highlight, headings, lists, quotes, code, and links.

#### Scenario: Supported Markdown subset — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a note contains supported syntax
- **THEN** the corresponding visual formatting is rendered

#### Scenario: Supported Markdown subset — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Syntax-visible editing

Markdown punctuation SHALL remain part of the editable text while formatting is visually represented.

#### Scenario: Syntax-visible editing — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a formatted note is edited
- **THEN** the raw Markdown syntax remains directly editable

#### Scenario: Syntax-visible editing — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Lossless storage

Markdown rendering SHALL be derived from stored source text rather than storing a destructive rich-text representation.

#### Scenario: Lossless storage — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the same note is loaded after save
- **THEN** the original Markdown source is preserved

#### Scenario: Lossless storage — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Things-style Markdown subset with syntax-visible rich rendering** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Supported Markdown subset`
- `Syntax-visible editing`
- `Lossless storage`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Supported Markdown subset` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Syntax-visible editing` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Lossless storage` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Supported Markdown subset`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Syntax-visible editing`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Lossless storage`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-things-markdown` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-things-markdown --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-things-markdown --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 20 — `add-scheduling-model`

**Required completed outcome:** Anytime/Someday/on-date/Today/This Evening scheduling semantics

## Proposal blueprint

## Why

This change delivers **Anytime/Someday/on-date/Today/This Evening scheduling semantics** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Schedule states: A to-do/project SHALL support Anytime, Someday, Today/date-based scheduling, and This Evening as a Today presentation state.
- Schedule transitions: Moving among Anytime, Someday, Today, evening, and future dates SHALL preserve unrelated context and metadata.
- Local-date semantics: Schedule dates SHALL be stored and compared as user-local dates rather than UTC midnight timestamps.

## Dependencies

- `add-things-markdown`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Anytime/Someday/on-date/Today/This Evening scheduling semantics.

## ADDED Requirements

### Requirement: Schedule states

A to-do/project SHALL support Anytime, Someday, Today/date-based scheduling, and This Evening as a Today presentation state.

#### Scenario: Schedule states — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a schedule command is applied
- **THEN** projection membership changes according to schedule semantics

#### Scenario: Schedule states — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Schedule transitions

Moving among Anytime, Someday, Today, evening, and future dates SHALL preserve unrelated context and metadata.

#### Scenario: Schedule transitions — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** an item is rescheduled
- **THEN** parent project/area, tags, notes, and deadline are preserved

#### Scenario: Schedule transitions — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Local-date semantics

Schedule dates SHALL be stored and compared as user-local dates rather than UTC midnight timestamps.

#### Scenario: Local-date semantics — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the user's timezone crosses a UTC date boundary
- **THEN** the intended calendar date remains unchanged

#### Scenario: Local-date semantics — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Anytime/Someday/on-date/Today/This Evening scheduling semantics** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Schedule states`
- `Schedule transitions`
- `Local-date semantics`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Schedule states` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Schedule transitions` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Local-date semantics` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Schedule states`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Schedule transitions`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Local-date semantics`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-scheduling-model` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-scheduling-model --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-scheduling-model --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 21 — `add-deadlines`

**Required completed outcome:** Independent deadline model, overdue/due-today/future states and sorting

## Proposal blueprint

## Why

This change delivers **Independent deadline model, overdue/due-today/future states and sorting** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Independent deadline: A deadline SHALL be independent from start/schedule date and reminder.
- Deadline states: The system SHALL derive future, due-today, and overdue states from the user's local date.
- Deadline projections: Deadline metadata SHALL be queryable for deadline-specific views and visible in other applicable views without duplicating the entity.

## Dependencies

- `add-scheduling-model`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Independent deadline model, overdue/due-today/future states and sorting.

## ADDED Requirements

### Requirement: Independent deadline

A deadline SHALL be independent from start/schedule date and reminder.

#### Scenario: Independent deadline — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a deadline is added or cleared
- **THEN** the start schedule is unchanged

#### Scenario: Independent deadline — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Deadline states

The system SHALL derive future, due-today, and overdue states from the user's local date.

#### Scenario: Deadline states — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the local date advances past an open item's deadline
- **THEN** the item becomes overdue without mutating the stored deadline

#### Scenario: Deadline states — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Deadline projections

Deadline metadata SHALL be queryable for deadline-specific views and visible in other applicable views without duplicating the entity.

#### Scenario: Deadline projections — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** an item with a deadline appears in multiple projections
- **THEN** all representations reference the same deadline value

#### Scenario: Deadline projections — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Independent deadline model, overdue/due-today/future states and sorting** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Independent deadline`
- `Deadline states`
- `Deadline projections`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Independent deadline` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Deadline states` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Deadline projections` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Independent deadline`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Deadline states`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Deadline projections`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-deadlines` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-deadlines --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-deadlines --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 22 — `add-today-projection`

**Required completed outcome:** Derived Today view, This Evening, manual ordering and correct membership

## Proposal blueprint

## Why

This change delivers **Derived Today view, This Evening, manual ordering and correct membership** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Today membership: Today SHALL be a derived projection over eligible open items scheduled for today or otherwise required by documented Today semantics.
- This Evening section: Today SHALL support a distinct This Evening section with independent presentation ordering.
- Today manual ordering: Users SHALL be able to reorder Today independently from project/area ordering.

## Dependencies

- `add-scheduling-model`
- `add-fractional-ordering-engine`
- `add-todo-crud`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Derived Today view, This Evening, manual ordering and correct membership.

## ADDED Requirements

### Requirement: Today membership

Today SHALL be a derived projection over eligible open items scheduled for today or otherwise required by documented Today semantics.

#### Scenario: Today membership — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the local day begins or an item is scheduled for today
- **THEN** the item is included without being moved out of its project/context

#### Scenario: Today membership — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: This Evening section

Today SHALL support a distinct This Evening section with independent presentation ordering.

#### Scenario: This Evening section — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a Today item is moved to This Evening
- **THEN** it remains a Today item while rendering in the evening section

#### Scenario: This Evening section — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Today manual ordering

Users SHALL be able to reorder Today independently from project/area ordering.

#### Scenario: Today manual ordering — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a Today item is dragged
- **THEN** Today rank changes while contextual rank remains stable

#### Scenario: Today manual ordering — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Derived Today view, This Evening, manual ordering and correct membership** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Today membership`
- `This Evening section`
- `Today manual ordering`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Today membership` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `This Evening section` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Today manual ordering` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Today membership`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `This Evening section`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Today manual ordering`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-today-projection` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-today-projection --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-today-projection --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 23 — `add-upcoming-projection`

**Required completed outcome:** Date-grouped future agenda including rescheduling behavior

## Proposal blueprint

## Why

This change delivers **Date-grouped future agenda including rescheduling behavior** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Date-grouped Upcoming: Upcoming SHALL group future scheduled items by local calendar date and include future recurrence/deadline/calendar projections as defined by their capabilities.
- Drag-to-reschedule: Moving an eligible item from one Upcoming date to another SHALL change its schedule date through the command layer.
- Projection-only calendar rows: External calendar events displayed in Upcoming SHALL remain read-only unless an integration explicitly supports editing.

## Dependencies

- `add-scheduling-model`
- `add-deadlines`
- `add-recurrence-model (later integration may extend this capability)`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Date-grouped future agenda including rescheduling behavior.

## ADDED Requirements

### Requirement: Date-grouped Upcoming

Upcoming SHALL group future scheduled items by local calendar date and include future recurrence/deadline/calendar projections as defined by their capabilities.

#### Scenario: Date-grouped Upcoming — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a future item exists
- **THEN** it appears under the correct future date

#### Scenario: Date-grouped Upcoming — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Drag-to-reschedule

Moving an eligible item from one Upcoming date to another SHALL change its schedule date through the command layer.

#### Scenario: Drag-to-reschedule — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a task is dropped on another date
- **THEN** the schedule is updated and the item moves to that day

#### Scenario: Drag-to-reschedule — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Projection-only calendar rows

External calendar events displayed in Upcoming SHALL remain read-only unless an integration explicitly supports editing.

#### Scenario: Projection-only calendar rows — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user attempts a task-only mutation on an external calendar event
- **THEN** the action is unavailable or rejected

#### Scenario: Projection-only calendar rows — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Date-grouped future agenda including rescheduling behavior** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Date-grouped Upcoming`
- `Drag-to-reschedule`
- `Projection-only calendar rows`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Date-grouped Upcoming` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Drag-to-reschedule` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Projection-only calendar rows` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Date-grouped Upcoming`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Drag-to-reschedule`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Projection-only calendar rows`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-upcoming-projection` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-upcoming-projection --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-upcoming-projection --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 24 — `add-anytime-projection`

**Required completed outcome:** Correct currently-actionable projection and exclusions

## Proposal blueprint

## Why

This change delivers **Correct currently-actionable projection and exclusions** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Actionable membership: Anytime SHALL include currently actionable open items that are not Someday and are not blocked by a future start date.
- Exclusions: Future-scheduled, Someday, completed, canceled, deleted, and otherwise inactive items SHALL be excluded from Anytime.
- Context grouping: Anytime MAY group by areas/projects but SHALL not create new parent relationships as a side effect of projection rendering.

## Dependencies

- `add-upcoming-projection`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Correct currently-actionable projection and exclusions.

## ADDED Requirements

### Requirement: Actionable membership

Anytime SHALL include currently actionable open items that are not Someday and are not blocked by a future start date.

#### Scenario: Actionable membership — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** an open item has no future block
- **THEN** it is eligible for Anytime

#### Scenario: Actionable membership — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Exclusions

Future-scheduled, Someday, completed, canceled, deleted, and otherwise inactive items SHALL be excluded from Anytime.

#### Scenario: Exclusions — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** an item's state becomes inactive
- **THEN** it disappears from Anytime without losing context

#### Scenario: Exclusions — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Context grouping

Anytime MAY group by areas/projects but SHALL not create new parent relationships as a side effect of projection rendering.

#### Scenario: Context grouping — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** Anytime is rendered
- **THEN** grouping reflects existing context only

#### Scenario: Context grouping — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Correct currently-actionable projection and exclusions** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Actionable membership`
- `Exclusions`
- `Context grouping`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Actionable membership` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Exclusions` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Context grouping` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Actionable membership`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Exclusions`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Context grouping`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-anytime-projection` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-anytime-projection --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-anytime-projection --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 25 — `add-someday-projection`

**Required completed outcome:** Deferred/inactive tasks and projects with correct transitions

## Proposal blueprint

## Why

This change delivers **Deferred/inactive tasks and projects with correct transitions** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Someday membership: Someday SHALL contain open items/projects intentionally deferred into the Someday schedule state.
- Someday reactivation: A Someday item SHALL become actionable only through an explicit schedule/context action that changes its schedule state.
- Parent/child independence: A parent being active SHALL not implicitly reactivate a child explicitly scheduled Someday.

## Dependencies

- `add-anytime-projection`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Deferred/inactive tasks and projects with correct transitions.

## ADDED Requirements

### Requirement: Someday membership

Someday SHALL contain open items/projects intentionally deferred into the Someday schedule state.

#### Scenario: Someday membership — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** an item is deferred to Someday
- **THEN** it leaves actionable date views and appears in Someday

#### Scenario: Someday membership — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Someday reactivation

A Someday item SHALL become actionable only through an explicit schedule/context action that changes its schedule state.

#### Scenario: Someday reactivation — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a Someday item is assigned Today/Anytime/future date
- **THEN** projection membership updates accordingly

#### Scenario: Someday reactivation — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Parent/child independence

A parent being active SHALL not implicitly reactivate a child explicitly scheduled Someday.

#### Scenario: Parent/child independence — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** an active project contains a Someday to-do
- **THEN** the child remains excluded from actionable projections

#### Scenario: Parent/child independence — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Deferred/inactive tasks and projects with correct transitions** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Someday membership`
- `Someday reactivation`
- `Parent/child independence`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Someday membership` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Someday reactivation` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Parent/child independence` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Someday membership`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Someday reactivation`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Parent/child independence`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-someday-projection` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-someday-projection --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-someday-projection --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 26 — `add-completion-and-cancellation`

**Required completed outcome:** Distinct completed/canceled semantics, project handling and reopening

## Proposal blueprint

## Why

This change delivers **Distinct completed/canceled semantics, project handling and reopening** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Completed versus canceled: The domain SHALL distinguish completed and canceled terminal states.
- Reopen: Completed or canceled items SHALL support reopening when allowed, restoring them to an open state without inventing unrelated dates.
- Project completion safety: Completing/canceling a project with open children SHALL follow an explicit tested policy rather than silently losing child work.

## Dependencies

- `add-someday-projection`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Distinct completed/canceled semantics, project handling and reopening.

## ADDED Requirements

### Requirement: Completed versus canceled

The domain SHALL distinguish completed and canceled terminal states.

#### Scenario: Completed versus canceled — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user completes or cancels an item
- **THEN** the corresponding state and timestamp are recorded distinctly

#### Scenario: Completed versus canceled — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Reopen

Completed or canceled items SHALL support reopening when allowed, restoring them to an open state without inventing unrelated dates.

#### Scenario: Reopen — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user reopens an item
- **THEN** terminal metadata is cleared according to policy and context is preserved

#### Scenario: Reopen — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Project completion safety

Completing/canceling a project with open children SHALL follow an explicit tested policy rather than silently losing child work.

#### Scenario: Project completion safety — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a terminal action is requested on a non-empty active project
- **THEN** the defined confirmation/child transition behavior occurs atomically

#### Scenario: Project completion safety — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Distinct completed/canceled semantics, project handling and reopening** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Completed versus canceled`
- `Reopen`
- `Project completion safety`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Completed versus canceled` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Reopen` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Project completion safety` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Completed versus canceled`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Reopen`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Project completion safety`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-completion-and-cancellation` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-completion-and-cancellation --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-completion-and-cancellation --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 27 — `add-logbook`

**Required completed outcome:** Completion history, date grouping, canceled items, projects and logging behavior

## Proposal blueprint

## Why

This change delivers **Completion history, date grouping, canceled items, projects and logging behavior** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Logbook membership: Logbook SHALL derive completed/canceled entities from terminal state and logging policy.
- History timestamps: Creation, completion, cancellation, and logging timestamps SHALL remain queryable after logging.
- Logging policy: Automatic versus manual logging behavior SHALL be controlled by preference while preserving completed state independently.

## Dependencies

- `add-completion-and-cancellation`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Completion history, date grouping, canceled items, projects and logging behavior.

## ADDED Requirements

### Requirement: Logbook membership

Logbook SHALL derive completed/canceled entities from terminal state and logging policy.

#### Scenario: Logbook membership — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** an eligible item is logged
- **THEN** it appears in Logbook grouped according to completion/cancellation date

#### Scenario: Logbook membership — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: History timestamps

Creation, completion, cancellation, and logging timestamps SHALL remain queryable after logging.

#### Scenario: History timestamps — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a logged item is inspected
- **THEN** historical timestamps are available without reconstructing them from display order

#### Scenario: History timestamps — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Logging policy

Automatic versus manual logging behavior SHALL be controlled by preference while preserving completed state independently.

#### Scenario: Logging policy — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the logging preference changes
- **THEN** completed items move to Logbook according to policy without changing completion timestamps

#### Scenario: Logging policy — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Completion history, date grouping, canceled items, projects and logging behavior** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Logbook membership`
- `History timestamps`
- `Logging policy`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Logbook membership` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `History timestamps` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Logging policy` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Logbook membership`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `History timestamps`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Logging policy`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-logbook` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-logbook --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-logbook --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 28 — `add-trash`

**Required completed outcome:** Recoverable deletion, restore, permanent deletion and empty-trash flow

## Proposal blueprint

## Why

This change delivers **Recoverable deletion, restore, permanent deletion and empty-trash flow** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Soft deletion: Deleting a supported entity SHALL move it into recoverable Trash via tombstone/deleted state rather than immediate physical loss.
- Restore: Trash items SHALL be restorable with deterministic recovery of their prior or valid fallback context.
- Permanent deletion: Permanent deletion/empty Trash SHALL require an explicit action and remove or anonymize data according to retention policy.

## Dependencies

- `add-logbook`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Recoverable deletion, restore, permanent deletion and empty-trash flow.

## ADDED Requirements

### Requirement: Soft deletion

Deleting a supported entity SHALL move it into recoverable Trash via tombstone/deleted state rather than immediate physical loss.

#### Scenario: Soft deletion — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user deletes an eligible entity
- **THEN** it disappears from active projections and appears in Trash

#### Scenario: Soft deletion — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Restore

Trash items SHALL be restorable with deterministic recovery of their prior or valid fallback context.

#### Scenario: Restore — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a deleted entity is restored
- **THEN** it becomes active in a valid context with its user data preserved

#### Scenario: Restore — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Permanent deletion

Permanent deletion/empty Trash SHALL require an explicit action and remove or anonymize data according to retention policy.

#### Scenario: Permanent deletion — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user confirms permanent deletion
- **THEN** the data is irrecoverably removed from active storage and sync tombstones remain only as long as required

#### Scenario: Permanent deletion — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Recoverable deletion, restore, permanent deletion and empty-trash flow** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Soft deletion`
- `Restore`
- `Permanent deletion`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Soft deletion` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Restore` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Permanent deletion` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Soft deletion`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Restore`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Permanent deletion`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-trash` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-trash --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-trash --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 29 — `add-special-lists`

**Required completed outcome:** Tomorrow, Deadlines, Repeating, All Projects and Logged Projects

## Proposal blueprint

## Why

This change delivers **Tomorrow, Deadlines, Repeating, All Projects and Logged Projects** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Tomorrow view: The system SHALL expose a Tomorrow special view for items scheduled for the next local calendar date.
- Deadlines and Repeating views: The system SHALL expose special views for open deadline-bearing items and recurrence templates.
- Project special views: All Projects and Logged Projects SHALL expose open and logged/completed project sets respectively.

## Dependencies

- `add-trash`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Tomorrow, Deadlines, Repeating, All Projects and Logged Projects.

## ADDED Requirements

### Requirement: Tomorrow view

The system SHALL expose a Tomorrow special view for items scheduled for the next local calendar date.

#### Scenario: Tomorrow view — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** Tomorrow is opened
- **THEN** eligible items for the next day are shown

#### Scenario: Tomorrow view — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Deadlines and Repeating views

The system SHALL expose special views for open deadline-bearing items and recurrence templates.

#### Scenario: Deadlines and Repeating views — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** those special destinations are opened
- **THEN** items are ordered/grouped according to their capability semantics

#### Scenario: Deadlines and Repeating views — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Project special views

All Projects and Logged Projects SHALL expose open and logged/completed project sets respectively.

#### Scenario: Project special views — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user opens either project special view
- **THEN** the correct project population appears without adding permanent sidebar membership

#### Scenario: Project special views — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Tomorrow, Deadlines, Repeating, All Projects and Logged Projects** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Tomorrow view`
- `Deadlines and Repeating views`
- `Project special views`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Tomorrow view` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Deadlines and Repeating views` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Project special views` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Tomorrow view`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Deadlines and Repeating views`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Project special views`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-special-lists` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-special-lists --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-special-lists --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 30 — `add-project-progress`

**Required completed outcome:** Derived project progress rings and Today completion progress

## Proposal blueprint

## Why

This change delivers **Derived project progress rings and Today completion progress** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Project progress: Project progress SHALL be derived from eligible child completion rather than stored as an independently editable percentage.
- Today progress: Today SHALL expose a completion progress indicator based on the defined set of completable Today items.
- Edge-case policy: Zero-item, canceled-item, and non-completable projected rows SHALL have explicit inclusion/exclusion rules for progress calculations.

## Dependencies

- `add-projects`
- `add-completion-and-cancellation`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Derived project progress rings and Today completion progress.

## ADDED Requirements

### Requirement: Project progress

Project progress SHALL be derived from eligible child completion rather than stored as an independently editable percentage.

#### Scenario: Project progress — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a child task completion changes
- **THEN** the project progress indicator recalculates

#### Scenario: Project progress — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Today progress

Today SHALL expose a completion progress indicator based on the defined set of completable Today items.

#### Scenario: Today progress — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a Today item is completed or reopened
- **THEN** the Today progress indicator updates

#### Scenario: Today progress — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Edge-case policy

Zero-item, canceled-item, and non-completable projected rows SHALL have explicit inclusion/exclusion rules for progress calculations.

#### Scenario: Edge-case policy — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a project/list has only excluded rows
- **THEN** the progress UI renders the defined neutral state

#### Scenario: Edge-case policy — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Derived project progress rings and Today completion progress** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Project progress`
- `Today progress`
- `Edge-case policy`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Project progress` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Today progress` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Edge-case policy` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Project progress`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Today progress`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Edge-case policy`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-project-progress` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-project-progress --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-project-progress --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 31 — `add-move-engine`

**Required completed outcome:** One authoritative movement engine for tasks/projects/headings and destinations

## Proposal blueprint

## Why

This change delivers **One authoritative movement engine for tasks/projects/headings and destinations** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Authoritative destination validation: The move engine SHALL define valid destinations for to-dos, projects, and headings in one domain/application service.
- Move preservation: Moving an entity SHALL preserve data unrelated to membership/order/schedule unless the destination semantics require a change.
- Adapter reuse: Drag-and-drop, Move dialog, keyboard commands, and automation API SHALL call the same move operation.

## Dependencies

- `add-project-progress`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

One authoritative movement engine for tasks/projects/headings and destinations.

## ADDED Requirements

### Requirement: Authoritative destination validation

The move engine SHALL define valid destinations for to-dos, projects, and headings in one domain/application service.

#### Scenario: Authoritative destination validation — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** any adapter requests a move
- **THEN** destination validation uses the same authoritative rules

#### Scenario: Authoritative destination validation — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Move preservation

Moving an entity SHALL preserve data unrelated to membership/order/schedule unless the destination semantics require a change.

#### Scenario: Move preservation — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a to-do moves between projects
- **THEN** title, notes, tags, deadline, checklist, and identity remain unchanged

#### Scenario: Move preservation — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Adapter reuse

Drag-and-drop, Move dialog, keyboard commands, and automation API SHALL call the same move operation.

#### Scenario: Adapter reuse — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the same move is initiated through different UI/API surfaces
- **THEN** the resulting domain state is equivalent

#### Scenario: Adapter reuse — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **One authoritative movement engine for tasks/projects/headings and destinations** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Authoritative destination validation`
- `Move preservation`
- `Adapter reuse`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Authoritative destination validation` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Move preservation` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Adapter reuse` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Authoritative destination validation`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Move preservation`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Adapter reuse`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-move-engine` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-move-engine --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-move-engine --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 32 — `add-drag-and-drop`

**Required completed outcome:** Reorder/move/date-reschedule behavior across desktop and touch input

## Proposal blueprint

## Why

This change delivers **Reorder/move/date-reschedule behavior across desktop and touch input** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Pointer drag reorder: Users SHALL reorder supported lists with pointer/touch drag while receiving stable insertion feedback.
- Cross-context move: Dragging to a valid project/area/sidebar destination SHALL invoke the move engine rather than manipulating UI-only state.
- Upcoming reschedule drag: Dragging an eligible task between Upcoming date sections SHALL schedule it to the target date.

## Dependencies

- `add-move-engine`
- `add-fractional-ordering-engine`
- `add-multi-selection (integration may be completed in 33)`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Reorder/move/date-reschedule behavior across desktop and touch input.

## ADDED Requirements

### Requirement: Pointer drag reorder

Users SHALL reorder supported lists with pointer/touch drag while receiving stable insertion feedback.

#### Scenario: Pointer drag reorder — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** an item is dragged within a valid ordering scope
- **THEN** its rank updates to the indicated insertion point

#### Scenario: Pointer drag reorder — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Cross-context move

Dragging to a valid project/area/sidebar destination SHALL invoke the move engine rather than manipulating UI-only state.

#### Scenario: Cross-context move — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** an item is dropped on a valid destination
- **THEN** canonical context changes atomically

#### Scenario: Cross-context move — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Upcoming reschedule drag

Dragging an eligible task between Upcoming date sections SHALL schedule it to the target date.

#### Scenario: Upcoming reschedule drag — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** an item is dropped on a different day
- **THEN** the schedule command runs and contextual membership is preserved

#### Scenario: Upcoming reschedule drag — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Reorder/move/date-reschedule behavior across desktop and touch input** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Pointer drag reorder`
- `Cross-context move`
- `Upcoming reschedule drag`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Pointer drag reorder` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Cross-context move` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Upcoming reschedule drag` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Pointer drag reorder`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Cross-context move`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Upcoming reschedule drag`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-drag-and-drop` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-drag-and-drop --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-drag-and-drop --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 33 — `add-multi-selection`

**Required completed outcome:** Range/toggle/select-all state plus bulk commands and multi-item movement

## Proposal blueprint

## Why

This change delivers **Range/toggle/select-all state plus bulk commands and multi-item movement** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Selection state machine: The UI SHALL support single, toggle, range, select-all, and clear selection semantics over selectable list rows.
- Bulk commands: Supported commands such as complete, cancel, schedule, deadline, move, tag, and delete SHALL operate on the selected set transactionally or with explicit per-item results.
- Selection stability: Selection SHALL survive non-destructive list rerenders where selected entities remain visible and valid.

## Dependencies

- `add-drag-and-drop`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Range/toggle/select-all state plus bulk commands and multi-item movement.

## ADDED Requirements

### Requirement: Selection state machine

The UI SHALL support single, toggle, range, select-all, and clear selection semantics over selectable list rows.

#### Scenario: Selection state machine — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user extends or toggles selection
- **THEN** the selected ID set and focus/anchor behave deterministically

#### Scenario: Selection state machine — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Bulk commands

Supported commands such as complete, cancel, schedule, deadline, move, tag, and delete SHALL operate on the selected set transactionally or with explicit per-item results.

#### Scenario: Bulk commands — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a bulk command runs
- **THEN** all eligible selected items are processed and failures are reported deterministically

#### Scenario: Bulk commands — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Selection stability

Selection SHALL survive non-destructive list rerenders where selected entities remain visible and valid.

#### Scenario: Selection stability — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** projection data refreshes
- **THEN** still-valid selected items remain selected

#### Scenario: Selection stability — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Range/toggle/select-all state plus bulk commands and multi-item movement** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Selection state machine`
- `Bulk commands`
- `Selection stability`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Selection state machine` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Bulk commands` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Selection stability` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Selection state machine`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Bulk commands`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Selection stability`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-multi-selection` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-multi-selection --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-multi-selection --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 34 — `add-magic-plus`

**Required completed outcome:** Context-aware Things-style creation control and insertion positioning

## Proposal blueprint

## Why

This change delivers **Context-aware Things-style creation control and insertion positioning** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Contextual creation: Magic Plus SHALL create a to-do in the currently visible logical context by default.
- Drag insertion: Dragging the add control between rows SHALL create at the indicated rank/location.
- Alternate creation targets: Where supported, dragging the creation control toward heading/Inbox affordances SHALL create the appropriate entity/context through normal commands.

## Dependencies

- `add-multi-selection`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Context-aware Things-style creation control and insertion positioning.

## ADDED Requirements

### Requirement: Contextual creation

Magic Plus SHALL create a to-do in the currently visible logical context by default.

#### Scenario: Contextual creation — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the add control is activated normally
- **THEN** a new editable item appears at the expected insertion point/context

#### Scenario: Contextual creation — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Drag insertion

Dragging the add control between rows SHALL create at the indicated rank/location.

#### Scenario: Drag insertion — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the control is dropped between existing items
- **THEN** the new item receives a rank between those neighbors

#### Scenario: Drag insertion — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Alternate creation targets

Where supported, dragging the creation control toward heading/Inbox affordances SHALL create the appropriate entity/context through normal commands.

#### Scenario: Alternate creation targets — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the control is dropped on a special creation target
- **THEN** the corresponding create command executes with deterministic context

#### Scenario: Alternate creation targets — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Context-aware Things-style creation control and insertion positioning** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Contextual creation`
- `Drag insertion`
- `Alternate creation targets`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Contextual creation` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Drag insertion` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Alternate creation targets` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Contextual creation`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Drag insertion`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Alternate creation targets`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-magic-plus` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-magic-plus --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-magic-plus --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 35 — `add-expanded-item-editor`

**Required completed outcome:** Smooth row-to-expanded editor with title, notes and optional metadata controls

## Proposal blueprint

## Why

This change delivers **Smooth row-to-expanded editor with title, notes and optional metadata controls** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Inline expansion: A task row SHALL expand into an editor without losing list position, selection context, or canonical identity.
- Progressive fields: Title and notes SHALL remain primary while schedule, deadline, tags, checklist, and recurrence controls are accessible without permanently cluttering the closed row.
- Commit/cancel editing: Editor changes SHALL have deterministic persistence/optimistic behavior and keyboard/pointer dismissal semantics.

## Dependencies

- `add-magic-plus`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Smooth row-to-expanded editor with title, notes and optional metadata controls.

## ADDED Requirements

### Requirement: Inline expansion

A task row SHALL expand into an editor without losing list position, selection context, or canonical identity.

#### Scenario: Inline expansion — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user opens a task
- **THEN** the row transitions into an editable detail surface

#### Scenario: Inline expansion — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Progressive fields

Title and notes SHALL remain primary while schedule, deadline, tags, checklist, and recurrence controls are accessible without permanently cluttering the closed row.

#### Scenario: Progressive fields — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the editor is expanded
- **THEN** optional metadata controls can be revealed and changed

#### Scenario: Progressive fields — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Commit/cancel editing

Editor changes SHALL have deterministic persistence/optimistic behavior and keyboard/pointer dismissal semantics.

#### Scenario: Commit/cancel editing — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user finishes or abandons an edit
- **THEN** the resulting saved/reverted state matches the defined editor policy

#### Scenario: Commit/cancel editing — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Smooth row-to-expanded editor with title, notes and optional metadata controls** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Inline expansion`
- `Progressive fields`
- `Commit/cancel editing`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Inline expansion` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Progressive fields` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Commit/cancel editing` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Inline expansion`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Progressive fields`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Commit/cancel editing`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-expanded-item-editor` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-expanded-item-editor --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-expanded-item-editor --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 36 — `add-date-pickers`

**Required completed outcome:** When/deadline/reminder date interfaces and quick destinations

## Proposal blueprint

## Why

This change delivers **When/deadline/reminder date interfaces and quick destinations** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- When picker: The date UI SHALL provide Today, This Evening, Tomorrow, Anytime, Someday, explicit date, and clear actions where valid.
- Deadline picker: The deadline UI SHALL select or clear an explicit deadline independently from schedule/reminder.
- Reminder time picker: A reminder time SHALL be attachable only under valid start-date semantics and represented in the user's timezone.

## Dependencies

- `add-expanded-item-editor`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

When/deadline/reminder date interfaces and quick destinations.

## ADDED Requirements

### Requirement: When picker

The date UI SHALL provide Today, This Evening, Tomorrow, Anytime, Someday, explicit date, and clear actions where valid.

#### Scenario: When picker — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user opens the When control
- **THEN** available schedule destinations can be selected without editing unrelated fields

#### Scenario: When picker — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Deadline picker

The deadline UI SHALL select or clear an explicit deadline independently from schedule/reminder.

#### Scenario: Deadline picker — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a deadline date is chosen
- **THEN** only deadline state changes

#### Scenario: Deadline picker — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Reminder time picker

A reminder time SHALL be attachable only under valid start-date semantics and represented in the user's timezone.

#### Scenario: Reminder time picker — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a reminder is set
- **THEN** the reminder resolves to the intended local instant and schedule constraints are enforced

#### Scenario: Reminder time picker — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **When/deadline/reminder date interfaces and quick destinations** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `When picker`
- `Deadline picker`
- `Reminder time picker`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `When picker` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Deadline picker` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Reminder time picker` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `When picker`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Deadline picker`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Reminder time picker`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-date-pickers` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-date-pickers --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-date-pickers --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 37 — `add-natural-language-dates`

**Required completed outcome:** Deterministic Rust date parser with tested supported expressions

## Proposal blueprint

## Why

This change delivers **Deterministic Rust date parser with tested supported expressions** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Deterministic natural-date parsing: The Rust parser SHALL recognize supported relative dates, weekdays, month/day expressions, offsets, and reminder times without network or LLM dependence.
- Locale-aware parsing: Parsing SHALL be modular by locale and SHALL respect configured regional date conventions where ambiguity exists.
- Ambiguity handling: Ambiguous input SHALL produce explicit candidates or a recoverable parse failure rather than silently choosing an unsafe date.

## Dependencies

- `add-date-pickers`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Deterministic Rust date parser with tested supported expressions.

## ADDED Requirements

### Requirement: Deterministic natural-date parsing

The Rust parser SHALL recognize supported relative dates, weekdays, month/day expressions, offsets, and reminder times without network or LLM dependence.

#### Scenario: Deterministic natural-date parsing — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a supported phrase such as `17d from jul9` is entered
- **THEN** the parser returns the deterministic intended date/time candidate

#### Scenario: Deterministic natural-date parsing — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Locale-aware parsing

Parsing SHALL be modular by locale and SHALL respect configured regional date conventions where ambiguity exists.

#### Scenario: Locale-aware parsing — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the parser runs under a supported locale
- **THEN** tokens and numeric date formats follow that locale's rules

#### Scenario: Locale-aware parsing — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Ambiguity handling

Ambiguous input SHALL produce explicit candidates or a recoverable parse failure rather than silently choosing an unsafe date.

#### Scenario: Ambiguity handling — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** input has multiple plausible interpretations
- **THEN** the UI can request/offer disambiguation

#### Scenario: Ambiguity handling — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Deterministic Rust date parser with tested supported expressions** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Deterministic natural-date parsing`
- `Locale-aware parsing`
- `Ambiguity handling`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Deterministic natural-date parsing` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Locale-aware parsing` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Ambiguity handling` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Deterministic natural-date parsing`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Locale-aware parsing`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Ambiguity handling`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-natural-language-dates` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-natural-language-dates --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-natural-language-dates --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 38 — `add-reminders`

**Required completed outcome:** Reminder persistence, scheduler, browser Web Push and routing

## Proposal blueprint

## Why

This change delivers **Reminder persistence, scheduler, browser Web Push and routing** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Reminder persistence: Reminder configuration SHALL persist local date/time, timezone context, owning entity, and delivery state separately from deadlines.
- Reliable scheduler: The backend scheduler SHALL deliver due reminders idempotently and tolerate restarts without duplicate delivery storms.
- Web Push delivery: Browsers with granted permission SHALL receive Web Push notifications that deep-link to the owning item.

## Dependencies

- `add-date-pickers`
- `add-natural-language-dates`
- `add-persistence-foundation`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Reminder persistence, scheduler, browser Web Push and routing.

## ADDED Requirements

### Requirement: Reminder persistence

Reminder configuration SHALL persist local date/time, timezone context, owning entity, and delivery state separately from deadlines.

#### Scenario: Reminder persistence — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a reminder is saved
- **THEN** it can be scheduled without changing deadline state

#### Scenario: Reminder persistence — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Reliable scheduler

The backend scheduler SHALL deliver due reminders idempotently and tolerate restarts without duplicate delivery storms.

#### Scenario: Reliable scheduler — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a worker restarts around a due reminder
- **THEN** the reminder is eventually delivered once according to the delivery policy

#### Scenario: Reliable scheduler — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Web Push delivery

Browsers with granted permission SHALL receive Web Push notifications that deep-link to the owning item.

#### Scenario: Web Push delivery — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a due reminder is delivered to an active subscription
- **THEN** the notification is shown and opening it navigates to the correct entity

#### Scenario: Web Push delivery — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Reminder persistence, scheduler, browser Web Push and routing** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Reminder persistence`
- `Reliable scheduler`
- `Web Push delivery`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Reminder persistence` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Reliable scheduler` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Web Push delivery` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Reminder persistence`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Reliable scheduler`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Web Push delivery`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-reminders` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-reminders --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-reminders --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 39 — `add-reminder-snoozing`

**Required completed outcome:** Snooze durations, tomorrow behavior and scheduler integration

## Proposal blueprint

## Why

This change delivers **Snooze durations, tomorrow behavior and scheduler integration** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Snooze choices: Delivered reminders SHALL support configured snooze choices including short durations and next-day behavior.
- Snooze idempotency: Repeated clicks/retries on the same snooze action SHALL not create duplicate scheduled deliveries.
- Tomorrow semantics: Snoozing to tomorrow SHALL resolve using the user's local timezone/date policy rather than adding a fixed 24 hours.

## Dependencies

- `add-reminders`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Snooze durations, tomorrow behavior and scheduler integration.

## ADDED Requirements

### Requirement: Snooze choices

Delivered reminders SHALL support configured snooze choices including short durations and next-day behavior.

#### Scenario: Snooze choices — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user snoozes a reminder
- **THEN** a new delivery time is computed from the selected choice

#### Scenario: Snooze choices — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Snooze idempotency

Repeated clicks/retries on the same snooze action SHALL not create duplicate scheduled deliveries.

#### Scenario: Snooze idempotency — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the same snooze command is retried
- **THEN** the resulting reminder schedule contains one logical snooze

#### Scenario: Snooze idempotency — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Tomorrow semantics

Snoozing to tomorrow SHALL resolve using the user's local timezone/date policy rather than adding a fixed 24 hours.

#### Scenario: Tomorrow semantics — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a DST boundary occurs before tomorrow
- **THEN** the reminder still lands on the intended local tomorrow time

#### Scenario: Tomorrow semantics — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Snooze durations, tomorrow behavior and scheduler integration** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Snooze choices`
- `Snooze idempotency`
- `Tomorrow semantics`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Snooze choices` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Snooze idempotency` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Tomorrow semantics` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Snooze choices`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Snooze idempotency`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Tomorrow semantics`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-reminder-snoozing` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-reminder-snoozing --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-reminder-snoozing --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 40 — `add-recurrence-model`

**Required completed outcome:** Regular and after-completion recurrence domain including end conditions

## Proposal blueprint

## Why

This change delivers **Regular and after-completion recurrence domain including end conditions** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Recurrence patterns: The domain SHALL support regular daily/weekly/monthly/yearly schedules and after-completion schedules for to-dos and projects.
- Pattern detail: Regular recurrence SHALL support selected weekdays/dates and optional end conditions where defined.
- Template inheritance: A recurrence template SHALL carry the future-copy title, notes, tags, checklist/project structure, reminder/deadline rules, and context without sharing mutable copy state.

## Dependencies

- `add-reminder-snoozing`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Regular and after-completion recurrence domain including end conditions.

## ADDED Requirements

### Requirement: Recurrence patterns

The domain SHALL support regular daily/weekly/monthly/yearly schedules and after-completion schedules for to-dos and projects.

#### Scenario: Recurrence patterns — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a recurrence template is created
- **THEN** its pattern can deterministically compute eligible future occurrences

#### Scenario: Recurrence patterns — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Pattern detail

Regular recurrence SHALL support selected weekdays/dates and optional end conditions where defined.

#### Scenario: Pattern detail — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a custom regular pattern is configured
- **THEN** only dates satisfying the rule are generated

#### Scenario: Pattern detail — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Template inheritance

A recurrence template SHALL carry the future-copy title, notes, tags, checklist/project structure, reminder/deadline rules, and context without sharing mutable copy state.

#### Scenario: Template inheritance — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a future copy is materialized
- **THEN** it inherits template content according to copy semantics

#### Scenario: Template inheritance — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Regular and after-completion recurrence domain including end conditions** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Recurrence patterns`
- `Pattern detail`
- `Template inheritance`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Recurrence patterns` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Pattern detail` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Template inheritance` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Recurrence patterns`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Pattern detail`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Template inheritance`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-recurrence-model` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-recurrence-model --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-recurrence-model --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 41 — `add-recurrence-materializer`

**Required completed outcome:** Concurrent-safe, idempotent generation of recurrence instances

## Proposal blueprint

## Why

This change delivers **Concurrent-safe, idempotent generation of recurrence instances** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Idempotent materialization: For a given template and logical occurrence, at most one generated copy SHALL exist.
- Horizon generation: The materializer SHALL generate the required upcoming occurrences far enough for Upcoming/reminder behavior without unbounded eager generation.
- Restart safety: Materialization progress SHALL be recoverable from persisted recurrence/template state rather than in-memory scheduler state.

## Dependencies

- `add-recurrence-model`
- `add-persistence-foundation`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Concurrent-safe, idempotent generation of recurrence instances.

## ADDED Requirements

### Requirement: Idempotent materialization

For a given template and logical occurrence, at most one generated copy SHALL exist.

#### Scenario: Idempotent materialization — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** two workers attempt the same occurrence concurrently
- **THEN** one logical copy is committed

#### Scenario: Idempotent materialization — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Horizon generation

The materializer SHALL generate the required upcoming occurrences far enough for Upcoming/reminder behavior without unbounded eager generation.

#### Scenario: Horizon generation — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the worker processes active templates
- **THEN** the configured horizon is populated and later runs extend it safely

#### Scenario: Horizon generation — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Restart safety

Materialization progress SHALL be recoverable from persisted recurrence/template state rather than in-memory scheduler state.

#### Scenario: Restart safety — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the worker restarts
- **THEN** missing eligible occurrences are created without duplicating existing ones

#### Scenario: Restart safety — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Concurrent-safe, idempotent generation of recurrence instances** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Idempotent materialization`
- `Horizon generation`
- `Restart safety`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Idempotent materialization` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Horizon generation` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Restart safety` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Idempotent materialization`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Horizon generation`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Restart safety`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-recurrence-materializer` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-recurrence-materializer --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-recurrence-materializer --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 42 — `add-recurrence-exceptions`

**Required completed outcome:** Early completion, one-time schedule overrides and future-rule editing

## Proposal blueprint

## Why

This change delivers **Early completion, one-time schedule overrides and future-rule editing** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Early completion: A scheduled recurrence copy/template SHALL support early completion according to defined recurrence semantics.
- One-time versus future edit: Rescheduling/editing a generated occurrence SHALL distinguish a one-time exception from modifying the recurrence template/future rule.
- After-completion anchor: For after-completion recurrence, the next occurrence SHALL be anchored to the actual completion date/time according to the rule.

## Dependencies

- `add-recurrence-materializer`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Early completion, one-time schedule overrides and future-rule editing.

## ADDED Requirements

### Requirement: Early completion

A scheduled recurrence copy/template SHALL support early completion according to defined recurrence semantics.

#### Scenario: Early completion — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** an eligible upcoming recurring item is completed before its scheduled date
- **THEN** the occurrence is completed and future generation remains correct

#### Scenario: Early completion — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: One-time versus future edit

Rescheduling/editing a generated occurrence SHALL distinguish a one-time exception from modifying the recurrence template/future rule.

#### Scenario: One-time versus future edit — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user edits one recurring occurrence
- **THEN** only the selected occurrence changes unless the future-rule action is chosen

#### Scenario: One-time versus future edit — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: After-completion anchor

For after-completion recurrence, the next occurrence SHALL be anchored to the actual completion date/time according to the rule.

#### Scenario: After-completion anchor — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a copy is completed late
- **THEN** the next scheduled copy moves relative to that completion rather than the original planned date

#### Scenario: After-completion anchor — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Early completion, one-time schedule overrides and future-rule editing** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Early completion`
- `One-time versus future edit`
- `After-completion anchor`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Early completion` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `One-time versus future edit` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `After-completion anchor` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Early completion`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `One-time versus future edit`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `After-completion anchor`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-recurrence-exceptions` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-recurrence-exceptions --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-recurrence-exceptions --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 43 — `add-quick-find`

**Required completed outcome:** Type-to-search navigation across entities, tags, lists and settings

## Proposal blueprint

## Why

This change delivers **Type-to-search navigation across entities, tags, lists and settings** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Type-to-open Quick Find: Typing outside an editor SHALL open Quick Find and seed it with the typed query.
- Destination search: Quick Find SHALL search built-in lists, special lists, areas, projects, headings, tags, and to-do titles for navigation.
- Recent and exact navigation: Exact/high-confidence destinations and recent destinations SHALL receive deterministic ranking without hiding other matches.

## Dependencies

- `add-web-application-shell`
- `add-special-lists`
- `add-tags`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Type-to-search navigation across entities, tags, lists and settings.

## ADDED Requirements

### Requirement: Type-to-open Quick Find

Typing outside an editor SHALL open Quick Find and seed it with the typed query.

#### Scenario: Type-to-open Quick Find — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a printable key is pressed while no text field owns focus
- **THEN** Quick Find opens without dropping the typed character

#### Scenario: Type-to-open Quick Find — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Destination search

Quick Find SHALL search built-in lists, special lists, areas, projects, headings, tags, and to-do titles for navigation.

#### Scenario: Destination search — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a destination name is queried
- **THEN** matching navigable destinations are ranked and selectable

#### Scenario: Destination search — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Recent and exact navigation

Exact/high-confidence destinations and recent destinations SHALL receive deterministic ranking without hiding other matches.

#### Scenario: Recent and exact navigation — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the user repeats a recent navigation query
- **THEN** the recent destination is easy to select while ranking remains stable

#### Scenario: Recent and exact navigation — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Type-to-search navigation across entities, tags, lists and settings** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Type-to-open Quick Find`
- `Destination search`
- `Recent and exact navigation`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Type-to-open Quick Find` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Destination search` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Recent and exact navigation` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Type-to-open Quick Find`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Destination search`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Recent and exact navigation`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-quick-find` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-quick-find --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-quick-find --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 44 — `add-deep-search`

**Required completed outcome:** Search notes/checklists/logged content with ranking and normalization

## Proposal blueprint

## Why

This change delivers **Search notes/checklists/logged content with ranking and normalization** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Deep content search: Continue/deep search SHALL include notes, checklist text, and logged/completed content in addition to normal Quick Find scope.
- Normalized matching: Search SHALL be case-insensitive and diacritic-normalized while preserving original display text.
- Offline/server consistency: The local offline search index and server-backed search SHALL use compatible tokenization/ranking semantics sufficient to avoid surprising result loss.

## Dependencies

- `add-quick-find`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Search notes/checklists/logged content with ranking and normalization.

## ADDED Requirements

### Requirement: Deep content search

Continue/deep search SHALL include notes, checklist text, and logged/completed content in addition to normal Quick Find scope.

#### Scenario: Deep content search — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user expands a Quick Find search
- **THEN** matching deep content appears with its parent entity context

#### Scenario: Deep content search — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Normalized matching

Search SHALL be case-insensitive and diacritic-normalized while preserving original display text.

#### Scenario: Normalized matching — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a query omits case/diacritics
- **THEN** equivalent content still matches

#### Scenario: Normalized matching — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Offline/server consistency

The local offline search index and server-backed search SHALL use compatible tokenization/ranking semantics sufficient to avoid surprising result loss.

#### Scenario: Offline/server consistency — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the same synced dataset is searched online and offline
- **THEN** core matching results are materially consistent

#### Scenario: Offline/server consistency — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Search notes/checklists/logged content with ranking and normalization** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Deep content search`
- `Normalized matching`
- `Offline/server consistency`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Deep content search` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Normalized matching` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Offline/server consistency` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Deep content search`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Normalized matching`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Offline/server consistency`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-deep-search` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-deep-search --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-deep-search --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 45 — `add-keyboard-command-system`

**Required completed outcome:** Central command/shortcut registry covering navigation and task operations

## Proposal blueprint

## Why

This change delivers **Central command/shortcut registry covering navigation and task operations** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Central shortcut registry: Keyboard shortcuts SHALL be declared in a centralized command registry with availability predicates rather than hard-coded per component.
- Command coverage: Keyboard commands SHALL cover creation, completion/cancellation, scheduling/deadline, navigation/search, move, tags, selection, and reordering as specified.
- Conflict handling: Browser/OS-reserved shortcut conflicts SHALL be mapped to safe web equivalents and documented.

## Dependencies

- `add-deep-search`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Central command/shortcut registry covering navigation and task operations.

## ADDED Requirements

### Requirement: Central shortcut registry

Keyboard shortcuts SHALL be declared in a centralized command registry with availability predicates rather than hard-coded per component.

#### Scenario: Central shortcut registry — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a shortcut is pressed
- **THEN** the registry resolves the active command for current context

#### Scenario: Central shortcut registry — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Command coverage

Keyboard commands SHALL cover creation, completion/cancellation, scheduling/deadline, navigation/search, move, tags, selection, and reordering as specified.

#### Scenario: Command coverage — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a keyboard-only user performs a supported workflow
- **THEN** the action is reachable without pointer input

#### Scenario: Command coverage — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Conflict handling

Browser/OS-reserved shortcut conflicts SHALL be mapped to safe web equivalents and documented.

#### Scenario: Conflict handling — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a native Things shortcut cannot be captured reliably in browsers
- **THEN** the web mapping remains discoverable and does not break essential browser behavior

#### Scenario: Conflict handling — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Central command/shortcut registry covering navigation and task operations** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Central shortcut registry`
- `Command coverage`
- `Conflict handling`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Central shortcut registry` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Command coverage` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Conflict handling` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Central shortcut registry`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Command coverage`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Conflict handling`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-keyboard-command-system` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-keyboard-command-system --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-keyboard-command-system --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 46 — `add-type-travel`

**Required completed outcome:** Typing outside editors immediately invokes navigation/search behavior

## Proposal blueprint

## Why

This change delivers **Typing outside editors immediately invokes navigation/search behavior** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Type travel: Printable typing outside editable controls SHALL transition immediately into destination/search navigation.
- Editor exemption: Typing inside note/title/checklist/date text inputs SHALL never trigger Type Travel.
- Escape/restore focus: Leaving Type Travel SHALL restore a sensible prior navigation focus target.

## Dependencies

- `add-keyboard-command-system`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Typing outside editors immediately invokes navigation/search behavior.

## ADDED Requirements

### Requirement: Type travel

Printable typing outside editable controls SHALL transition immediately into destination/search navigation.

#### Scenario: Type travel — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the user starts typing in a list
- **THEN** the application opens search/navigation rather than discarding input

#### Scenario: Type travel — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Editor exemption

Typing inside note/title/checklist/date text inputs SHALL never trigger Type Travel.

#### Scenario: Editor exemption — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a text editor owns focus
- **THEN** input remains in that editor

#### Scenario: Editor exemption — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Escape/restore focus

Leaving Type Travel SHALL restore a sensible prior navigation focus target.

#### Scenario: Escape/restore focus — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the user dismisses Quick Find
- **THEN** keyboard navigation can continue from the prior context

#### Scenario: Escape/restore focus — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Typing outside editors immediately invokes navigation/search behavior** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Type travel`
- `Editor exemption`
- `Escape/restore focus`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Type travel` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Editor exemption` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Escape/restore focus` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Type travel`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Editor exemption`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Escape/restore focus`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-type-travel` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-type-travel --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-type-travel --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 47 — `add-mobile-gestures`

**Required completed outcome:** Swipe, pull, long-press, drag and touch multi-selection state machines

## Proposal blueprint

## Why

This change delivers **Swipe, pull, long-press, drag and touch multi-selection state machines** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Swipe actions: Mobile pointer handling SHALL expose configured swipe actions such as When or selection without interfering with ordinary scrolling.
- Long-press drag: Long press SHALL enter reorder/move mode where supported while suppressing accidental click activation.
- Pull-to-search: A supported downward gesture at the appropriate scroll boundary SHALL reveal Quick Find/navigation.

## Dependencies

- `add-type-travel`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Swipe, pull, long-press, drag and touch multi-selection state machines.

## ADDED Requirements

### Requirement: Swipe actions

Mobile pointer handling SHALL expose configured swipe actions such as When or selection without interfering with ordinary scrolling.

#### Scenario: Swipe actions — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a horizontal gesture crosses the action threshold
- **THEN** the corresponding action is revealed/executed

#### Scenario: Swipe actions — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Long-press drag

Long press SHALL enter reorder/move mode where supported while suppressing accidental click activation.

#### Scenario: Long-press drag — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a row is held beyond the gesture threshold
- **THEN** drag affordance begins predictably

#### Scenario: Long-press drag — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Pull-to-search

A supported downward gesture at the appropriate scroll boundary SHALL reveal Quick Find/navigation.

#### Scenario: Pull-to-search — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the user pulls down from the top of a list
- **THEN** Quick Find becomes available without browser refresh conflicts

#### Scenario: Pull-to-search — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Swipe, pull, long-press, drag and touch multi-selection state machines** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Swipe actions`
- `Long-press drag`
- `Pull-to-search`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Swipe actions` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Long-press drag` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Pull-to-search` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Swipe actions`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Long-press drag`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Pull-to-search`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-mobile-gestures` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-mobile-gestures --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-mobile-gestures --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 48 — `add-calendar-integration`

**Required completed outcome:** Provider-neutral calendar interface plus initial web calendar adapters

## Proposal blueprint

## Why

This change delivers **Provider-neutral calendar interface plus initial web calendar adapters** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Provider-neutral calendar API: Core application code SHALL depend on a CalendarProvider contract rather than Google/Microsoft/CalDAV-specific types.
- Read-only projection: Connected calendar events SHALL appear in Today/Upcoming as external read-only rows by default.
- Provider resilience: Calendar failures/auth expiry SHALL degrade the calendar projection without blocking task data or corrupting cached calendar state.

## Dependencies

- `add-today-projection`
- `add-upcoming-projection`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Provider-neutral calendar interface plus initial web calendar adapters.

## ADDED Requirements

### Requirement: Provider-neutral calendar API

Core application code SHALL depend on a CalendarProvider contract rather than Google/Microsoft/CalDAV-specific types.

#### Scenario: Provider-neutral calendar API — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** calendar data is requested
- **THEN** the application receives canonical calendar/event models

#### Scenario: Provider-neutral calendar API — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Read-only projection

Connected calendar events SHALL appear in Today/Upcoming as external read-only rows by default.

#### Scenario: Read-only projection — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** an event falls on a rendered date
- **THEN** it is displayed without becoming a Todo entity

#### Scenario: Read-only projection — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Provider resilience

Calendar failures/auth expiry SHALL degrade the calendar projection without blocking task data or corrupting cached calendar state.

#### Scenario: Provider resilience — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a provider request fails
- **THEN** task views remain usable and the integration exposes a reconnect/error state

#### Scenario: Provider resilience — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Provider-neutral calendar interface plus initial web calendar adapters** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Provider-neutral calendar API`
- `Read-only projection`
- `Provider resilience`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Provider-neutral calendar API` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Read-only projection` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Provider resilience` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Provider-neutral calendar API`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Read-only projection`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Provider resilience`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-calendar-integration` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-calendar-integration --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-calendar-integration --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 49 — `add-offline-local-replica`

**Required completed outcome:** IndexedDB local replica, optimistic writes, outbox and offline startup

## Proposal blueprint

## Why

This change delivers **IndexedDB local replica, optimistic writes, outbox and offline startup** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- IndexedDB local replica: The browser SHALL persist the user's synchronized working set, preferences needed offline, sync cursor, and outbound operations in IndexedDB.
- Optimistic local commands: Supported mutations SHALL update the local replica/UI before server acknowledgement and enqueue a durable outbound operation.
- Outbox recovery: The outbox SHALL survive browser reloads and replay idempotently after connectivity returns.

## Dependencies

- `add-command-transaction-layer`
- `add-persistence-foundation`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

IndexedDB local replica, optimistic writes, outbox and offline startup.

## ADDED Requirements

### Requirement: IndexedDB local replica

The browser SHALL persist the user's synchronized working set, preferences needed offline, sync cursor, and outbound operations in IndexedDB.

#### Scenario: IndexedDB local replica — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the app reloads offline after prior sync
- **THEN** the user can view cached data and continue supported mutations

#### Scenario: IndexedDB local replica — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Optimistic local commands

Supported mutations SHALL update the local replica/UI before server acknowledgement and enqueue a durable outbound operation.

#### Scenario: Optimistic local commands — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user completes a task offline
- **THEN** the UI updates immediately and the operation remains queued

#### Scenario: Optimistic local commands — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Outbox recovery

The outbox SHALL survive browser reloads and replay idempotently after connectivity returns.

#### Scenario: Outbox recovery — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** connectivity returns after queued edits
- **THEN** pending operations are pushed without duplication

#### Scenario: Outbox recovery — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **IndexedDB local replica, optimistic writes, outbox and offline startup** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `IndexedDB local replica`
- `Optimistic local commands`
- `Outbox recovery`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `IndexedDB local replica` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Optimistic local commands` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Outbox recovery` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `IndexedDB local replica`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Optimistic local commands`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Outbox recovery`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-offline-local-replica` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-offline-local-replica --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-offline-local-replica --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 50 — `add-cloud-sync-protocol`

**Required completed outcome:** Incremental push/pull, cursors, operation IDs, versions and conflict rules

## Proposal blueprint

## Why

This change delivers **Incremental push/pull, cursors, operation IDs, versions and conflict rules** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Incremental push/pull protocol: Clients SHALL push idempotent operations and pull server changes using monotonic per-account cursor/sequence semantics.
- Idempotent operation IDs: The server SHALL deduplicate retried client operations by stable client operation identifier.
- Conflict policy: The sync layer SHALL implement explicit field/domain conflict rules for concurrent edits, deletes, completion, and ordering rather than generic destructive last-write-wins.

## Dependencies

- `add-offline-local-replica`
- `add-command-transaction-layer`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Incremental push/pull, cursors, operation IDs, versions and conflict rules.

## ADDED Requirements

### Requirement: Incremental push/pull protocol

Clients SHALL push idempotent operations and pull server changes using monotonic per-account cursor/sequence semantics.

#### Scenario: Incremental push/pull protocol — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a client reconnects after being offline
- **THEN** only unsent operations and unseen server changes are exchanged

#### Scenario: Incremental push/pull protocol — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Idempotent operation IDs

The server SHALL deduplicate retried client operations by stable client operation identifier.

#### Scenario: Idempotent operation IDs — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a network retry resubmits a successful mutation
- **THEN** the mutation is not applied twice

#### Scenario: Idempotent operation IDs — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Conflict policy

The sync layer SHALL implement explicit field/domain conflict rules for concurrent edits, deletes, completion, and ordering rather than generic destructive last-write-wins.

#### Scenario: Conflict policy — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** two clients change overlapping state
- **THEN** the documented resolver produces a deterministic convergent result

#### Scenario: Conflict policy — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Incremental push/pull, cursors, operation IDs, versions and conflict rules** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Incremental push/pull protocol`
- `Idempotent operation IDs`
- `Conflict policy`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Incremental push/pull protocol` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Idempotent operation IDs` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Conflict policy` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Incremental push/pull protocol`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Idempotent operation IDs`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Conflict policy`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-cloud-sync-protocol` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-cloud-sync-protocol --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-cloud-sync-protocol --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 51 — `add-realtime-sync`

**Required completed outcome:** WebSocket/SSE invalidation hints feeding the canonical sync protocol

## Proposal blueprint

## Why

This change delivers **WebSocket/SSE invalidation hints feeding the canonical sync protocol** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Realtime invalidation hints: Realtime transport SHALL notify connected clients that newer server sequence data exists without introducing a second mutation protocol.
- Reconnect behavior: Realtime disconnect/reconnect SHALL not lose correctness because cursor-based sync remains authoritative.
- Backpressure/coalescing: Repeated realtime hints SHALL be coalesced so a busy account does not create unbounded simultaneous pull loops.

## Dependencies

- `add-cloud-sync-protocol`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

WebSocket/SSE invalidation hints feeding the canonical sync protocol.

## ADDED Requirements

### Requirement: Realtime invalidation hints

Realtime transport SHALL notify connected clients that newer server sequence data exists without introducing a second mutation protocol.

#### Scenario: Realtime invalidation hints — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** another device commits a change
- **THEN** the connected client is prompted to run normal incremental pull

#### Scenario: Realtime invalidation hints — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Reconnect behavior

Realtime disconnect/reconnect SHALL not lose correctness because cursor-based sync remains authoritative.

#### Scenario: Reconnect behavior — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the WebSocket/SSE connection drops
- **THEN** the client later catches up from its cursor

#### Scenario: Reconnect behavior — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Backpressure/coalescing

Repeated realtime hints SHALL be coalesced so a busy account does not create unbounded simultaneous pull loops.

#### Scenario: Backpressure/coalescing — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** many changes arrive quickly
- **THEN** the client schedules bounded sync work

#### Scenario: Backpressure/coalescing — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **WebSocket/SSE invalidation hints feeding the canonical sync protocol** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Realtime invalidation hints`
- `Reconnect behavior`
- `Backpressure/coalescing`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Realtime invalidation hints` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Reconnect behavior` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Backpressure/coalescing` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Realtime invalidation hints`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Reconnect behavior`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Backpressure/coalescing`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-realtime-sync` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-realtime-sync --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-realtime-sync --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 52 — `add-multitab-coordination`

**Required completed outcome:** BroadcastChannel/local leadership and synchronized browser-tab replicas

## Proposal blueprint

## Why

This change delivers **BroadcastChannel/local leadership and synchronized browser-tab replicas** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Multi-tab coordination: Tabs SHALL coordinate sync/outbox ownership through browser-native coordination so duplicate workers are minimized.
- Cross-tab state propagation: Committed local/server changes SHALL be broadcast so other tabs refresh their replica/view promptly.
- Leader recovery: If the coordinating tab closes, another tab SHALL take over without losing outbox work.

## Dependencies

- `add-realtime-sync`
- `add-offline-local-replica`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

BroadcastChannel/local leadership and synchronized browser-tab replicas.

## ADDED Requirements

### Requirement: Multi-tab coordination

Tabs SHALL coordinate sync/outbox ownership through browser-native coordination so duplicate workers are minimized.

#### Scenario: Multi-tab coordination — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** two tabs are open for the same account
- **THEN** only the elected/appropriate tab drives shared sync work where possible

#### Scenario: Multi-tab coordination — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Cross-tab state propagation

Committed local/server changes SHALL be broadcast so other tabs refresh their replica/view promptly.

#### Scenario: Cross-tab state propagation — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** one tab completes a to-do
- **THEN** the other tab reflects the update without full reload

#### Scenario: Cross-tab state propagation — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Leader recovery

If the coordinating tab closes, another tab SHALL take over without losing outbox work.

#### Scenario: Leader recovery — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the leader tab exits
- **THEN** a remaining tab resumes coordination

#### Scenario: Leader recovery — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **BroadcastChannel/local leadership and synchronized browser-tab replicas** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Multi-tab coordination`
- `Cross-tab state propagation`
- `Leader recovery`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Multi-tab coordination` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Cross-tab state propagation` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Leader recovery` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Multi-tab coordination`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Cross-tab state propagation`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Leader recovery`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-multitab-coordination` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-multitab-coordination --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-multitab-coordination --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 53 — `add-quick-entry`

**Required completed outcome:** Keyboard-invoked capture overlay that saves without leaving current context

## Proposal blueprint

## Why

This change delivers **Keyboard-invoked capture overlay that saves without leaving current context** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Quick Entry overlay: A keyboard-accessible overlay SHALL capture a new to-do without navigating away from the current destination.
- Full capture metadata: Quick Entry SHALL support title, notes, schedule, deadline, tags, destination, checklist, and other supported capture metadata.
- Dismiss/save semantics: Save and cancel SHALL have deterministic keyboard behavior and never leave a half-created canonical item.

## Dependencies

- `add-multitab-coordination`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Keyboard-invoked capture overlay that saves without leaving current context.

## ADDED Requirements

### Requirement: Quick Entry overlay

A keyboard-accessible overlay SHALL capture a new to-do without navigating away from the current destination.

#### Scenario: Quick Entry overlay — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the Quick Entry command is invoked
- **THEN** an editor opens above the current view and focus moves into title entry

#### Scenario: Quick Entry overlay — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Full capture metadata

Quick Entry SHALL support title, notes, schedule, deadline, tags, destination, checklist, and other supported capture metadata.

#### Scenario: Full capture metadata — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** metadata is entered before save
- **THEN** the created to-do receives it through the normal create command

#### Scenario: Full capture metadata — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Dismiss/save semantics

Save and cancel SHALL have deterministic keyboard behavior and never leave a half-created canonical item.

#### Scenario: Dismiss/save semantics — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the overlay is submitted or dismissed
- **THEN** one valid task is created or no task is created

#### Scenario: Dismiss/save semantics — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Keyboard-invoked capture overlay that saves without leaving current context** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Quick Entry overlay`
- `Full capture metadata`
- `Dismiss/save semantics`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Quick Entry overlay` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Full capture metadata` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Dismiss/save semantics` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Quick Entry overlay`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Full capture metadata`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Dismiss/save semantics`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-quick-entry` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-quick-entry --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-quick-entry --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 54 — `add-browser-capture`

**Required completed outcome:** PWA/share/browser capture of title, URL and selected text

## Proposal blueprint

## Why

This change delivers **PWA/share/browser capture of title, URL and selected text** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Web share capture: The PWA/browser integration SHALL accept shared page title, URL, and selected text where browser capabilities provide them.
- Extension/bookmark adapter boundary: Optional browser-extension/bookmark capture SHALL call documented command URLs/API rather than duplicate domain logic.
- Sanitized external text: Captured third-party page content SHALL be treated as untrusted text and sanitized for display/navigation.

## Dependencies

- `add-quick-entry`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

PWA/share/browser capture of title, URL and selected text.

## ADDED Requirements

### Requirement: Web share capture

The PWA/browser integration SHALL accept shared page title, URL, and selected text where browser capabilities provide them.

#### Scenario: Web share capture — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** content is shared to the app
- **THEN** a prefilled capture flow opens

#### Scenario: Web share capture — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Extension/bookmark adapter boundary

Optional browser-extension/bookmark capture SHALL call documented command URLs/API rather than duplicate domain logic.

#### Scenario: Extension/bookmark adapter boundary — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** an external capture surface creates a task
- **THEN** the same create command/invariants apply

#### Scenario: Extension/bookmark adapter boundary — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Sanitized external text

Captured third-party page content SHALL be treated as untrusted text and sanitized for display/navigation.

#### Scenario: Sanitized external text — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a page title or selection contains markup/script-like content
- **THEN** it is stored/rendered safely as task text

#### Scenario: Sanitized external text — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **PWA/share/browser capture of title, URL and selected text** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Web share capture`
- `Extension/bookmark adapter boundary`
- `Sanitized external text`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Web share capture` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Extension/bookmark adapter boundary` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Sanitized external text` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Web share capture`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Extension/bookmark adapter boundary`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Sanitized external text`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-browser-capture` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-browser-capture --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-browser-capture --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 55 — `add-mail-to-inbox`

**Required completed outcome:** Secure inbound email address translating mail to Inbox tasks

## Proposal blueprint

## Why

This change delivers **Secure inbound email address translating mail to Inbox tasks** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Private inbound address: Each enabled account SHALL receive a high-entropy inbound email alias/token that can be rotated or disabled.
- Mail-to-task mapping: Inbound mail SHALL map subject to title and supported body text to notes using deterministic normalization.
- Abuse and duplicate protection: Email ingress SHALL enforce size/rate limits and message-id/idempotency handling.

## Dependencies

- `add-browser-capture`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Secure inbound email address translating mail to Inbox tasks.

## ADDED Requirements

### Requirement: Private inbound address

Each enabled account SHALL receive a high-entropy inbound email alias/token that can be rotated or disabled.

#### Scenario: Private inbound address — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** mail is delivered to a valid active alias
- **THEN** the message is associated with exactly one account

#### Scenario: Private inbound address — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Mail-to-task mapping

Inbound mail SHALL map subject to title and supported body text to notes using deterministic normalization.

#### Scenario: Mail-to-task mapping — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a valid email is accepted
- **THEN** one Inbox to-do is created with the expected content

#### Scenario: Mail-to-task mapping — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Abuse and duplicate protection

Email ingress SHALL enforce size/rate limits and message-id/idempotency handling.

#### Scenario: Abuse and duplicate protection — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the same mail is retried by the provider
- **THEN** duplicate Inbox tasks are not created

#### Scenario: Abuse and duplicate protection — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Secure inbound email address translating mail to Inbox tasks** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Private inbound address`
- `Mail-to-task mapping`
- `Abuse and duplicate protection`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Private inbound address` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Mail-to-task mapping` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Abuse and duplicate protection` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Private inbound address`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Mail-to-task mapping`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Abuse and duplicate protection`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-mail-to-inbox` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-mail-to-inbox --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-mail-to-inbox --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 56 — `add-command-urls`

**Required completed outcome:** Web equivalents for add/update/show/search automation URLs

## Proposal blueprint

## Why

This change delivers **Web equivalents for add/update/show/search automation URLs** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Command URL surface: The app SHALL expose documented URL actions for add, add-project, update, show, and search equivalents.
- Safe parameter parsing: Command URL parameters SHALL use strict typed parsing and reject unauthorized entity references or unsupported values.
- Shareable navigation URLs: Non-mutating show/search URLs SHALL remain bookmarkable and stable across sessions.

## Dependencies

- `add-mail-to-inbox`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Web equivalents for add/update/show/search automation URLs.

## ADDED Requirements

### Requirement: Command URL surface

The app SHALL expose documented URL actions for add, add-project, update, show, and search equivalents.

#### Scenario: Command URL surface — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a valid command URL is opened
- **THEN** the intended route/command is executed after authentication/validation

#### Scenario: Command URL surface — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Safe parameter parsing

Command URL parameters SHALL use strict typed parsing and reject unauthorized entity references or unsupported values.

#### Scenario: Safe parameter parsing — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a malformed or cross-account URL is opened
- **THEN** the operation fails safely without partial mutation

#### Scenario: Safe parameter parsing — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Shareable navigation URLs

Non-mutating show/search URLs SHALL remain bookmarkable and stable across sessions.

#### Scenario: Shareable navigation URLs — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user reopens a saved navigation URL
- **THEN** the same logical destination/query is restored

#### Scenario: Shareable navigation URLs — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Web equivalents for add/update/show/search automation URLs** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Command URL surface`
- `Safe parameter parsing`
- `Shareable navigation URLs`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Command URL surface` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Safe parameter parsing` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Shareable navigation URLs` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Command URL surface`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Safe parameter parsing`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Shareable navigation URLs`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-command-urls` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-command-urls --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-command-urls --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 57 — `add-automation-api`

**Required completed outcome:** Scoped personal API tokens and REST automation over application commands

## Proposal blueprint

## Why

This change delivers **Scoped personal API tokens and REST automation over application commands** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Scoped API tokens: Users SHALL create, view-once, scope, revoke, and rotate personal automation API tokens.
- Command-layer API: REST automation mutations SHALL delegate to the same application commands as the first-party UI.
- Stable API contract: The API SHALL use versioned routes and typed error responses suitable for automation.

## Dependencies

- `add-command-urls`
- `add-account-authentication`
- `add-command-transaction-layer`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Scoped personal API tokens and REST automation over application commands.

## ADDED Requirements

### Requirement: Scoped API tokens

Users SHALL create, view-once, scope, revoke, and rotate personal automation API tokens.

#### Scenario: Scoped API tokens — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a scoped token calls an endpoint
- **THEN** only authorized operations are allowed

#### Scenario: Scoped API tokens — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Command-layer API

REST automation mutations SHALL delegate to the same application commands as the first-party UI.

#### Scenario: Command-layer API — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** an API client moves/completes/creates an item
- **THEN** domain validation and sync change-log behavior match the UI

#### Scenario: Command-layer API — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Stable API contract

The API SHALL use versioned routes and typed error responses suitable for automation.

#### Scenario: Stable API contract — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a client receives an error
- **THEN** it can distinguish authentication, validation, conflict, and not-found outcomes

#### Scenario: Stable API contract — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Scoped personal API tokens and REST automation over application commands** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Scoped API tokens`
- `Command-layer API`
- `Stable API contract`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Scoped API tokens` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Command-layer API` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Stable API contract` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Scoped API tokens`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Command-layer API`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Stable API contract`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-automation-api` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-automation-api --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-automation-api --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 58 — `add-pwa`

**Required completed outcome:** Installability, manifest, offline application shell, shortcuts and update behavior

## Proposal blueprint

## Why

This change delivers **Installability, manifest, offline application shell, shortcuts and update behavior** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- PWA installability: The web application SHALL provide a valid manifest and installable shell where browser support exists.
- Offline shell: Previously loaded core application assets and offline route handling SHALL allow the app to start without network access.
- Update lifecycle: New deployments SHALL update service-worker/app assets safely without trapping a running client on incompatible code/data.

## Dependencies

- `add-automation-api`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Installability, manifest, offline application shell, shortcuts and update behavior.

## ADDED Requirements

### Requirement: PWA installability

The web application SHALL provide a valid manifest and installable shell where browser support exists.

#### Scenario: PWA installability — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a supported browser evaluates installability
- **THEN** the app can be installed with project-owned icons/name

#### Scenario: PWA installability — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Offline shell

Previously loaded core application assets and offline route handling SHALL allow the app to start without network access.

#### Scenario: Offline shell — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the installed app launches offline
- **THEN** the shell and local replica load rather than a browser network error page

#### Scenario: Offline shell — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Update lifecycle

New deployments SHALL update service-worker/app assets safely without trapping a running client on incompatible code/data.

#### Scenario: Update lifecycle — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** an updated app version becomes available
- **THEN** the user/client transitions according to an explicit refresh/update policy

#### Scenario: Update lifecycle — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Installability, manifest, offline application shell, shortcuts and update behavior** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `PWA installability`
- `Offline shell`
- `Update lifecycle`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `PWA installability` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Offline shell` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Update lifecycle` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `PWA installability`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Offline shell`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Update lifecycle`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-pwa` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-pwa --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-pwa --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 59 — `add-web-companion-views`

**Required completed outcome:** Compact Today/progress/quick-add views equivalent to native companion surfaces

## Proposal blueprint

## Why

This change delivers **Compact Today/progress/quick-add views equivalent to native companion surfaces** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Compact Today view: The web app SHALL expose a compact Today presentation suitable for small surfaces while preserving canonical completion actions.
- Progress companion: A compact progress presentation SHALL show derived Today completion state using the same progress calculation as the main app.
- Quick add companion: A compact quick-add surface SHALL create Inbox/current-context items through the normal command layer.

## Dependencies

- `add-pwa`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Compact Today/progress/quick-add views equivalent to native companion surfaces.

## ADDED Requirements

### Requirement: Compact Today view

The web app SHALL expose a compact Today presentation suitable for small surfaces while preserving canonical completion actions.

#### Scenario: Compact Today view — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the compact Today route is opened
- **THEN** the user sees a concise Today list and can complete supported items

#### Scenario: Compact Today view — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Progress companion

A compact progress presentation SHALL show derived Today completion state using the same progress calculation as the main app.

#### Scenario: Progress companion — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** Today state changes
- **THEN** the companion progress updates consistently

#### Scenario: Progress companion — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Quick add companion

A compact quick-add surface SHALL create Inbox/current-context items through the normal command layer.

#### Scenario: Quick add companion — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user adds from the companion view
- **THEN** the created item appears in the main app after sync

#### Scenario: Quick add companion — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Compact Today/progress/quick-add views equivalent to native companion surfaces** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Compact Today view`
- `Progress companion`
- `Quick add companion`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Compact Today view` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Progress companion` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Quick add companion` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Compact Today view`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Progress companion`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Quick add companion`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-web-companion-views` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-web-companion-views --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-web-companion-views --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 60 — `add-multiple-window-support`

**Required completed outcome:** Popout list/project/area windows sharing synchronized local state

## Proposal blueprint

## Why

This change delivers **Popout list/project/area windows sharing synchronized local state** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Popout destinations: Lists, projects, and areas SHALL be openable in separate browser windows where browser policy permits.
- Shared state: Multiple app windows SHALL share synchronized local/server state via the same multi-tab coordination mechanisms.
- Window-safe commands: Commands in a popout SHALL not assume ownership of singleton UI state from the original window.

## Dependencies

- `add-web-companion-views`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Popout list/project/area windows sharing synchronized local state.

## ADDED Requirements

### Requirement: Popout destinations

Lists, projects, and areas SHALL be openable in separate browser windows where browser policy permits.

#### Scenario: Popout destinations — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user opens a supported destination in a new window
- **THEN** the window loads the same canonical route/data

#### Scenario: Popout destinations — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Shared state

Multiple app windows SHALL share synchronized local/server state via the same multi-tab coordination mechanisms.

#### Scenario: Shared state — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** an item changes in one window
- **THEN** other windows converge without manual reload

#### Scenario: Shared state — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Window-safe commands

Commands in a popout SHALL not assume ownership of singleton UI state from the original window.

#### Scenario: Window-safe commands — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a popout performs an edit
- **THEN** the command completes independently and other windows update

#### Scenario: Window-safe commands — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Popout list/project/area windows sharing synchronized local state** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Popout destinations`
- `Shared state`
- `Window-safe commands`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Popout destinations` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Shared state` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Window-safe commands` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Popout destinations`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Shared state`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Window-safe commands`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-multiple-window-support` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-multiple-window-support --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-multiple-window-support --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 61 — `add-import-export`

**Required completed outcome:** Versioned JSON backups plus CSV/Markdown/import adapters

## Proposal blueprint

## Why

This change delivers **Versioned JSON backups plus CSV/Markdown/import adapters** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Versioned backup export: The system SHALL export a versioned, documented JSON backup independent of raw database table layout.
- Human-readable export: Supported lists/projects/items SHALL export to CSV/Markdown formats with deterministic field mapping.
- Import validation: Imports SHALL validate schema/content, report unsupported records, and avoid partial destructive replacement unless explicitly chosen.

## Dependencies

- `add-multiple-window-support`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Versioned JSON backups plus CSV/Markdown/import adapters.

## ADDED Requirements

### Requirement: Versioned backup export

The system SHALL export a versioned, documented JSON backup independent of raw database table layout.

#### Scenario: Versioned backup export — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user exports account data
- **THEN** the archive can be validated and imported by a compatible version

#### Scenario: Versioned backup export — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Human-readable export

Supported lists/projects/items SHALL export to CSV/Markdown formats with deterministic field mapping.

#### Scenario: Human-readable export — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a human-readable export is requested
- **THEN** the produced file preserves essential task content and hierarchy as documented

#### Scenario: Human-readable export — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Import validation

Imports SHALL validate schema/content, report unsupported records, and avoid partial destructive replacement unless explicitly chosen.

#### Scenario: Import validation — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** an invalid or partially supported import is submitted
- **THEN** the user receives a deterministic report and existing data remains safe

#### Scenario: Import validation — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Versioned JSON backups plus CSV/Markdown/import adapters** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Versioned backup export`
- `Human-readable export`
- `Import validation`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Versioned backup export` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Human-readable export` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Import validation` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Versioned backup export`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Human-readable export`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Import validation`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-import-export` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-import-export --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-import-export --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 62 — `add-sharing-and-printing`

**Required completed outcome:** Deterministic share and print render models

## Proposal blueprint

## Why

This change delivers **Deterministic share and print render models** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Share render models: The application SHALL generate deterministic share-friendly representations of to-dos, projects, and lists separate from interactive UI markup.
- Print layouts: Printable lists/projects SHALL use dedicated print layout rules rather than browser-printing the interactive shell unchanged.
- Content safety: Shared/printed rendering SHALL escape untrusted text and respect visibility/privacy rules.

## Dependencies

- `add-import-export`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Deterministic share and print render models.

## ADDED Requirements

### Requirement: Share render models

The application SHALL generate deterministic share-friendly representations of to-dos, projects, and lists separate from interactive UI markup.

#### Scenario: Share render models — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user shares an entity
- **THEN** the representation includes intended content without private/internal metadata

#### Scenario: Share render models — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Print layouts

Printable lists/projects SHALL use dedicated print layout rules rather than browser-printing the interactive shell unchanged.

#### Scenario: Print layouts — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a print action is invoked
- **THEN** navigation controls and non-print affordances are omitted

#### Scenario: Print layouts — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Content safety

Shared/printed rendering SHALL escape untrusted text and respect visibility/privacy rules.

#### Scenario: Content safety — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** notes contain markup-like text
- **THEN** output renders the text safely

#### Scenario: Content safety — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Deterministic share and print render models** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Share render models`
- `Print layouts`
- `Content safety`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Share render models` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Print layouts` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Content safety` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Share render models`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Print layouts`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Content safety`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-sharing-and-printing` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-sharing-and-printing --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-sharing-and-printing --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 63 — `add-settings`

**Required completed outcome:** General, appearance, Today, calendar, notifications, account, sync and data settings

## Proposal blueprint

## Why

This change delivers **General, appearance, Today, calendar, notifications, account, sync and data settings** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Settings domains: The app SHALL provide settings for general behavior, appearance, Today, calendar, notifications, Quick Entry/keyboard, Logbook, account/sync/integrations, and data controls as capabilities become available.
- Preference scope: Each preference SHALL declare whether it is account-synced, device-local, or browser-local.
- Validation/defaults: Settings SHALL have typed defaults and validation so unknown/invalid persisted values cannot break app startup.

## Dependencies

- `add-sharing-and-printing`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

General, appearance, Today, calendar, notifications, account, sync and data settings.

## ADDED Requirements

### Requirement: Settings domains

The app SHALL provide settings for general behavior, appearance, Today, calendar, notifications, Quick Entry/keyboard, Logbook, account/sync/integrations, and data controls as capabilities become available.

#### Scenario: Settings domains — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a settings route is opened
- **THEN** available preferences are grouped and editable in stable categories

#### Scenario: Settings domains — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Preference scope

Each preference SHALL declare whether it is account-synced, device-local, or browser-local.

#### Scenario: Preference scope — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a preference is changed
- **THEN** it persists and synchronizes only according to its declared scope

#### Scenario: Preference scope — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Validation/defaults

Settings SHALL have typed defaults and validation so unknown/invalid persisted values cannot break app startup.

#### Scenario: Validation/defaults — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** stored settings are missing or invalid
- **THEN** safe defaults are applied and the app remains usable

#### Scenario: Validation/defaults — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **General, appearance, Today, calendar, notifications, account, sync and data settings** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Settings domains`
- `Preference scope`
- `Validation/defaults`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Settings domains` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Preference scope` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Validation/defaults` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Settings domains`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Preference scope`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Validation/defaults`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-settings` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-settings --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-settings --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 64 — `add-theme-and-text-sizing`

**Required completed outcome:** Light/dark/system themes and accessibility-aware text sizing

## Proposal blueprint

## Why

This change delivers **Light/dark/system themes and accessibility-aware text sizing** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Theme choice: Users SHALL choose Light, Dark, or System theme, with System following browser/OS preference.
- Text size: Users SHALL adjust application text size within supported bounds without breaking layout or accessibility.
- Contrast/reduced motion compatibility: Theme/text settings SHALL remain compatible with accessibility contrast and reduced-motion behavior.

## Dependencies

- `add-settings`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Light/dark/system themes and accessibility-aware text sizing.

## ADDED Requirements

### Requirement: Theme choice

Users SHALL choose Light, Dark, or System theme, with System following browser/OS preference.

#### Scenario: Theme choice — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a theme selection changes
- **THEN** the app updates immediately and persists according to preference scope

#### Scenario: Theme choice — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Text size

Users SHALL adjust application text size within supported bounds without breaking layout or accessibility.

#### Scenario: Text size — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** text size is increased
- **THEN** rows/editors adapt without clipping essential controls

#### Scenario: Text size — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Contrast/reduced motion compatibility

Theme/text settings SHALL remain compatible with accessibility contrast and reduced-motion behavior.

#### Scenario: Contrast/reduced motion compatibility — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** accessibility preferences are active
- **THEN** the visual system respects them

#### Scenario: Contrast/reduced motion compatibility — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Light/dark/system themes and accessibility-aware text sizing** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Theme choice`
- `Text size`
- `Contrast/reduced motion compatibility`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Theme choice` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Text size` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Contrast/reduced motion compatibility` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Theme choice`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Text size`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Contrast/reduced motion compatibility`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-theme-and-text-sizing` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-theme-and-text-sizing --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-theme-and-text-sizing --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 65 — `add-localization`

**Required completed outcome:** Locale, week start, hour format, timezone and parser-locale infrastructure

## Proposal blueprint

## Why

This change delivers **Locale, week start, hour format, timezone and parser-locale infrastructure** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Locale settings: The system SHALL separate UI language/locale, date formatting locale, first day of week, 12/24-hour format, and timezone configuration.
- Timezone-correct dates: Date-only scheduling/deadline values SHALL remain date-only while reminders/instants SHALL resolve with timezone/DST rules.
- Parser locale integration: Natural-language date parsing SHALL select the configured parser locale independently from storage representation.

## Dependencies

- `add-theme-and-text-sizing`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Locale, week start, hour format, timezone and parser-locale infrastructure.

## ADDED Requirements

### Requirement: Locale settings

The system SHALL separate UI language/locale, date formatting locale, first day of week, 12/24-hour format, and timezone configuration.

#### Scenario: Locale settings — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a locale-related preference changes
- **THEN** date/time/UI formatting updates in the relevant scope only

#### Scenario: Locale settings — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Timezone-correct dates

Date-only scheduling/deadline values SHALL remain date-only while reminders/instants SHALL resolve with timezone/DST rules.

#### Scenario: Timezone-correct dates — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the user changes timezone
- **THEN** date-only task intent is preserved and instant-based reminders are recalculated/displayed correctly

#### Scenario: Timezone-correct dates — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Parser locale integration

Natural-language date parsing SHALL select the configured parser locale independently from storage representation.

#### Scenario: Parser locale integration — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the UI locale/parser locale changes
- **THEN** parsing behavior changes without rewriting stored dates

#### Scenario: Parser locale integration — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Locale, week start, hour format, timezone and parser-locale infrastructure** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Locale settings`
- `Timezone-correct dates`
- `Parser locale integration`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Locale settings` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Timezone-correct dates` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Parser locale integration` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Locale settings`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Timezone-correct dates`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Parser locale integration`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-localization` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-localization --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-localization --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 66 — `add-accessibility`

**Required completed outcome:** Complete keyboard access, semantics, screen-reader behavior, focus and reduced motion

## Proposal blueprint

## Why

This change delivers **Complete keyboard access, semantics, screen-reader behavior, focus and reduced motion** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Keyboard-only operation: All core workflows SHALL be operable by keyboard with visible focus and logical focus order.
- Semantic accessibility: Interactive controls, lists, dialogs, states, and progress indicators SHALL expose appropriate accessible names/roles/states.
- Motion/target/contrast requirements: The UI SHALL respect reduced motion, minimum touch target guidance, and defined contrast requirements.

## Dependencies

- `add-localization`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Complete keyboard access, semantics, screen-reader behavior, focus and reduced motion.

## ADDED Requirements

### Requirement: Keyboard-only operation

All core workflows SHALL be operable by keyboard with visible focus and logical focus order.

#### Scenario: Keyboard-only operation — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a user avoids pointer input
- **THEN** navigation, creation, editing, moving, completing, and dialogs remain reachable

#### Scenario: Keyboard-only operation — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Semantic accessibility

Interactive controls, lists, dialogs, states, and progress indicators SHALL expose appropriate accessible names/roles/states.

#### Scenario: Semantic accessibility — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** assistive technology inspects a screen
- **THEN** the UI conveys equivalent structure and state

#### Scenario: Semantic accessibility — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Motion/target/contrast requirements

The UI SHALL respect reduced motion, minimum touch target guidance, and defined contrast requirements.

#### Scenario: Motion/target/contrast requirements — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** accessibility preferences or mobile input are used
- **THEN** the interface remains perceivable and operable

#### Scenario: Motion/target/contrast requirements — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Complete keyboard access, semantics, screen-reader behavior, focus and reduced motion** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Keyboard-only operation`
- `Semantic accessibility`
- `Motion/target/contrast requirements`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Keyboard-only operation` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Semantic accessibility` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Motion/target/contrast requirements` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Keyboard-only operation`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Semantic accessibility`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Motion/target/contrast requirements`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-accessibility` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-accessibility --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-accessibility --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 67 — `add-animation-system`

**Required completed outcome:** Centralized insertion/removal/editor/popover/drag/navigation motion primitives

## Proposal blueprint

## Why

This change delivers **Centralized insertion/removal/editor/popover/drag/navigation motion primitives** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Motion primitives: Insertion, removal/completion, editor expansion, popovers, drag lift/drop, and navigation transitions SHALL use centralized motion primitives.
- State-preserving animation: Animations SHALL not alter domain state ordering or block command completion if interrupted.
- Reduced motion: Nonessential transitions SHALL be removed or shortened when reduced-motion preference is active.

## Dependencies

- `add-accessibility`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Centralized insertion/removal/editor/popover/drag/navigation motion primitives.

## ADDED Requirements

### Requirement: Motion primitives

Insertion, removal/completion, editor expansion, popovers, drag lift/drop, and navigation transitions SHALL use centralized motion primitives.

#### Scenario: Motion primitives — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a supported transition occurs
- **THEN** duration/easing/geometry comes from shared motion tokens

#### Scenario: Motion primitives — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: State-preserving animation

Animations SHALL not alter domain state ordering or block command completion if interrupted.

#### Scenario: State-preserving animation — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** an animation is skipped/interrupted
- **THEN** the final UI/domain state remains correct

#### Scenario: State-preserving animation — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Reduced motion

Nonessential transitions SHALL be removed or shortened when reduced-motion preference is active.

#### Scenario: Reduced motion — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the browser reports reduced motion
- **THEN** the app avoids unnecessary motion while preserving spatial clarity

#### Scenario: Reduced motion — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Centralized insertion/removal/editor/popover/drag/navigation motion primitives** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Motion primitives`
- `State-preserving animation`
- `Reduced motion`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Motion primitives` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `State-preserving animation` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Reduced motion` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Motion primitives`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `State-preserving animation`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Reduced motion`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-animation-system` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-animation-system --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-animation-system --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 68 — `add-large-list-performance`

**Required completed outcome:** Benchmarks and optimizations for thousands to tens of thousands of items

## Proposal blueprint

## Why

This change delivers **Benchmarks and optimizations for thousands to tens of thousands of items** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Performance budgets: The project SHALL define measurable budgets for initial render, scroll responsiveness, search, reorder, sync apply, and local-replica startup across large datasets.
- List virtualization: Large projections SHALL virtualize rendering where needed without breaking keyboard navigation, selection, or drag/reorder semantics.
- Query/index tuning: Server and local queries SHALL use indexes/batching appropriate for large accounts.

## Dependencies

- `add-offline-local-replica`
- `add-deep-search`
- `add-drag-and-drop`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Benchmarks and optimizations for thousands to tens of thousands of items.

## ADDED Requirements

### Requirement: Performance budgets

The project SHALL define measurable budgets for initial render, scroll responsiveness, search, reorder, sync apply, and local-replica startup across large datasets.

#### Scenario: Performance budgets — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** benchmark fixtures of 1k/5k/10k+ items run
- **THEN** results are recorded and budget regressions fail the benchmark gate where configured

#### Scenario: Performance budgets — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: List virtualization

Large projections SHALL virtualize rendering where needed without breaking keyboard navigation, selection, or drag/reorder semantics.

#### Scenario: List virtualization — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a list exceeds the virtualization threshold
- **THEN** visible rendering stays responsive and interaction state remains correct

#### Scenario: List virtualization — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Query/index tuning

Server and local queries SHALL use indexes/batching appropriate for large accounts.

#### Scenario: Query/index tuning — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** large-account performance tests execute
- **THEN** database/local queries avoid obvious N+1 and full-scan regressions

#### Scenario: Query/index tuning — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Benchmarks and optimizations for thousands to tens of thousands of items** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Performance budgets`
- `List virtualization`
- `Query/index tuning`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Performance budgets` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `List virtualization` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Query/index tuning` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Performance budgets`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `List virtualization`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Query/index tuning`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-large-list-performance` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-large-list-performance --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-large-list-performance --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 69 — `add-security-hardening`

**Required completed outcome:** Argon2id, secure sessions, CSRF/CSP, rate limiting, isolation and API authorization

## Proposal blueprint

## Why

This change delivers **Argon2id, secure sessions, CSRF/CSP, rate limiting, isolation and API authorization** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Session/web hardening: The deployed web app SHALL enforce secure cookies, CSRF defenses, CSP, secure headers, and authentication throttling appropriate to its architecture.
- Authorization isolation tests: Every user-owned API/query family SHALL include cross-account isolation tests.
- Secret/input/rate safety: Secrets SHALL remain server-side, inputs SHALL be bounded/validated, and abuse-prone endpoints SHALL enforce rate/resource limits.

## Dependencies

- `add-large-list-performance`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Argon2id, secure sessions, CSRF/CSP, rate limiting, isolation and API authorization.

## ADDED Requirements

### Requirement: Session/web hardening

The deployed web app SHALL enforce secure cookies, CSRF defenses, CSP, secure headers, and authentication throttling appropriate to its architecture.

#### Scenario: Session/web hardening — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a browser session performs authenticated mutations
- **THEN** requests are protected from common cross-site/session attacks

#### Scenario: Session/web hardening — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Authorization isolation tests

Every user-owned API/query family SHALL include cross-account isolation tests.

#### Scenario: Authorization isolation tests — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a foreign identifier is supplied
- **THEN** the request cannot read or mutate another account's data

#### Scenario: Authorization isolation tests — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Secret/input/rate safety

Secrets SHALL remain server-side, inputs SHALL be bounded/validated, and abuse-prone endpoints SHALL enforce rate/resource limits.

#### Scenario: Secret/input/rate safety — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** an endpoint receives oversized/repeated/malformed input
- **THEN** the service rejects or limits it predictably

#### Scenario: Secret/input/rate safety — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Argon2id, secure sessions, CSRF/CSP, rate limiting, isolation and API authorization** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Session/web hardening`
- `Authorization isolation tests`
- `Secret/input/rate safety`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Session/web hardening` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Authorization isolation tests` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Secret/input/rate safety` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Session/web hardening`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Authorization isolation tests`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Secret/input/rate safety`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-security-hardening` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-security-hardening --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-security-hardening --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 70 — `add-observability`

**Required completed outcome:** Structured tracing, request/sync/scheduler/integration diagnostics and privacy-safe metrics

## Proposal blueprint

## Why

This change delivers **Structured tracing, request/sync/scheduler/integration diagnostics and privacy-safe metrics** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Structured tracing: Server, worker, sync, scheduler, and integration paths SHALL emit structured traces with correlation/request identifiers.
- Operational metrics: The system SHALL expose privacy-safe health/performance counters for requests, sync, reminders, recurrence, database pool, and external integrations.
- Diagnostic privacy: Logs/metrics SHALL avoid task titles, notes, email bodies, tokens, and secrets unless explicitly redacted/test-safe.

## Dependencies

- `add-security-hardening`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Structured tracing, request/sync/scheduler/integration diagnostics and privacy-safe metrics.

## ADDED Requirements

### Requirement: Structured tracing

Server, worker, sync, scheduler, and integration paths SHALL emit structured traces with correlation/request identifiers.

#### Scenario: Structured tracing — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a request or background job is processed
- **THEN** operators can follow its execution without logging private task content by default

#### Scenario: Structured tracing — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Operational metrics

The system SHALL expose privacy-safe health/performance counters for requests, sync, reminders, recurrence, database pool, and external integrations.

#### Scenario: Operational metrics — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the service is monitored
- **THEN** failures/backlogs/latency can be detected without inspecting user data

#### Scenario: Operational metrics — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Diagnostic privacy

Logs/metrics SHALL avoid task titles, notes, email bodies, tokens, and secrets unless explicitly redacted/test-safe.

#### Scenario: Diagnostic privacy — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** an error involves user content
- **THEN** diagnostics contain identifiers/metadata sufficient for debugging without raw private content

#### Scenario: Diagnostic privacy — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Structured tracing, request/sync/scheduler/integration diagnostics and privacy-safe metrics** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Structured tracing`
- `Operational metrics`
- `Diagnostic privacy`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Structured tracing` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Operational metrics` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Diagnostic privacy` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Structured tracing`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Operational metrics`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Diagnostic privacy`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-observability` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-observability --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-observability --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 71 — `add-visual-parity-suite`

**Required completed outcome:** Fixed-viewport screenshot baselines and Rust-driven visual regression checks

## Proposal blueprint

## Why

This change delivers **Fixed-viewport screenshot baselines and Rust-driven visual regression checks** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Fixed visual baselines: The test suite SHALL capture approved visual baselines for representative desktop/tablet/mobile states at fixed viewports.
- Deterministic screenshot fixtures: Visual tests SHALL use deterministic seeded data, fonts/assets, time, theme, and animation suppression.
- Diff reporting: Visual regression failures SHALL produce inspectable expected/actual/diff artifacts and configurable thresholds.

## Dependencies

- `add-animation-system`
- `add-theme-and-text-sizing`
- `add-responsive-layout-foundation`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Fixed-viewport screenshot baselines and Rust-driven visual regression checks.

## ADDED Requirements

### Requirement: Fixed visual baselines

The test suite SHALL capture approved visual baselines for representative desktop/tablet/mobile states at fixed viewports.

#### Scenario: Fixed visual baselines — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a parity snapshot test runs
- **THEN** the rendered state is compared to the project baseline

#### Scenario: Fixed visual baselines — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Deterministic screenshot fixtures

Visual tests SHALL use deterministic seeded data, fonts/assets, time, theme, and animation suppression.

#### Scenario: Deterministic screenshot fixtures — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the same commit is tested repeatedly
- **THEN** screenshots are stable enough for meaningful diffing

#### Scenario: Deterministic screenshot fixtures — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Diff reporting

Visual regression failures SHALL produce inspectable expected/actual/diff artifacts and configurable thresholds.

#### Scenario: Diff reporting — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a snapshot differs beyond threshold
- **THEN** the CI/agent report identifies the affected view and diff artifact

#### Scenario: Diff reporting — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Fixed-viewport screenshot baselines and Rust-driven visual regression checks** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Fixed visual baselines`
- `Deterministic screenshot fixtures`
- `Diff reporting`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Fixed visual baselines` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Deterministic screenshot fixtures` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Diff reporting` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Fixed visual baselines`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Deterministic screenshot fixtures`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Diff reporting`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-visual-parity-suite` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-visual-parity-suite --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-visual-parity-suite --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 72 — `add-behavior-parity-suite`

**Required completed outcome:** End-to-end GIVEN/WHEN/THEN parity scenarios across all major workflows

## Proposal blueprint

## Why

This change delivers **End-to-end GIVEN/WHEN/THEN parity scenarios across all major workflows** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Behavior parity scenarios: The suite SHALL encode end-to-end GIVEN/WHEN/THEN scenarios for core capture, scheduling, projections, recurrence, search, move, sync, offline, and interaction behaviors.
- Cross-view identity assertions: Parity tests SHALL verify that projection changes do not duplicate/corrupt canonical entity/context/order state.
- Offline/multi-device assertions: Behavior tests SHALL cover offline edits, reconnect, conflict handling, and multi-tab/device convergence.

## Dependencies

- `add-visual-parity-suite`
- `add-cloud-sync-protocol`
- `add-recurrence-exceptions`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

End-to-end GIVEN/WHEN/THEN parity scenarios across all major workflows.

## ADDED Requirements

### Requirement: Behavior parity scenarios

The suite SHALL encode end-to-end GIVEN/WHEN/THEN scenarios for core capture, scheduling, projections, recurrence, search, move, sync, offline, and interaction behaviors.

#### Scenario: Behavior parity scenarios — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the parity suite runs
- **THEN** documented user workflows execute against the full application

#### Scenario: Behavior parity scenarios — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Cross-view identity assertions

Parity tests SHALL verify that projection changes do not duplicate/corrupt canonical entity/context/order state.

#### Scenario: Cross-view identity assertions — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a task moves among Today/Upcoming/Anytime/Someday
- **THEN** the same canonical entity and unaffected contextual ordering are preserved

#### Scenario: Cross-view identity assertions — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Offline/multi-device assertions

Behavior tests SHALL cover offline edits, reconnect, conflict handling, and multi-tab/device convergence.

#### Scenario: Offline/multi-device assertions — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** two clients perform the defined concurrent scenario
- **THEN** they converge according to sync policy

#### Scenario: Offline/multi-device assertions — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **End-to-end GIVEN/WHEN/THEN parity scenarios across all major workflows** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Behavior parity scenarios`
- `Cross-view identity assertions`
- `Offline/multi-device assertions`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Behavior parity scenarios` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Cross-view identity assertions` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Offline/multi-device assertions` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Behavior parity scenarios`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Cross-view identity assertions`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Offline/multi-device assertions`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-behavior-parity-suite` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-behavior-parity-suite --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-behavior-parity-suite --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 73 — `add-production-deployment`

**Required completed outcome:** Reproducible deployment, health/readiness, backups, migrations and worker topology

## Proposal blueprint

## Why

This change delivers **Reproducible deployment, health/readiness, backups, migrations and worker topology** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Reproducible deployment: The project SHALL define reproducible production artifacts/topology for web server, background worker, database connectivity, static/WASM assets, and required ingress integrations.
- Safe migrations and health: Deployment SHALL provide readiness/liveness checks and a migration procedure that avoids serving incompatible schema/code states.
- Backup/recovery: Production operations SHALL define database backup, retention, and restore verification procedures.

## Dependencies

- `add-security-hardening`
- `add-observability`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Reproducible deployment, health/readiness, backups, migrations and worker topology.

## ADDED Requirements

### Requirement: Reproducible deployment

The project SHALL define reproducible production artifacts/topology for web server, background worker, database connectivity, static/WASM assets, and required ingress integrations.

#### Scenario: Reproducible deployment — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a clean deployment is built
- **THEN** the documented components start with explicit configuration

#### Scenario: Reproducible deployment — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Safe migrations and health

Deployment SHALL provide readiness/liveness checks and a migration procedure that avoids serving incompatible schema/code states.

#### Scenario: Safe migrations and health — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a new version is rolled out
- **THEN** migrations and application readiness occur in the documented safe order

#### Scenario: Safe migrations and health — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Backup/recovery

Production operations SHALL define database backup, retention, and restore verification procedures.

#### Scenario: Backup/recovery — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** a backup job runs and a restore drill is performed
- **THEN** recoverability is demonstrably tested

#### Scenario: Backup/recovery — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Reproducible deployment, health/readiness, backups, migrations and worker topology** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Reproducible deployment`
- `Safe migrations and health`
- `Backup/recovery`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Reproducible deployment` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Safe migrations and health` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Backup/recovery` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Reproducible deployment`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Safe migrations and health`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Backup/recovery`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify add-production-deployment` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate add-production-deployment --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive add-production-deployment --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# 74 — `things-web-parity-hardening`

**Required completed outcome:** Full audit against parity matrix; eliminate remaining implementation/spec/test gaps

## Proposal blueprint

## Why

This change delivers **Full audit against parity matrix; eliminate remaining implementation/spec/test gaps** as the next independently verifiable layer in the Things-style full-stack Rust roadmap. It must be complete enough to archive without relying on a future proposal to make its own acceptance criteria true.

## Scope

- Parity matrix closure: The final hardening change SHALL audit every roadmap capability against the frozen parity matrix and classify unresolved items before closure.
- Zero known implementation gaps: The change SHALL resolve all known BUG/MISSING findings that are in scope for the web product before completion.
- Release-quality validation: The complete workspace SHALL pass strict OpenSpec validation, Rust quality gates, behavior parity, visual parity, accessibility/security checks, and production build/deployment verification.

## Dependencies

- `add-production-deployment`
- `add-behavior-parity-suite`
- `add-visual-parity-suite`

## Non-goals

- Do not implement later roadmap capabilities merely because they are convenient.
- Do not bypass the application/domain boundaries already established by earlier archived changes.
- Do not copy Cultured Code trademarks, logo, marketing art, or proprietary icon assets.
- Do not add handwritten JavaScript/TypeScript application logic.

## What Changes

- Add or modify the capability spec required for this change.
- Implement the Rust domain/application/infrastructure/UI work needed by the requirements.
- Add automated tests at the appropriate unit/integration/browser level.
- Update persistence/migrations, offline/sync behavior, accessibility, and observability only where this change requires them.
- Keep the repository passing strict OpenSpec validation and applicable Rust quality gates.

## Success Criteria

The change is ready to archive only when every requirement scenario in the delta spec is implemented and tested, every `tasks.md` checkbox is complete, implementation verification finds no critical mismatch, and post-archive canonical-spec validation passes.

## Delta spec blueprint

## Purpose

Full audit against parity matrix; eliminate remaining implementation/spec/test gaps.

## ADDED Requirements

### Requirement: Parity matrix closure

The final hardening change SHALL audit every roadmap capability against the frozen parity matrix and classify unresolved items before closure.

#### Scenario: Parity matrix closure — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the parity audit runs
- **THEN** every item has an implemented/tested status or an explicitly accepted web-equivalent difference

#### Scenario: Parity matrix closure — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Zero known implementation gaps

The change SHALL resolve all known BUG/MISSING findings that are in scope for the web product before completion.

#### Scenario: Zero known implementation gaps — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the final verification is performed
- **THEN** no known in-scope bug or missing roadmap requirement remains unaddressed

#### Scenario: Zero known implementation gaps — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

### Requirement: Release-quality validation

The complete workspace SHALL pass strict OpenSpec validation, Rust quality gates, behavior parity, visual parity, accessibility/security checks, and production build/deployment verification.

#### Scenario: Release-quality validation — primary behavior

- **GIVEN** the user/account/application is in a valid state for this capability
- **WHEN** the final release candidate is evaluated
- **THEN** all required release gates pass and the repository is clean

#### Scenario: Release-quality validation — invariant preservation

- **WHEN** the same behavior is exercised through retries, refreshes, or another supported adapter where applicable
- **THEN** the capability SHALL preserve canonical identity, authorization, and unrelated domain state
- **AND** failures SHALL be reported without partial silent corruption

## Design blueprint

## Context

This change implements **Full audit against parity matrix; eliminate remaining implementation/spec/test gaps** after all listed dependencies have been archived into the canonical specs. The design must fit the existing Rust workspace rather than creating a parallel architecture.

## Requirements Driving the Design

- `Parity matrix closure`
- `Zero known implementation gaps`
- `Release-quality validation`

## Architecture

- Put business invariants in domain/application crates.
- Put Leptos/browser interaction in web/UI crates.
- Put PostgreSQL/IndexedDB/provider-specific code behind infrastructure/adaptor interfaces.
- Route all authoritative mutations through typed application commands.
- Reuse existing entity IDs, ordering, sync, auth, and error types instead of inventing feature-local variants.
- Make browser-visible state derivable from canonical domain state and projection/query services.

## Data and Persistence

- Introduce only the minimum new schema required by this capability.
- Use Rust-managed migrations for persisted changes.
- Preserve stable IDs and optimistic versions.
- Define deletion/tombstone behavior for any new persistent entity.
- Define serialization/versioning when data crosses sync/API/offline boundaries.

## Offline and Sync

- If this capability mutates user data, define its optimistic local operation and server reconciliation behavior.
- New operations must be idempotent where retries are possible.
- Projection-only UI state must not be synchronized as canonical domain data.
- If this change precedes the full sync implementation, keep command/event shapes compatible with later outbox/change-log use.

## UI and Accessibility

- Reuse design-system primitives.
- Define keyboard and pointer behavior; define touch behavior when the surface is mobile-relevant.
- Maintain visible focus, accessible names/roles/states, and reduced-motion compatibility.
- Responsive behavior must be deliberate rather than accidental CSS wrapping.

## Error Handling

- Use typed errors for validation, authorization, conflict, not-found, provider, and persistence failures as applicable.
- Never expose secrets or foreign-account existence through errors.
- Avoid partial state transitions; use transactional boundaries where persistent multi-write changes occur.

## Testing Strategy

- Unit tests for pure Rust invariants and transformations.
- Persistence/migration tests for schema changes.
- Integration tests for application commands/queries.
- Browser/E2E tests for user-visible behavior once the harness exists.
- Property/concurrency tests when ordering, recurrence, sync, or idempotency is involved.
- Add visual snapshots once the visual parity harness exists.

## Rollout / Migration

The change must be safe to apply to an existing development database and safe to run in a mixed pre/post-reload browser state. Any incompatible data-format change requires an explicit migration/version strategy.

## Open Questions the Agent Must Resolve From the Repository

Before implementation, inspect the current canonical specs and code for naming, crate placement, existing traits, test harnesses, and migration conventions. Record any non-obvious resolution here rather than inventing a parallel pattern.

## Tasks / Definition of Done

## 1. Plan and validate
- [ ] 1.1 Read current canonical specs and all archived dependencies.
- [ ] 1.2 Run `openspec instructions proposal/specs/design/tasks` for this change and adapt these seeds to the actual schema instructions.
- [ ] 1.3 Confirm proposal scope and non-goals; remove any work that belongs to later roadmap changes.
- [ ] 1.4 Run strict non-interactive OpenSpec validation and fix all findings before apply.

## 2. Domain/application implementation
- [ ] 2.1 Implement `Parity matrix closure` in the appropriate Rust domain/application layer.
- [ ] 2.2 Implement `Zero known implementation gaps` in the appropriate Rust domain/application layer.
- [ ] 2.3 Implement `Release-quality validation` in the appropriate Rust domain/application layer.

## 3. Infrastructure and UI
- [ ] 3.1 Add only the required persistence/provider/browser adapters and Rust-managed migrations.
- [ ] 3.2 Integrate the capability with existing Leptos routes/components using shared design-system primitives.
- [ ] 3.3 Wire commands/queries/events through existing application boundaries; do not mutate storage directly from UI.
- [ ] 3.4 Handle offline/sync hooks, authorization, typed errors, and accessibility where applicable.

## 4. Tests and verification
- [ ] 4.1 Add automated coverage for the primary and invariant-preservation scenarios of `Parity matrix closure`.
- [ ] 4.2 Add automated coverage for the primary and invariant-preservation scenarios of `Zero known implementation gaps`.
- [ ] 4.3 Add automated coverage for the primary and invariant-preservation scenarios of `Release-quality validation`.
- [ ] 4.4 Run applicable Rust unit/integration/browser/migration tests and fix failures.
- [ ] 4.5 Run `cargo fmt --all -- --check`, Clippy, and workspace tests where configured.
- [ ] 4.6 Run `/opsx:verify things-web-parity-hardening` when available, otherwise perform an explicit artifact-to-implementation verification and record it.
- [ ] 4.7 Resolve all critical/actionable verification findings.

## 5. Archive and commit
- [ ] 5.1 Re-run `openspec validate things-web-parity-hardening --strict --no-interactive`.
- [ ] 5.2 Archive with `openspec archive things-web-parity-hardening --yes` without bypassing validation.
- [ ] 5.3 Run strict validation for canonical specs and archived-task completeness.
- [ ] 5.4 Review the full Git diff for accidental/unrelated changes.
- [ ] 5.5 Update `OVERNIGHT_REPORT.md` and create the single completion commit for this numbered change.


---

# PART 5 — OVERNIGHT REPORT TEMPLATE

# OVERNIGHT_REPORT

## Run summary

- Branch:
- Worktree:
- Start time:
- End time:
- Bootstrap status:
- Total implementation changes complete:
- Total blocked:
- Total skipped due to dependency:
- Total completion commits:
- HEAD SHA:
- OpenSpec live-spec strict validation:
- OpenSpec archived-task validation:
- `cargo fmt`:
- `cargo clippy`:
- Rust tests:
- Browser/E2E tests:
- Visual regression:
- Known follow-ups:
- Start app:
- Run tests:
- Review instructions:

## Baseline before implementation

- Existing repository state:
- Existing uncommitted user work:
- OpenSpec version:
- Rust toolchain:
- Baseline build:
- Baseline tests:
- Baseline OpenSpec validation:
- Existing failures not caused by overnight changes:

## Change log

Copy this section for every numbered change.

### 01 — bootstrap-fullstack-rust-workspace

- Status: `PENDING`
- Started:
- Finished:
- Proposal:
- Strict pre-apply validation:
- Apply:
- Tests:
- Verification:
- Repairs:
- Strict pre-archive validation:
- Archive:
- Live-spec validation:
- Archived-task validation:
- Commit SHA:
- Design decisions:
- Environmental/live checks deferred:
- Blocker details:
- Dependency impact:

## Final notes

- Remaining first uncompleted change:
- Blocked changes:
- Important architectural decisions:
- Data/schema migrations introduced:
- External integrations requiring real credentials:
- Security-sensitive follow-ups:


---

# PART 6 — MORNING QUALITY GATE SCRIPT

```bash
#!/usr/bin/env bash
set -euo pipefail

echo "== OpenSpec: all changes/specs =="
openspec validate --all --strict --no-interactive

echo "== OpenSpec: archived task completeness =="
openspec validate --archived --strict --no-interactive

if command -v cargo >/dev/null 2>&1; then
  echo "== cargo fmt =="
  cargo fmt --all -- --check

  echo "== cargo clippy =="
  cargo clippy --workspace --all-targets --all-features -- -D warnings

  echo "== cargo test =="
  cargo test --workspace --all-features

  if command -v cargo-nextest >/dev/null 2>&1; then
    echo "== cargo nextest =="
    cargo nextest run --workspace --all-features
  fi
else
  echo "cargo not found; skipping Rust quality gates" >&2
fi
```

---

# PART 7 — FINAL COMPLETION CONDITION

The autonomous run is finished only when one of these is true:

1. `74-things-web-parity-hardening` is implemented, verified, archived, committed,
   and every final release gate passes; or
2. the execution environment ends; or
3. a true blocker prevents dependent work and no independent later work remains.

At successful completion:

- `git status` is clean;
- the overnight branch contains a reviewable commit per completed numbered change;
- all completed changes are archived;
- canonical OpenSpec specs validate strictly;
- archived task completeness validates;
- applicable Rust/browser/visual/security/deployment tests pass;
- `OVERNIGHT_REPORT.md` identifies the exact HEAD SHA and any real environmental
  checks that could not be performed;
- no known in-scope BUG/MISSING item remains in the final parity audit.

**Do not stop to ask the user whether to continue.**
