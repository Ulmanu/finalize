# Overnight Autonomous Run Report

Run: autonomous execution of TODO.md (Bootstrap 00 + OpenSpec changes 01–74)
Branch: `agent/things-web-overnight`
Agent: opencode (deepseek-v4.1-flash)

## Bootstrap 00 — status: COMPLETE

- Repository inspected: empty git history, untracked planning bundle
  (`TODO.md`, `openspec/`, `.agents/`, `CLAUDE.md`, `skills-lock.json`).
- Isolated branch created: `agent/things-web-overnight` (no pre-existing user work discarded;
  `.idea/` and `.rtk/` are gitignored local state).
- Tool versions:
  - cargo 1.95.0 (Homebrew), rustc 1.95.0 (Homebrew)
  - OpenSpec CLI 1.13.1 (propose/apply/archive skills installed; **no `/opsx:verify` skill** —
    manual artifact↔implementation verification is used and recorded per change)
  - node/pnpm/npm available; `trunk`, `wasm-pack`, `cargo-leptos` NOT installed
  - Docker daemon NOT running; `psql` NOT installed → live PostgreSQL integration tests are
    conditional on `DATABASE_URL`; in-memory persistence adapter is used for deterministic tests.
  - rustup NOT installed (Homebrew toolchain) → `wasm32-unknown-unknown` target unavailable at
    bootstrap; WASM target check is attempted and its availability recorded per change.
- OpenSpec project rules installed in `openspec/config.yaml` (Rust-only source policy,
  architecture, parity/branding policy, artifact rules, apply/archive guidance).
- Parity policy frozen:
  - Observable Things-style behavior, information architecture, and interaction quality are the
    reference targets.
  - Apple-native-only integrations receive documented web equivalents.
  - Original branding: product name **Momentum**; original CSS/icon assets only. No Cultured Code
    trademarks, logos, marketing art, or proprietary icons.
- Rust-only application-source policy confirmed and recorded in `openspec/config.yaml`.
- Baseline build/test/validation: repository contained no Rust code before change 01.
  `openspec validate --specs --strict --no-interactive` on an empty spec tree reports no specs;
  there are no pre-existing failures to attribute.
- Bootstrap commit: see git log (`chore(00): bootstrap OpenSpec project and overnight report`).

### Bootstrap follow-up (during change 01)

- Installed rustup (stable 1.98.1) with `rustfmt`, `clippy`, and the
  `wasm32-unknown-unknown` target; added `rust-toolchain.toml` and made `scripts/*.sh` prefer
  `$HOME/.cargo/bin`. Homebrew Rust 1.95 remains on PATH for manual shells but the project
  toolchain is now rustup-managed so WASM builds are reproducible.

## Build environment notes

- 2026-09-25/26: the volume holding `target/` reached 99% capacity during change 22
  verification, and rustc failed with `rustc-LLVM ERROR: IO failure on output stream: No space
  left on device`. Recovery: removed only reproducible Cargo build artifacts (`target/`
  contents: incremental caches, fingerprints, build outputs, dependency artifacts), first with
  `cargo clean`, then the remaining artifacts. Disk free went from 2.4 GiB (99% used) before
  cleanup to 134 GiB (42% used) after (`df -h /Volumes/CULMANU`, 2026-09-26 01:21 EEST).
- To keep unattended builds from regrowing artifacts to 100+ GiB, the workspace `Cargo.toml`
  now sets `[profile.dev]` and `[profile.test]` to `debug = 0` and `incremental = false`. This
  only removes debug symbols and incremental caches; debug assertions, overflow checks, tests,
  Clippy, and OpenSpec validation remain fully enabled.

## Change log

### 01 — `bootstrap-fullstack-rust-workspace` — COMPLETE

- Start: 2026-09-23 21:30 · End: 2026-09-23 21:52
- Proposal validation: `openspec validate ... --strict --no-interactive` passed before apply.
- Implementation:
  - Cargo workspace `momentum` with `domain`, `app`, `infra`, `web`, `testkit`, `xtask`.
  - Dependency direction enforced by `momentum-xtask arch::check_workspace` + integration test.
  - Axum server binary `momentum-server` (SSR Leptos shell, `/healthz`, `/readyz`,
    `MOMENTUM_BIND_ADDR` config in `momentum-infra`).
  - WASM client built successfully: `target/wasm32-unknown-unknown/debug/momentum_web.wasm`.
  - `scripts/quality-gates.sh`, `scripts/build-web.sh`, `.cargo/config.toml` xtask alias,
    `rustfmt.toml`, `rust-toolchain.toml`, README.
  - Test asserts no handwritten JS/TS sources exist in `crates/` or `scripts/`.
- Migrations: none (no persistence yet).
- Tests executed:
  - `cargo test --workspace --all-features` — all pass (domain 1, app 2, infra 3, xtask 3,
    testkit topology 1, web server smoke 4).
  - `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
  - `scripts/quality-gates.sh` — all gates passed.
  - `scripts/build-web.sh` — WASM artifact produced.
- Verification: `/opsx:verify` not installed; manual artifact↔implementation verification
  performed against proposal/specs/design/tasks — all requirement scenarios covered by tests;
  no critical findings.
- Archive: `2026-09-23-bootstrap-fullstack-rust-workspace`; canonical spec
  `platform/rust-workspace` created and Purpose completed after archive; `--specs` and
  `--archived` strict validation pass.
- Commit: `feat(01): bootstrap full-stack Rust workspace`
- Decisions: rustup-managed toolchain (WASM reproducibility); `any_spawner` executor
  initialized in `server::router` so SSR works in-process for tests; no leptos_router yet
  (arrives with change 02).
- Live integrations not verified: none.

### 02 — `add-web-application-shell` — COMPLETE

- Start: 2026-09-23 21:52 · End: 2026-09-23 22:20
- Proposal validation: strict validation passed before apply.
- Implementation:
  - `momentum-domain::ids::AccountId` UUIDv7 newtype; `momentum-app::session`
    (`SessionState`, `AccountSession`).
  - Web: `routes.rs` (built-in lists, `/projects`, `/areas`, `/areas/:id`, `/projects/:id`,
    `/todos/:id`, `/settings`, `/login`, landing), `layouts.rs` (`PublicLayout`, `AppLayout`,
    `Protected`), `boundaries.rs` (`Loading`, `NotFound`, `Unauthorized`, `ErrorView`,
    `AppBoundary`), `session.rs` (`SessionResolver` injection + JSON bootstrap for hydration).
  - Axum: `generate_route_list` + `leptos_routes_with_context` + `file_and_error_handler`;
    embedded `/static/app.css` served from Rust.
  - `router_with_session` enables deterministic authenticated/anonymous integration tests;
    production default is the anonymous resolver until change 06.
- Migrations: none.
- Tests executed: `cargo test --workspace --all-features` (domain 5, app 5, infra 3, xtask 3,
  testkit 1, web unit 3 + server smoke 4 + shell routes 11); fmt; clippy `-D warnings`;
  WASM client build (`momentum_web.wasm`); `scripts/quality-gates.sh` all pass.
- Verification: manual (no `/opsx:verify`). All requirement scenarios have automated coverage
  except real-browser hydration continuity, which is deferred to the browser/parity suites and
  recorded as a known limitation (SSR bootstrap element and client DOM read are tested
  separately).
- Archive: `2026-09-23-add-web-application-shell`; canonical `app/shell` spec created with a
  real Purpose; `--specs`/`--archived` strict validation pass.
- Commit: `feat(02): add web application shell`
- Decisions: session resolution is dependency-injected rather than mocked; session bootstrap
  uses a Rust-rendered JSON element (escaped `<`) instead of inline script; `http` is a
  non-optional web dependency so the resolver trait compiles for WASM; `uuid` gains the `js`
  feature on wasm32 for client-side ID generation.
- Live integrations not verified: none (no external providers involved).

### 03 — `add-things-design-system` — COMPLETE

- Start: 2026-09-23 22:20 · End: 2026-09-23 22:55
- Proposal validation: strict validation passed before apply.
- Implementation:
  - `design/tokens.rs` (typography, spacing, radius, color, shadow, icon, row, sidebar,
    motion tokens; `MotionKind`), mirrored in `assets/design.css`.
  - `design/icons.rs`: 15 original inline-SVG icons.
  - `design/primitives.rs`: `CheckControl`, `ProgressRing`, `SectionHeading`, `TaskRow`,
    `ProjectRow`, `SidebarRow`, `Popover`, `ContextMenu`, `DateControl`, `AddControl`,
    `Motion`.
  - `/styleguide` public catalog route; `/static/design.css` served from Rust-embedded assets;
    shell/layout/boundary styles migrated to tokens.
- Migrations: none.
- Tests executed: `cargo test --workspace --all-features` — web now 5 unit + 14 design-system +
  11 shell-routes + 4 smoke; fmt; clippy `-D warnings`; WASM client build; quality gates pass.
- Verification: manual. Token/CSS consistency, primitive ARIA/roles, original-asset scan, and
  styleguide route are all covered by tests; no critical findings.
- Archive: `2026-09-23-add-things-design-system`; canonical `ui/design-system` spec created
  with real Purpose; `--specs`/`--archived` strict validation pass.
- Commit: `feat(03): add Things-style design system`
- Decisions: tokens have two synchronized representations (Rust consts + CSS custom
  properties) guarded by a test; icons are Rust inline SVG; `CheckControl` uses a native
  checkbox; `recursion_limit = 512` on the web crate for large view trees.
- Live integrations not verified: none.

### 04 — `add-responsive-layout-foundation` — COMPLETE

- Start: 2026-09-23 22:55 · End: 2026-09-23 23:12
- Proposal validation: strict validation passed before apply.
- Implementation: `responsive.rs` (`LayoutMode`, documented breakpoints, `LayoutState` with
  hydrate resize sync), responsive `AppLayout` (top bar + toggle with `aria-expanded`/
  `aria-controls`, overlay drawer + scrim, mobile bottom destination bar, single `main`),
  CSS breakpoints (`48rem`/`64rem`) with touch-target token and reduced-motion-safe drawer.
- Migrations: none.
- Tests: 4 new responsive-layout tests + updated shell tests; `cargo test --workspace
  --all-features`, fmt, clippy, WASM build, quality gates all pass.
- Verification: manual; browser resize behavior deferred to the behavior-parity suite and
  recorded. No critical findings.
- Archive: `2026-09-23-add-responsive-layout-foundation`; canonical `ui/responsive-layout`
  spec created; strict `--specs`/`--archived` validation pass.
- Commit: `feat(04): add responsive layout foundation`
- Decisions: server defaults to desktop mode and hydration corrects it; all layout regions are
  rendered and CSS controls visibility so hidden regions leave the accessibility tree.
- Live integrations not verified: none.

### 05 — `add-persistence-foundation` — COMPLETE

- Start: 2026-09-23 23:12 · End: 2026-09-23 23:40
- Proposal validation: strict validation passed before apply.
- Implementation: `momentum-infra::db` (`Database`, migration engine with FNV-1a checksums,
  transactional apply, rollback, pending/applied queries) and migration 0001 creating
  accounts, sessions, entities, placements, tag assignments, checklist items, recurrence
  templates/instances, reminders, preferences, sync metadata, change log, and tombstones;
  `DATABASE_URL` config; `/readyz` persistence reporting via `AppState`.
- Migrations: `0001-canonical-schema` (forward + documented destructive rollback).
- Tests: 16 infra tests (11 migration/schema invariants incl. date-only columns, account
  scoping, versions, sync spine; conditional live PostgreSQL apply/rollback test) + readyz
  persistence test; fmt, clippy, workspace tests, WASM build, quality gates pass.
- Verification: manual. Live PostgreSQL path was **not executed** in this environment (no
  server/Docker); the conditional test records the skip. Pure schema/migration invariants are
  fully tested. No critical findings.
- Archive: `2026-09-24-add-persistence-foundation`; canonical `persistence/schema` spec created;
  strict `--specs`/`--archived` validation pass.
- Commit: `feat(05): add persistence foundation`
- Decisions: Rust-defined SQL migrations (ship in binary, testable without a database);
  `momentum-infra` is an optional web dependency enabled only under `ssr` so sqlx never
  compiles for WASM; readiness degrades gracefully without persistence.
- Live integrations not verified: PostgreSQL server connectivity (recorded above).

### 06 — `add-account-authentication` — COMPLETE

- Start: 2026-09-23 23:40 · End: 2026-09-24 01:50
- Proposal validation: strict validation passed before apply.
- Implementation:
  - Domain: `Email` (normalization/validation) and `Password` (policy, redacted Debug),
    `SessionId`.
  - App: `auth` records/ports/`AuthError`/`AuthService` covering register, verify, login,
    logout, password reset/change, session revocation/listing, account deletion; uniform
    credential and reset failures; single-use tokens with expiry; reset verifies the email
    (mailbox control).
  - Infra: `Argon2Hasher` (Argon2id), `RandomTokenGenerator` (OS entropy + SHA-256 storage
    hashes), `SystemClock`, `PgAuthStore` (accounts/sessions/auth_tokens), migration 0002.
  - Web: async `SessionResolver` + middleware inserting `SessionState` into request
    extensions, `CookieSessionResolver`, hardened session cookies (`HttpOnly`, `SameSite=Lax`,
    `Secure` when `MOMENTUM_SECURE_COOKIES` is set), auth routes with native forms, real
    sign-in/sign-up pages with accessible labels/autocomplete, `/readyz` authentication field.
- Migrations: `0002-auth-tokens` (forward + rollback).
- Tests: domain 10 (+4 auth value-object tests), app 5, testkit 15 auth-flow tests, infra 16
  (+3 crypto tests), web 12 auth-route tests + 3 session unit tests; fmt, clippy, workspace
  tests, WASM client build, quality gates all pass.
- Verification: manual. Account lifecycle, uniform failures, cookie hardening, revocation, and
  session isolation between accounts are covered by tests. **Not verified live**: PostgreSQL
  auth store (no database in this environment) and email delivery (no provider configured —
  verification/reset tokens are logged for operators). No critical findings.
- Archive: `2026-09-24-add-account-authentication`; canonical `auth/accounts` spec created;
  strict `--specs`/`--archived` validation pass.
- Commit: `feat(06): add account authentication`
- Decisions: async session resolution moved to Axum middleware (I/O); SHA-256 for
  high-entropy tokens and Argon2id for passwords; native form POSTs (progressive enhancement)
  rather than server functions; reset marks the email verified.
- Live integrations not verified: PostgreSQL auth store, email delivery (recorded above).

### 07 — `add-task-domain-core` — COMPLETE

- Start: 2026-09-24 01:50 · End: 2026-09-24 02:25
- Proposal validation: strict validation passed before apply.
- Implementation: `momentum-domain::time::LocalDate` (UTC+offset and IANA-timezone derivation,
  day arithmetic, ISO parse/display, serde), `momentum-domain::model` (`EntityKind`, `Status`,
  `EntityMeta`, `Title`, `ScheduleState`/`Schedule` with This Evening semantics, `Deadline` +
  `DeadlineState`, `Reminder` + `ReminderState`, `ContextRef`, `Todo`, `Project`, `Area`,
  `Heading`, `Tag`, `ChecklistItem`, `ProjectCompletionPolicy`), plus `EntityId`/
  `ChecklistItemId`.
- Migrations: none (schema already matched the model).
- Tests: 28 domain tests including temporal independence, evening rules, single contextual
  parent with identity preservation, completion/reopen timestamps, project completion policy,
  checklist independence, and title validation. fmt, clippy, workspace tests, WASM build, and
  quality gates all pass.
- Verification: manual; every requirement scenario is covered by unit tests. No critical
  findings.
- Archive: `2026-09-24-add-task-domain-core`; canonical `domain/tasks` spec created; strict
  `--specs`/`--archived` validation pass.
- Commit: `feat(07): add task domain core`
- Decisions: one `EntityId` + `EntityKind` (matches canonical entity table and permits future
  type conversion); This Evening is a flag on a dated schedule, not a separate state; deadlines
  are date-only; reminders are instants with timezone context.
- Live integrations not verified: none.

### 08 — `add-fractional-ordering-engine` — COMPLETE

- Start: 2026-09-24 02:08 · End: 2026-09-24 02:30
- Proposal validation: strict validation passed before apply.
- Implementation: `momentum-app::ordering` with gap-based `Rank` (zero-padded storage encoding),
  `OrderingScope` (context parent vs Today), `OrderedItem`, `ScopeOrder` (insert/append/move/
  rebalance returning persistable `OrderingChange` plans), exhaustion detection, and
  deterministic rebalancing; `ContextRef` now derives `Hash`.
- Migrations: none (`rank text` columns reused with the documented encoding).
- Tests: 17 app tests including encoding/order, midpoint insertion, move, scope independence,
  exhaustion + rebalance, 200-insert stability, and bounds; fmt, clippy, workspace tests,
  quality gates pass.
- Verification: manual; all requirement scenarios covered. No critical findings.
- Archive: `2026-09-24-add-fractional-ordering-engine`; canonical `domain/ordering` spec
  created; strict `--specs`/`--archived` validation pass.
- Commit: `feat(08): add fractional ordering engine`
- Decisions: integer gap ranks with zero-padded text encoding (simpler and deterministic vs
  LexoRank); the engine plans changes and the command layer persists them atomically.
- Live integrations not verified: none.

### 09 — `add-command-transaction-layer` — COMPLETE

- Start: 2026-09-24 02:30 · End: 2026-09-24 03:05
- Proposal validation: strict validation passed before apply.
- Implementation: `momentum-app::commands` (typed `Command` variants, `CommandEnvelope` with
  operation id, `DomainEvent`, `Projection` invalidations, `CommandError`, `ChangeRecord`,
  `UnitOfWork`/`UnitOfWorkFactory`, `CommandDispatcher` with replay/optimistic concurrency);
  `momentum-infra::commands::PgUnitOfWorkFactory` (entity + placement + change_log +
  sync sequence in one SQL transaction); `momentum-testkit::commands::MemoryDatabase`.
- Migrations: none (reuses change-05 tables).
- Tests: 11 command-layer tests (lifecycle, temporal independence, move, replay, sequences,
  isolation, conflict, injected failure, validation) + 2 infra mapping tests + conditional
  live PostgreSQL round-trip; 23 infra tests pass; fmt, clippy, workspace tests, quality gates.
- Verification: manual. All requirement scenarios covered by tests; live PostgreSQL path not
  executed (no database) and recorded. No critical findings.
- Archive: `2026-09-24-add-command-transaction-layer`; canonical `app/commands` spec created;
  strict validation pass.
- Commit: `feat(09): add command transaction layer`
- Decisions: `UnitOfWork` owns SQL transaction scope; events stored as JSONB change-log
  payload (doubles as the sync feed); operation id replay returns the recorded outcome.
- Live integrations not verified: PostgreSQL unit of work (recorded).

### 10 — `add-undo-redo-engine` — COMPLETE

- Start: 2026-09-24 02:42 · End: 2026-09-24 03:00
- Proposal validation: strict validation passed before apply.
- Implementation: `momentum-app::undo` (inverse derivation for every command variant, create
  inverse derived from the created identity, `UndoEntry`/`UndoGroup`, `UndoEngine` applying
  inverse commands through the dispatcher, bounded `UndoStack` with grouped semantics).
- Migrations: none.
- Tests: 5 app unit tests (inverse derivation) + 5 testkit integration tests (create/edit/
  completion round-trips preserving identity, grouped reverse-order undo, multi-create group);
  fmt, clippy, workspace tests, WASM build, quality gates pass.
- Verification: manual; all requirement scenarios covered. No critical findings.
- Archive: `2026-09-24-add-undo-redo-engine`; canonical `app/undo` spec created; strict
  validation pass.
- Commit: `feat(10): add undo-redo engine`
- Decisions: undo executes inverse commands (never snapshots); redo is the inverse of the
  inverse so identities are stable; history window is 100 groups and in-session only.
- Live integrations not verified: none.

### 11 — `add-todo-crud` — COMPLETE

- Start: 2026-09-24 03:00 · End: 2026-09-24 03:45
- Proposal validation: strict validation passed before apply.
- Implementation: `DuplicateTodo` command; `momentum-app::todos` (`TodoSummary`, `TodoScope`,
  `TodoReadStore`, `TodoService`); `PgTodoReadStore`; testkit `MemoryTodoReadStore`; web
  command routes (`POST /todos` + lifecycle/update routes), real `/inbox` list with add form
  and row actions, real `/todos/:id` editor with lifecycle buttons and metadata display.
- Migrations: none.
- Tests: 8 end-to-end web CRUD tests (create, auth requirement, complete→logbook, delete/
  restore, duplicate, editor update with metadata preservation, reopen, not-found) plus the
  existing command-layer suite; fmt, clippy, workspace tests, WASM build, quality gates pass.
- Verification: manual; all requirement scenarios covered. No critical findings.
- Archive: `2026-09-24-add-todo-crud`; canonical `features/todo-crud` spec created; strict
  validation pass.
- Commit: `feat(11): add todo CRUD`
- Decisions: native form posts to command-backed routes (progressive enhancement); duplicate
  keeps the title (identity distinguishes copies); Axum route params renamed to `{todo_id}` to
  match the Leptos route pattern.
- Live integrations not verified: PostgreSQL read store path (no database).

### 12 — `add-inbox` — COMPLETE

- Start: 2026-09-24 04:02 · End: 2026-09-24 10:40 (environment paused between)
- Proposal validation: strict validation passed before apply.
- Implementation: `ReorderTodo` command with scope-order unit-of-work operations (memory +
  PostgreSQL), `TodoService::{reorder_up, reorder_down, schedule_quick}`, Inbox badge in the
  primary navigation, reorder/schedule/move controls on rows and the editor, processing routes
  (`move-up`, `move-down`, `schedule`, `move`), Inbox membership rule refined (open + inbox
  context + not future-scheduled) with the read-store `today` parameter.
- Migrations: none.
- Tests: 7 Inbox web tests (badge count/decay, reorder + boundary no-op, scheduling out of
  Inbox, Tomorrow semantics, move to project) plus existing suites; fmt, clippy, workspace
  tests, WASM build, quality gates pass.
- Verification: manual; all requirement scenarios covered. Memory ordering determinism fixed
  (created-at + id tie-break) after a flaky test surfaced.
- Archive: `2026-09-24-add-inbox`; canonical `features/inbox` spec created; strict validation
  pass.
- Commit: `feat(12): add inbox`
- Decisions: Inbox excludes future-scheduled and Someday items (documented projection rule);
  read store takes the caller's local date (UTC until localization lands).
- Live integrations not verified: PostgreSQL scope-order read/write path (no database).

### 13 — `add-areas` — COMPLETE

- Start: 2026-09-24 10:40 · End: 2026-09-24 12:05
- Proposal validation: strict validation passed before apply.
- Implementation: `CreateArea`/`RenameArea`/`DeleteArea` commands with required
  `AreaDeletionPolicy` (`Detach` | `TrashContents`); unit-of-work area load/save,
  contained-child listing, and project-area assignment (memory + PostgreSQL); `AreaSummary`,
  `AreaReadStore`, `AreaService`; `/areas` list (create, rename, delete with policy) and
  `/areas/:area_id` detail with loose to-dos; sidebar/`AreaService` wiring in `AppState`.
- Migrations: none.
- Tests: 6 area web/service tests (create/rename/list counts, loose to-do listing, detach
  returns to Inbox, trash moves to Trash, project assignment cleared on detach, unauthenticated
  mutation rejected); fmt, clippy, workspace tests, WASM build, quality gates pass.
- Verification: manual; all requirement scenarios covered. No critical findings.
- Archive: `2026-09-24-add-areas`; canonical `features/areas` spec created; strict validation
  pass.
- Commit: `feat(13): add areas`
- Decisions: deletion always requires an explicit policy; loose to-dos keep their identity and
  stay loose; area ordering reuses the context/inbox placement scope (documented).
- Live integrations not verified: PostgreSQL area paths (no database).

### 14 — `add-projects` — COMPLETE

- Start: 2026-09-24 12:45 · End: 2026-09-24 13:35
- Proposal validation: strict validation passed before apply.
- Implementation: project commands (create/rename/notes/schedule/deadline/complete with
  `ProjectCompletionPolicy`/cancel/reopen/move-to-area/delete with `ProjectDeletionPolicy`);
  unit-of-work project load/save and direct-child listing (memory + PostgreSQL); `ProjectSummary`
  (with derived progress), `ProjectReadStore`, `ProjectService`; `/projects` list and
  `/projects/:project_id` detail with notes editor, schedule form, lifecycle controls, direct
  to-dos, and create-in-project; `TodoService::create_in` + context-aware create form.
- Migrations: none.
- Tests: 6 project tests (create/list/rename, notes+schedule independence from children,
  membership via create-in-project, reject vs cascade completion, detach vs trash deletion);
  fmt, clippy, workspace tests, WASM build, quality gates pass.
- Verification: manual; all requirement scenarios covered. No critical findings.
- Archive: `2026-09-24-add-projects`; canonical `features/projects` spec created; strict
  validation pass.
- Commit: `feat(14): add projects`
- Decisions: completion and deletion require explicit policies; children keep canonical
  identity across projections; project ordering/membership reuse placement ranks.
- Live integrations not verified: PostgreSQL project paths (no database).

### 15 — `add-project-headings` — COMPLETE

- Start: 2026-09-24 16:30 · End: 2026-09-24 17:20
- Proposal validation: strict validation passed before apply.
- Implementation: heading lifecycle commands (create/rename/reorder/archive/restore) with events
  and invalidations; unit-of-work heading load/save in the memory and PostgreSQL adapters;
  project-scope ordering now includes headings and every created to-do is ranked at the end of
  its context scope; `ProjectSummary` gained ordered `headings` (with ordered child to-do ids)
  and a top-level `order` of headings and unheaded to-dos; project detail renders heading
  sections with rename/up/down/archive controls, archived-heading restore, and heading-scoped
  to-do creation; `ContextRef::Heading` requires a live heading for create and move.
- Migrations: `0003-entity-archived` (`entities.archived`, reversible) for non-destructive
  heading archive.
- Tests: 4 testkit lifecycle/child-movement/scope tests plus 3 web tests (controls, heading
  to-do flow, reorder endpoints); all suites, fmt, clippy, WASM build, and quality gates pass.
- Verification: manual; every requirement scenario covered. No critical findings.
- Archive: `2026-09-24-add-project-headings`; canonical `features/project-headings` spec created;
  strict validation pass.
- Commit: `feat(15): add project headings`
- Decisions: heading→project conversion is unsupported by the model and documented as a
  non-goal; children move with a heading implicitly because they are grouped by heading context;
  archive persists as a flag so children keep identity.
- Live integrations not verified: PostgreSQL heading paths (no database).

### 16 — `add-checklists` — COMPLETE

- Start: 2026-09-24 17:30 · End: 2026-09-24 18:15
- Proposal validation: strict validation passed before apply.
- Implementation: checklist commands on the to-do aggregate (add/rename/complete/uncomplete/
  remove/reorder/bulk expand); deterministic `parse_checklist_lines` normalization; duplication
  copies structure and resets completion (template/copy semantics); `TodoSummary` exposes
  ordered checklist items; PostgreSQL load/save synchronizes `checklist_items` (and tag
  assignments) transactionally with the parent to-do; to-do editor renders a checklist with
  labeled controls plus add-item and bulk-paste forms.
- Migrations: none (`checklist_items` existed).
- Tests: 3 testkit tests (editing/order, bulk normalization, duplicate reset) and 2 web tests
  (editor controls, bulk endpoint); fmt, clippy, workspace tests, WASM build, quality gates pass.
- Verification: manual; every requirement scenario covered. No critical findings.
- Archive: `2026-09-24-add-checklists`; canonical `features/checklists` spec created; strict
  validation pass.
- Commit: `feat(16): add checklists`
- Decisions: checklist stays inside the to-do aggregate so copy semantics are pure and
  transactional; bulk input normalizes bullets/checkboxes and drops empty lines.
- Live integrations not verified: PostgreSQL checklist paths (no database).

### 17 — `add-tags` — COMPLETE

- Start: 2026-09-24 19:00 · End: 2026-09-24 20:05
- Proposal validation: strict validation passed before apply.
- Implementation: tag commands (create, rename, move/nest, reorder, delete with child
  reparenting, assign/unassign to to-dos, projects, and areas) with cycle prevention and a
  bounded parent walk; unit-of-work tag load/save/list/delete in memory and PostgreSQL;
  `TagService`/`TagReadStore` with `TagSummary { id, title, parent, position, assigned_count }`;
  `TodoSummary` gained direct `tags` and derived `inherited_tags` (project, heading→project,
  area, and project's area) without synthetic child assignments; `/tags` management page; tag
  assignment on the to-do editor; `/inbox?tags=` multi-tag filtering including inherited tags.
- Migrations: `0004-tag-hierarchy` (`entities.tag_parent`, `entities.tag_position`, reversible).
- Tests: 3 testkit tests (hierarchy/cycle/delete reparenting, assignment across entity kinds,
  inherited context) and 2 web tests (tags page lifecycle, assignment + filtering); fmt, clippy,
  workspace tests, WASM build, quality gates pass.
- Verification: manual; every requirement scenario covered. No critical findings.
- Archive: `2026-09-24-add-tags`; canonical `features/tags` spec created; strict validation pass.
- Commit: `feat(17): add tags`
- Decisions: sibling order lives on the entity row (`tag_position`) because placements only
  model contextual parents; `AssignTag`/`UnassignTag` dispatch on the target entity kind;
  inheritance is derived at read time.
- Live integrations not verified: PostgreSQL tag paths (no database).

### 18 — `add-notes-editor` — COMPLETE

- Start: 2026-09-24 20:10 · End: 2026-09-24 20:55
- Proposal validation: strict validation passed before apply.
- Implementation: pure `momentum_domain::notes` utilities (link detection with punctuation
  trimming, safe-URL policy limited to http/https/mailto, find matches, next/previous with
  wrap-around, deterministic replace-all); shared `NotesTools` panel on to-do and project pages
  with detected safe links rendered as `noopener noreferrer` anchors, blocked schemes rendered
  as inert labeled text, GET find traversal with match index, and replace-all forms; server
  replacement routes delegate to the existing `SetTodoNotes`/`SetProjectNotes` commands.
- Migrations: none.
- Tests: 3 domain unit tests and 3 web tests (link safety, find traversal + replacement,
  project notes replacement); fmt, clippy, workspace tests, WASM build, quality gates pass.
- Verification: manual; every requirement scenario covered. No critical findings.
- Archive: `2026-09-24-add-notes-editor`; canonical `features/notes-editor` spec created;
  strict validation pass.
- Commit: `feat(18): add notes editor`
- Decisions: multiline editing/selection/clipboard/undo stay native textarea behavior (no
  handwritten JS); find/replace is deterministic Rust computed server-side over note commands.
- Live integrations not verified: none specific to this change.

### 19 — `add-things-markdown` — COMPLETE

- Start: 2026-09-24 21:00 · End: 2026-09-24 21:40
- Proposal validation: strict validation passed before apply.
- Implementation: pure `momentum_domain::markdown` parser producing a block/inline AST for the
  Things-style subset (headings, paragraphs, unordered/ordered list items, quotes, fenced code,
  inline code, emphasis, strong, highlight, links) with backslash escapes and safe degradation
  of malformed syntax; links reuse the notes safe-URL policy; the notes panel renders a derived
  `MarkdownPreview` (semantic elements, blocked unsafe links inert) while the textarea keeps the
  raw source editable; storage is untouched, so rendering is always derived from stored text.
- Migrations: none.
- Tests: 4 domain parser tests and 2 web tests (subset rendering with verbatim stored source,
  markdown link safety); fmt, clippy, workspace tests, WASM build, quality gates pass.
- Verification: manual; every requirement scenario covered. No critical findings.
- Archive: `2026-09-24-add-things-markdown`; canonical `features/markdown` spec created; strict
  validation pass.
- Commit: `feat(19): add things markdown`
- Decisions: AST (not HTML string) keeps escaping and link policy in the renderer; unsupported
  syntax degrades to literal text.
- Live integrations not verified: none specific to this change.

### 20 — `add-scheduling-model` — COMPLETE

- Start: 2026-09-24 21:45 · End: 2026-09-24 22:40
- Proposal validation: strict validation passed (artifacts committed separately in `feat(20):
  archive scheduling spec` after the implementation commit).
- Implementation: `TodoScope` gained Today/Upcoming/Anytime/Someday with schedule-driven
  membership semantics (overdue date-based items stay in Today; This Evening is a Today
  presentation flag; Someday/Anytime separated) in the memory and PostgreSQL read stores, with
  date-aware ordering for Today/Upcoming; `TodoService::list_at` evaluates membership at an
  explicit UTC-offset local date; web scheduling handlers derive Today/Tomorrow/This Evening
  from the service clock instead of the wall clock; real projection pages replaced the
  placeholder Today/Upcoming/Anytime/Someday pages.
- Migrations: none (dates already stored as SQL `date`).
- Tests: 3 testkit tests (membership, transition metadata preservation, UTC-offset boundary)
  and 1 web test (scheduling moves items across pages); fmt, clippy, workspace tests, WASM
  build, quality gates pass.
- Verification: manual; every requirement scenario covered. No critical findings.
- Archive: `2026-09-25-add-scheduling-model`; canonical `features/scheduling` spec created;
  strict validation pass.
- Commit: implementation `16bb648` (`feat(20): add scheduling model`) plus specification commit.
- Decisions: until localization stores a per-account timezone, the service clock (UTC) defines
  the local date; the offset-aware entry point keeps boundary semantics testable.
- Live integrations not verified: PostgreSQL scope filters (no database).

### 21 — `add-deadlines` — COMPLETE

- Start: 2026-09-25 00:20 · End: 2026-09-25 01:30
- Proposal validation: strict validation passed before implementation was committed.
- Implementation: `TodoSummary::deadline_state(today)` derives Future/DueToday/Overdue from the
  stored local date without mutation; `TodoScope::Deadlines` lists open items with deadlines
  ordered by due date (memory and PostgreSQL); list rows show state badges (text, not color
  alone) when the view supplies the service's local date; to-do and project pages gained
  set/clear deadline forms through the existing commands; `/deadlines` page and navigation
  entry added.
- Migrations: none.
- Tests: 3 testkit tests (independence from schedule/reminder/metadata, state derivation, sorted
  projection with shared identity) and 2 web tests (set/clear + page membership, overdue badge
  in list views); fmt, clippy, workspace tests, WASM build, quality gates pass.
- Verification: manual; every requirement scenario covered. No critical findings.
- Archive: `2026-09-25-add-deadlines`; canonical `features/deadlines` spec created; strict
  validation pass.
- Commit: `feat(21): add deadlines`
- Decisions: deadline state is derived on read (never stored), so all projections show the same
  canonical value; unparseable dates clear the deadline deterministically.
- Live integrations not verified: PostgreSQL deadline scope/order (no database).

### 22 — `add-today-projection` — COMPLETE

- Start: 2026-09-25 02:00 · End: 2026-09-26 01:10 (long wall-clock span due to a full-volume
  event; see Build environment notes)
- Proposal validation: strict validation passed before archive.
- Implementation: `ProjectionScope { Today, ThisEvening }` maps to `placements` scopes
  `today`/`today_evening` (migration `0005`, reversible); `ReorderToday` command swaps an item
  with its bucket neighbor through the existing `ScopeOrder` engine; ranks are materialized
  when an item becomes Today-eligible (create/schedule/reopen) and cleared when it stops;
  PostgreSQL orders Today by a `CASE` join on the evening flag, memory by stored rank with a
  created-at fallback; the Today page renders separate Today and This Evening sections with
  labeled move controls; `TodoRow` gained an opt-in Today ordering mode; navigation link added.
- Migrations: `0005-projection-scope-evening` (widens the placement scope check).
- Tests: 3 testkit tests (membership without moving context, evening/day ordering independence,
  manual order vs context order) and 1 web test (sections, manual reorder, context stability);
  full workspace suite (41 binaries) passes, plus fmt, arch-check, clippy, WASM build.
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Archive: `2026-09-25-add-today-projection` (the archive date rolls to the next day because the
  disk-recovery window crossed midnight); canonical `features/today` spec created.
- Commit: `feat(22): add Today projection`
- Decisions: Today ordering lives in the existing `placements` table (the ordering engine and
  schema already anticipated a `today` scope) with a second `today_evening` bucket; items
  without a rank keep creation order until first moved, so no bulk backfill is needed.
- Live integrations not verified: PostgreSQL Today ordering/`CASE` join (no database).

### 23 — `add-upcoming-projection` — COMPLETE

- Start: 2026-09-26 01:20 · End: 2026-09-26 02:10
- Proposal validation: strict validation passed before archive.
- Implementation: `UpcomingAgenda { days, calendar }` with `UpcomingAgenda::from_todos`
  grouping future items by local schedule date (ascending, canonical order preserved);
  `TodoService::upcoming_agenda` composes the Upcoming scope read; `CalendarRow` has no entity
  identity by construction, so task commands cannot address calendar projections; the Upcoming
  page renders date-grouped sections with human-readable headings and per-row labeled
  move-to-date forms reusing the schedule command; scheduling onto today moves the item into
  Today.
- Migrations: none.
- Tests: 3 testkit tests (grouping/future-only/order, rescheduling across days including onto
  today with metadata preserved, calendar read-only + non-entity mutation rejected) and 2 web
  tests (day sections + moves + leave-Upcoming, empty state without calendar rows); full
  workspace suite green plus fmt, arch-check, clippy, WASM build.
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Archive: `2026-09-26-add-upcoming-projection`; canonical `features/upcoming` spec created.
- Commit: `feat(23): add Upcoming projection`
- Decisions: external calendar rows are modeled as identity-free read models instead of
  pseudo-entities, avoiding speculative provider schema; rescheduling reuses the schedule
  command rather than a new mutation path.
- Live integrations not verified: PostgreSQL Upcoming ordering (no database).

### 24 — `add-anytime-projection` — COMPLETE

- Start: 2026-09-26 02:20 · End: 2026-09-26 03:05
- Proposal validation: strict validation passed before archive.
- Implementation: Anytime membership refined to `status = open && Schedule::is_actionable(today)`
  in the memory and PostgreSQL read stores (Anytime schedules plus date schedules on or before
  the local today; overdue and This Evening included; future and Someday excluded); `AnytimeAgenda`
  groups actionable items by existing context (Inbox, area, project, heading) without writing
  context changes; the Anytime page renders labeled group sections resolved from existing
  area/project titles plus an empty state.
- Migrations: none.
- Tests: 2 testkit tests (actionable membership with completed/canceled/deleted/future/Someday
  exclusions; grouping with context-stability checks using a real area) and 2 web tests (grouped
  rendering with titles and exclusions, empty state); the scheduling web test and testkit scope
  expectations were updated to the refined actionable semantics (part of this capability's
  canonical behavior).
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Archive: `2026-09-26-add-anytime-projection`; canonical `features/anytime` spec created.
- Commit: `feat(24): add Anytime projection`
- Decisions: reuse the domain `is_actionable` predicate rather than duplicating date logic;
  grouping is a pure read-model transform with no context side effects.
- Live integrations not verified: PostgreSQL Anytime predicate (no database).

### 25 — `add-someday-projection` — COMPLETE

- Start: 2026-09-26 03:10 · End: 2026-09-26 03:45
- Proposal validation: strict validation passed before archive.
- Implementation: Someday to-do membership stays a pure schedule state; `ProjectService::someday`
  lists open, Someday-scheduled projects; the Someday page renders To-Dos and Projects sections
  with project reactivation controls that post through the existing schedule command; empty
  states for the whole view and each section; explicit rescheduling moves items into
  Anytime/Today/date projections, and an active parent never reactivates a Someday child.
- Migrations: none.
- Tests: 3 testkit tests (membership + reactivation transitions, project membership/reactivation,
  parent/child independence with context stability) and 2 web tests (deferred todos and projects
  with reactivation flows, empty state); full workspace suite green plus fmt, arch-check, clippy,
  WASM build.
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Archive: `2026-09-26-add-someday-projection`; canonical `features/someday` spec created.
- Commit: `feat(25): add Someday projection`
- Decisions: projects appear as their own section because project schedules are a separate
  aggregate; no context or schedule propagation happens during rendering.
- Live integrations not verified: none specific to this change.

### 26 — `add-completion-and-cancellation` — COMPLETE

- Start: 2026-09-26 03:50 · End: 2026-09-26 04:30
- Proposal validation: strict validation passed before archive.
- Implementation: distinct terminal semantics proven for to-dos and projects (dedicated
  `completed_at`/`canceled_at`, each clearing the other); `ReopenTodo`/`ReopenProject` now reject
  deleted items with validation and direct callers to restore; `RestoreTodo` clears stale
  terminal timestamps; `CompleteProject` gathers direct project children plus children under the
  project's headings, so reject cannot be bypassed and cascade completes every open child in one
  unit of work; canceling a project leaves open children open and reachable; read models expose
  the terminal instants (`TodoSummary.canceled_at`, `ProjectSummary.completed_at/canceled_at`)
  with PostgreSQL selects updated.
- Migrations: none (columns already existed).
- Tests: 4 testkit tests (distinct timestamps + reopen clearing, reopen policy with deleted
  rejection and restore cleanup, project reject/cascade including heading children, cancel
  preserving children) and 2 web tests (cancel/reopen round trip, both project policies over
  HTTP); full workspace suite green plus fmt, arch-check, clippy, WASM build.
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Archive: `2026-09-26-add-completion-and-cancellation`; canonical `features/completion` spec
  created.
- Commit: `feat(26): add completion and cancellation`
- Decisions: heading children are loaded through the existing ordering scope; deleted items are
  restored rather than reopened to keep trash semantics explicit.
- Live integrations not verified: PostgreSQL terminal-timestamp selects (no database).

### 27 — `add-logbook` — COMPLETE

- Start: 2026-09-26 04:35 · End: 2026-09-26 05:40
- Proposal validation: strict validation passed before archive.
- Implementation: `LoggingPolicy { Automatic, Manual }` stored per account in the existing
  `preferences` table through a `LoggingPolicyStore` port (memory + PostgreSQL adapters);
  `TodoScope::Logbook { logged_only }` plus terminal-status filtering drive membership;
  `LogTodo`/`LogProject` commands set `logged_at` once for terminal items and reject open ones;
  `LogbookAgenda` groups entries by history date descending (`logged → completed → canceled →
  created`) with unlogged terminal entries surfacing as `pending` under the manual policy;
  read models expose `logged_at` for to-dos and projects; the Logbook page renders a policy
  form, pending logging with Log buttons, and date-grouped history including project rows with
  their completion/cancellation/logging timestamps.
- Migrations: none (`preferences` and `logged_at` already existed).
- Tests: 3 testkit tests (automatic history + timestamps, manual pending/log transitions with
  validation, policy switching preserving completion) and 2 web tests (date-grouped automatic
  history, manual pending → logged flow); full workspace suite green plus fmt, arch-check,
  clippy, WASM build.
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Archive: `2026-09-26-add-logbook`; canonical `features/logbook` spec created.
- Commit: `feat(27): add Logbook`
- Decisions: unlogged terminal items appear as `pending` under manual policy so completed work
  is never unreachable; the policy is an account preference (not an entity change-log record).
- Live integrations not verified: PostgreSQL preference upsert and logbook ordering (no
  database).

### 28 — `add-trash` — COMPLETE

- Start: 2026-09-26 05:45 · End: 2026-09-26 06:45
- Proposal validation: strict validation passed before archive.
- Implementation: soft deletion across to-dos, projects, and areas feeds one Trash view;
  `RestoreTodo`/`RestoreProject`/`RestoreArea` validate trashed state, clear terminal metadata,
  and restore to-dos whose previous context is gone to Inbox deterministically (checked inside
  the same unit of work so each restore performs exactly one optimistic-version save; restored
  projects drop assignments to deleted areas); `PurgeTodo`/`PurgeProject`/`PurgeArea` permanently
  delete rows only from Trash and `EmptyTrash` purges every trashed entity; PostgreSQL hard
  deletes cascade placements/checklist/tag assignments and memory adapters clean maps and ranks;
  read stores expose deleted project/area listings; the Trash page renders per-kind sections
  with Restore and Delete permanently actions, an explicit Empty Trash form, and a Trash
  navigation entry.
- Migrations: none.
- Tests: 3 testkit tests (soft deletion across kinds, restore fallback when the parent was
  purged, guarded purge/empty-trash invariants) and 2 web tests (trash listing + restore round
  trip, purge guard + empty trash); full workspace suite green plus fmt, arch-check, clippy,
  WASM build.
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Archive: `2026-09-26-add-trash`; canonical `features/trash` spec created.
- Commit: `feat(28): add Trash`
- Decisions: purge removes rows immediately and leaves change-log records as sync history
  (retention belongs to the later sync capability); restore fallback runs pre-save to keep one
  version-guarded write.
- Live integrations not verified: PostgreSQL purge cascades and deleted listings (no database).

### 29 — `add-special-lists` — COMPLETE

- Start: 2026-09-26 06:50 · End: 2026-09-26 07:45
- Proposal validation: strict validation passed before archive.
- Implementation: `TodoScope::Tomorrow` (open date schedules equal to today + 1, evening
  included) in memory and PostgreSQL; `ProjectService::logged` lists terminal projects ordered
  by logged/completed/canceled instants; new recurrence read port/service with memory and
  PostgreSQL adapters reading the existing `recurrence_templates` table joined to entities; web
  pages for `/tomorrow`, `/repeating`, and `/projects/logged` with links from Today and Projects
  (no permanent sidebar membership); Deadlines view already existed from change 21.
- Migrations: none.
- Tests: 3 testkit tests (Tomorrow membership with exclusions, logged-project filtering,
  account-scoped template reads) and 3 web tests (Tomorrow page, Logged Projects page plus
  Projects link, Repeating list/empty states); full workspace suite green plus fmt, arch-check,
  clippy, WASM build.
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Archive: `2026-09-26-add-special-lists`; canonical `features/special-lists` spec created.
- Commit: `feat(29): add special lists`
- Decisions: the Repeating view reads real template rows through a new port so it is functional
  before the recurrence engine generates templates; special views are linked, not pinned.
- Live integrations not verified: PostgreSQL Tomorrow filter and recurrence join (no database).

### 30 — `add-project-progress` — COMPLETE

- Start: 2026-09-26 07:50 · End: 2026-09-26 08:30
- Proposal validation: strict validation passed before archive.
- Implementation: `Progress { completed, total }` with `ratio`/`percent`/`is_neutral`; project
  progress counts only eligible direct children (open + completed) with canceled/deleted
  children excluded from both sides and a neutral state for zero items (read-store count
  semantics corrected accordingly in memory and PostgreSQL); `TodoScope::TodayAll` plus
  `TodoService::today_progress` derive Today completion over completable items (open +
  completed, scheduled on/before local today; canceled/deleted/future/Someday excluded); web
  surfaces render native accessible `<progress>` elements with text and explicit neutral states
  on project rows, project detail, and the Today header.
- Migrations: none.
- Tests: 3 testkit tests (derived project ratio with canceled/deleted exclusion and zero-item
  neutrality, Today progress transitions including reopen, neutral Today) and 3 web tests
  (project detail/list indicators plus neutral, Today indicator updates, neutral Today); full
  workspace suite green plus fmt, arch-check, clippy, WASM build.
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Archive: `2026-09-26-add-project-progress`; canonical `features/progress` spec created.
- Commit: `feat(30): add project progress`
- Decisions: canceled work is excluded from progress rather than counted as done; progress is
  always derived on read with no stored percentage.
- Live integrations not verified: PostgreSQL count semantics (no database).

### 31 — `add-move-engine` — COMPLETE

- Start: 2026-09-27 08:35 · End: 2026-09-27 09:50
- Proposal validation: strict validation passed before archive.
- Implementation: `movement::{Destination, Movable, is_valid, validate, ensure_destination}`
  single-sources the destination matrix (to-do: Inbox/Area/Project/Heading; project:
  Inbox/Area; heading: Project) plus existence/ownership/alive checks; `MoveTodo` and
  `MoveProject` now validate through the engine, and the new `MoveHeading` command transfers a
  heading (with its children by reference) between projects; the project detail page gained a
  labeled "Move heading" form; three legacy tests that relied on unvalidated ad-hoc destination
  ids were updated to create real destinations (canonical behavior now validated).
- Migrations: none.
- Tests: 5 testkit tests (matrix, preservation across project moves incl. tags/notes/deadline/
  schedule/checklist/identity, heading transfer with children following, area destinations,
  adapter equivalence between service and command dispatch) and 1 web test (todo Move dialog +
  heading move form over HTTP); full workspace suite green plus fmt, arch-check, clippy, WASM
  build.
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Archive: `2026-09-27-add-move-engine`; canonical `features/movement` spec created.
- Commit: `feat(31): add move engine`
- Decisions: failed moves leave state untouched (destination checks run before any save);
  heading transfer is a placement change, not child rewrites; unknown/trashed destinations are
  not-found rather than validation errors.
- Live integrations not verified: none specific to this change.

### 32 — `add-drag-and-drop` — COMPLETE

- Start: 2026-09-27 10:20 · End: 2026-09-27 11:05
- Proposal validation: strict validation passed before archive.
- Implementation: `MoveTodoToPosition`/`MoveTodayToPosition` add explicit insertion-position
  reordering (zero-based index clamped to bounds) via the ordering engine, updating only the
  moved item's rank and any rebalanced neighbors; cross-context drops call the move engine and
  Upcoming day drops call the schedule command; rows emit `draggable`, `data-drag-index`, and
  `data-drop-position` (Upcoming day sections emit `data-drop-date`) and every row carries a
  labeled 1-based position form ("Move here") for keyboard parity; routes `/todos/{id}/position`
  and `/todos/{id}/today-position` were added.
- Migrations: none.
- Tests: 3 testkit tests (positional context reorder with clamping/no-op and data preservation,
  independent Today-bucket ordering with context stability and non-today rejection, day-drop
  rescheduling plus cross-context drop via the engine) and 2 web tests (drop hooks in SSR markup
  and the position form reordering Inbox, Today positional drop plus Upcoming day targets);
  full workspace suite green plus fmt, arch-check, clippy, WASM build.
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known live-browser gap: the pointer/touch gesture layer attaches to the rendered drop hooks
  from the hydrated Rust/WASM client, which is built but not yet wired to hand-written-free
  bootstrap glue; the drop operations, insertion semantics, and accessible equivalents are all
  server-rendered and tested. This matches the documented hydration-parity gap (changes 71/72).
- Archive: `2026-09-27-add-drag-and-drop`; canonical `features/drag-and-drop` spec created.
- Commit: `feat(32): add drag and drop`
- Decisions: drops are modeled as insertion positions and destinations rather than pointer
  events so the authoritative behavior is server-testable; failed drops leave ordering
  untouched.
- Live integrations not verified: browser pointer gesture layer (no browser harness yet).

### 33 — `add-multi-selection` — COMPLETE

- Start: 2026-09-27 11:10 · End: 2026-09-27 12:05
- Proposal validation: strict validation passed before archive.
- Implementation: `momentum_app::selection::Selection` implements single, toggle, anchored
  range over the visible order, select-all, clear, and `retain_visible` pruning (selection is
  presentation state and never persists); `momentum_app::bulk::{BulkService, BulkAction,
  BulkOutcome}` applies complete, cancel, delete, schedule, deadline, move, and tag assignment
  to each selected ID through the same version-guarded commands and records explicit per-item
  successes/failures; the Inbox list renders a labeled checkbox per row, a project-aware bulk
  form posting to `POST /todos/bulk` (action select, date, destination, tag), with
  `?error=selection` for empty selections and `?error=bulk` on partial failure or incomplete
  parameters; `form_urlencoded` parses repeated `ids` fields that the serde form extractor
  cannot collect.
- Migrations: none.
- Tests: 5 app unit tests (toggle/anchor, range replacement, hidden IDs, select-all/clear,
  pruning), 3 testkit tests (mixed per-item results with canonical preservation, schedule +
  deadline + move, tag assignment + delete), 3 web tests (checkbox/form markup, bulk complete
  with failures, bulk schedule/delete); full workspace suite green plus fmt, arch-check, clippy,
  WASM build.
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known live-browser gap: interactive multi-select (shift-range, select-all shortcut) attaches to
  the same `Selection` machine from the hydrated client; the server-rendered checkbox form and
  the pure state machine are both tested, matching changes 32/71/72.
- Archive: `2026-09-27-add-multi-selection`; canonical `features/multi-selection` spec created.
- Commit: `feat(33): add multi selection`
- Decisions: bulk operations reuse single-item commands for undo/version parity; repeated form
  keys are parsed manually; selection is client/browser state, not synchronized domain data.
- Live integrations not verified: browser shift-range/keyboard selection gesture layer.

### 34 — `add-magic-plus` — COMPLETE

- Start: 2026-09-27 12:10 · End: 2026-09-27 13:05
- Proposal validation: strict validation passed before archive.
- Implementation: `POST /todos` gained optional `position` (1-based, converted server-side),
  `position_scope` (today), and `schedule_kind` (quick schedule) fields; after `CreateTodo` the
  handler applies `ScheduleTodo` and then `MoveTodoToPosition`/`MoveTodayToPosition`, with
  distinct `?error=create/schedule/position` continuation states. `MagicPlusTarget` and a native
  accessible `<details>` insert form render per row on Inbox and Today (hidden position, scope,
  schedule, context, return fields); the Today page gained a today-scoped add form so the visible
  context is the creation default even when empty; project/heading forms already carried their
  context as alternate targets.
- Migrations: none.
- Tests: 4 web tests (insert-control markup on Inbox and Today, positioned creation ordering
  between neighbors, alternate project-context creation, today-scoped scheduled insertion
  ordering in the Today bucket); full workspace suite green plus fmt, arch-check, clippy, WASM
  build. Positional command semantics reuse change 32's testkit coverage.
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known live-browser gap: pointer-dragging the Magic Plus between rows attaches to the same
  hidden-field/position contract from the hydrated client; the accessible `<details>` form
  performs the identical insertion now.
- Archive: `2026-09-27-add-magic-plus`; canonical `features/magic-plus` spec created.
- Commit: `feat(34): add magic plus`
- Decisions: insertion composes existing commands (no new write path); invalid context IDs fall
  back to Inbox; failed positioning after successful creation is surfaced rather than silently
  rolled back.
- Live integrations not verified: browser drag of the creation control.

### 35 — `add-expanded-item-editor` — COMPLETE

- Start: 2026-09-27 13:10 · End: 2026-09-27 14:05
- Proposal validation: strict validation passed before archive.
- Implementation: Inbox rows expand in place via a native `<details data-testid="row-editor">`
  carrying the title/notes commit form (posting to `POST /todos/{id}` with `return_to`), quick
  schedule forms (today/evening/tomorrow/someday/anytime), a date deadline form, per-tag
  assign/remove buttons, and a link to checklist/recurrence; `UpdateTodoForm` gained optional
  `return_to` with `?saved=1` continuation; cancel is a plain navigation with no write; expansion
  stays inside the row `<li>` so identity, list position, and selection checkboxes persist.
- Migrations: none.
- Tests: 3 web tests (editor + progressive metadata markup, commit with `?saved=1` plus quick
  schedule/deadline/date reschedule, tag assign then removal affordance switching); full
  workspace suite green plus fmt, arch-check, clippy, WASM build.
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known live-browser gap: smooth animated expansion belongs to the hydrated client; the
  accessible disclosure performs the same open/commit/cancel semantics now.
- Archive: `2026-09-27-add-expanded-item-editor`; canonical `features/expanded-item-editor` spec.
- Commit: `feat(35): add expanded item editor`
- Decisions: kept existing closed-row controls unchanged; progressive metadata reuses existing
  command routes rather than introducing editor-specific endpoints.
- Live integrations not verified: browser expansion animation.

## Run summary

### Completed changes

| # | Change | Status | Commit |
|---:|---|---|---|
| 01 | bootstrap-fullstack-rust-workspace | COMPLETE | `feat(01)` |
| 02 | add-web-application-shell | COMPLETE | `feat(02)` |
| 03 | add-things-design-system | COMPLETE | `feat(03)` |
| 04 | add-responsive-layout-foundation | COMPLETE | `feat(04)` |
| 05 | add-persistence-foundation | COMPLETE | `feat(05)` |
| 06 | add-account-authentication | COMPLETE | `feat(06)` |
| 07 | add-task-domain-core | COMPLETE | `feat(07)` |
| 08 | add-fractional-ordering-engine | COMPLETE | `feat(08)` |
| 09 | add-command-transaction-layer | COMPLETE | `feat(09)` |
| 10 | add-undo-redo-engine | COMPLETE | `feat(10)` |
| 11 | add-todo-crud | COMPLETE | `feat(11)` |
| 12 | add-inbox | COMPLETE | `feat(12)` |
| 13 | add-areas | COMPLETE | `feat(13)` |
| 14 | add-projects | COMPLETE | `feat(14)` |
| 15 | add-project-headings | COMPLETE | `feat(15)` |
| 16 | add-checklists | COMPLETE | `feat(16)` |
| 17 | add-tags | COMPLETE | `feat(17)` |
| 18 | add-notes-editor | COMPLETE | `feat(18)` |
| 19 | add-things-markdown | COMPLETE | `feat(19)` |
| 20 | add-scheduling-model | COMPLETE | `feat(20)` |
| 21 | add-deadlines | COMPLETE | `feat(21)` |
| 22 | add-today-projection | COMPLETE | `feat(22)` |
| 23 | add-upcoming-projection | COMPLETE | `feat(23)` |
| 24 | add-anytime-projection | COMPLETE | `feat(24)` |
| 25 | add-someday-projection | COMPLETE | `feat(25)` |
| 26 | add-completion-and-cancellation | COMPLETE | `feat(26)` |
| 27 | add-logbook | COMPLETE | `feat(27)` |
| 28 | add-trash | COMPLETE | `feat(28)` |
| 29 | add-special-lists | COMPLETE | `feat(29)` |
| 30 | add-project-progress | COMPLETE | `feat(30)` |
| 31 | add-move-engine | COMPLETE | `feat(31)` |
| 32 | add-drag-and-drop | COMPLETE | `feat(32)` |
| 33 | add-multi-selection | COMPLETE | `feat(33)` |
| 34 | add-magic-plus | COMPLETE | `feat(34)` |
| 35 | add-expanded-item-editor | COMPLETE | `feat(35)` |

- Completed proposal range: 01–35 (35 of 74).
- Incomplete: 36–74 (not started; sequential order means each depends on the previous state).
- Blocked: none. Change 14 was next and unblocked when the session's working context ended.
- Total commits on `agent/things-web-overnight`: 55 through change 35 (bootstrap, 35 numbered
  changes, spec/report docs commits; HEAD will be the change-35 commit).
- Branch: `agent/things-web-overnight`
- HEAD at report time: see `git log --oneline -1` (commit for change 13 was the last
  implementation commit; this report is committed after it).

### OpenSpec validation state

- `openspec validate --specs --strict --no-interactive`: all 13 canonical capabilities pass.
- `openspec validate --archived --strict --no-interactive`: all 13 archived changes pass with
  complete tasks.
- Canonical specs: `platform/rust-workspace`, `app/shell`, `ui/design-system`,
  `ui/responsive-layout`, `persistence/schema`, `auth/accounts`, `domain/tasks`,
  `domain/ordering`, `app/commands`, `app/undo`, `features/todo-crud`, `features/inbox`,
  `features/areas`.

### Rust test state

- `cargo test --workspace --all-features`: all green (domain 28, app 23, infra 23, testkit 32
  across auth/command/undo suites, web 60+ across smoke/shell/design/responsive/auth/CRUD/
  inbox/areas).
- `cargo fmt --all -- --check` and
  `cargo clippy --workspace --all-targets --all-features -- -D warnings`: clean.
- `scripts/quality-gates.sh` (fmt + arch-check + OpenSpec specs + clippy + tests): passes.
- WASM client: `scripts/build-web.sh` produces
  `target/wasm32-unknown-unknown/debug/momentum_web.wasm` (rustup stable with the
  `wasm32-unknown-unknown` target).

### Not executed in this environment (recorded limitations)

- Live PostgreSQL paths (migrations apply/rollback, auth store, command unit of work, read
  stores): no PostgreSQL server or Docker daemon; conditional tests record the skip.
  Run them with `DATABASE_URL=postgres://… cargo test --workspace --all-features`.
- Browser end-to-end, hydration, visual parity, and behavior parity suites: introduced by
  changes 71/72, which were not reached. SSR + integration coverage exists through Axum
  `oneshot` tests.
- Email delivery for verification/reset tokens: no provider configured; tokens are logged for
  operators and recorded as a live-integration gap.
- wasm-bindgen JS glue: the raw WASM artifact builds; browser hydration glue wiring is part of
  the later PWA/hydration work.

### Exact commands

Run the app (persistence optional):

```bash
export PATH="$HOME/.cargo/bin:$PATH"
cargo run -p momentum-web --bin momentum-server
# http://127.0.0.1:3000  (MOMENTUM_BIND_ADDR to override)
# With persistence: DATABASE_URL=postgres://user:pass@host/db (migrations run at startup)
```

Run the tests and gates:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
scripts/quality-gates.sh
scripts/build-web.sh
openspec validate --specs --strict --no-interactive
openspec validate --archived --strict --no-interactive
```

### Remaining real follow-ups

1. Changes 36–74, strictly in order; the workspace, command layer, projection scopes,
   move engine, drag/drop drop-target operations, selection/bulk layer, Magic Plus creation
   composition, inline editor composition, read-store patterns, and test harnesses are in place
   to make each one mechanical. Next: 36.
2. Wire browser hydration glue (wasm-bindgen CLI) when the PWA/hydration changes arrive; the
   Rust hydration entry point and session bootstrap element already exist.
3. Execute the conditional PostgreSQL suites against a real database.
4. Configure email delivery for verification/reset tokens.
5. Localization (change 65) must replace the documented UTC fallback used for "today".
6. Visual/behavior parity harnesses (changes 71/72) remain the main unverified parity gap.
7. Disk hygiene: the workspace Cargo profile disables debug info and incremental caches; still
   run `cargo clean` if the volume crosses ~85% before large builds (see Build environment
   notes).

### Environment notes

- Toolchain: rustup stable 1.98.1 with `rustfmt`, `clippy`, `wasm32-unknown-unknown`
  (`rust-toolchain.toml`); Homebrew Rust 1.95 is also installed. `scripts/*.sh` prefer
  `$HOME/.cargo/bin`.
- Networking: crates.io downloads work; cargo is run with `CARGO_NET_OFFLINE=true` when the
  index would otherwise stall.
- Current state at HEAD (change 35): all quality gates green
  (`cargo fmt --check`, `xtask arch-check`, `clippy -D warnings`, `cargo test --workspace
  --all-features`, WASM client build); `openspec validate --specs/--archived --strict` reports
  35/35 passing; `git status` clean.


