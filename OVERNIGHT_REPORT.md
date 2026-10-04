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

### 36 — `add-date-pickers` — COMPLETE

- Start: 2026-09-27 14:10 · End: 2026-09-27 15:05
- Proposal validation: strict validation passed before archive.
- Implementation: domain gained `parse_local_datetime` and `local_datetime_to_utc` (chrono-tz;
  typed errors for invalid zones and daylight-saving gaps, ambiguous times resolve to the earliest
  instant); `SetTodoReminder` now rejects attaching a reminder without a date-based schedule
  (clearing stays allowed); new `POST /todos/{id}/reminder` maps local time + IANA timezone to a
  UTC `ReminderInput` with `?error=reminder` continuation; the expanded row editor gained a When
  date picker, deadline set/clear, reminder set/clear (labeled datetime-local + timezone inputs),
  and rows show a reminder badge with the stored UTC instant.
- Migrations: none.
- Tests: 2 domain unit tests (conversion, DST gap/invalid zone/format), 2 testkit tests
  (schedule-gated reminder attach/clear with timezone round-trip; deadline independence), 3 web
  tests (picker markup, independent schedule/deadline/reminder commits with UTC conversion and
  badge, schedule-gate plus invalid-zone rejection); full workspace suite green.
- Fix: expanded-editor view depth exceeded test-thread stacks once the pickers landed; boxing the
  editor and Magic Plus views with `.into_any()` restores safe SSR stack usage (relevant to
  production 2 MB worker stacks too).
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known live-browser gap: native browser date/time pickers attach to these labeled inputs in the
  hydrated client; the server-rendered inputs are fully functional without script.
- Archive: `2026-09-27-add-date-pickers`; canonical `features/date-pickers` spec created.
- Commit: `feat(36): add date pickers`
- Decisions: explicit timezone carried by the form (preferences arrive with localization);
  quick destinations reuse the schedule route; reminder clearing never gated.
- Live integrations not verified: browser-native picker widgets; reminder delivery.

### 37 — `add-natural-language-dates` — COMPLETE

- Start: 2026-09-27 15:10 · End: 2026-09-27 16:05
- Proposal validation: strict validation passed before archive.
- Implementation: `momentum_domain::natural_date` provides `parse_natural_date(expression, base,
  Option<Locale>)` with relative keywords (today/tomorrow/tonight/next week), offsets (`in N
  days/weeks/months`, `N days from now`), upcoming vs. following-week weekdays, named month/day
  forms with ordinal stripping and year roll-forward, locale-resolved numeric dates
  (`Locale::EnUs`/`EnGb`), reminder time extraction (`9:30am`, `21:00`, `tonight` → 19:00), and a
  typed error set including `Ambiguous { candidates }` when no locale is supplied; the schedule
  route accepts `when=natural&text=...` and the row editor renders a labeled Natural date input.
- Migrations: none.
- Tests: 6 domain unit tests (keywords/offsets, weekdays, named months, locale + ambiguity,
  reminder times, recoverable errors) and 1 web test (input markup, `tomorrow` and `jun 20`
  commits, unknown-expression rejection preserving schedule); full workspace suite green.
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known live-browser gap: inline suggestion chips for parsed dates belong to the hydrated client;
  the input and parser are fully deterministic without script.
- Archive: `2026-09-27-add-natural-language-dates`; canonical `features/natural-language-dates`
  spec created.
- Commit: `feat(37): add natural language dates`
- Decisions: en-US locale default at the call site until account preferences exist; only the date
  part schedules (parsed reminder times are exposed for later consumption); past explicit dates
  roll forward to the next year.
- Live integrations not verified: none beyond the documented hydration gap.

### 38 — `add-reminders` — COMPLETE

- Start: 2026-09-27 16:10 · End: 2026-09-27 18:00
- Proposal validation: strict validation passed before archive.
- Implementation: domain gained `Reminder::is_due` and idempotent `mark_delivered` plus
  `Todo::mark_reminder_delivered`; new undo-inert `MarkTodoReminderDelivered` command and
  `TodoService::mark_reminder_delivered`; `momentum_app::reminders` provides `ReminderScheduler`
  over `ReminderSource`/`ReminderNotifier` traits with per-item results (failures stay scheduled
  for retry; restarts and repeat passes are no-ops because delivered state persists in the
  reminder JSON), `PushSubscription{Store}`, `build_push_notification` deep links, a fan-out
  `WebPushNotifier` over a `PushTransport`, and an in-process `MemoryPushSubscriptionStore`;
  `momentum_infra::PgReminderSource` reads due reminders from the entities table filtering in
  Rust; `POST /push/subscriptions` registers/refreshes endpoints via a new `with_push_subscriptions`
  AppState builder.
- Migrations: none (delivery state lives in `reminder_rule`).
- Tests: 1 domain unit test (due/idempotence), 3 testkit tests (deliver once and restart no-op,
  failure retry with state preserved, push fan-out with deep-link and JSON escaping plus endpoint
  refresh), 2 web tests (registration/refresh, missing-store error); full workspace suite green
  (69 suites) after re-running the killed gate; fmt/arch-check/clippy/openspec strict validation
  green before the interruption.
- Interruption note: the first gate run was terminated during `cargo test` compilation by an
  external SIGTERM; a clean re-run of the full workspace suite passed (EXIT=0).
- Follow-up fix: the first WASM build failed because `push_routes` was declared without an
  `ssr` feature gate (axum is optional); the module is now `#[cfg(feature = "ssr")]` and the WASM
  client builds cleanly. The failed build was masked by piping the build script to `tail` in the
  commit chain; future runs must check the script exit status directly.
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known live gap: the production `PushTransport` network adapter (VAPID/HTTP to a push service)
  and the browser service-worker subscription flow belong to the hydrated client and deployment
  configuration; subscription persistence, scheduling, payloads, and fan-out are implemented and
  tested behind traits.
- Archive: `2026-09-28-add-reminders`; canonical `features/reminder-delivery` spec created.
- Commit: `feat(38): add reminders`
- Decisions: at-least-once delivery with persisted state preventing storms; PG due filtering in
  Rust to avoid JSON SQL portability; manual JSON escaping since serde_json is feature-gated in
  the app crate.
- Live integrations not verified: real push service delivery; service-worker registration.

### 39 — `add-reminder-snoozing` — COMPLETE

- Start: 2026-09-27 18:10 · End: 2026-09-27 19:05
- Proposal validation: strict validation passed before archive.
- Implementation: the reminder state machine now treats `Snoozed` as deliverable and marks
  snoozed reminders delivered again when they fire; `Reminder::snooze(until)` transitions from
  Delivered/Snoozed and is a no-op for a repeated target (no duplicate deliveries, no version
  churn); `SnoozeChoice::{TenMinutes,OneHour,Tomorrow}` with `resolve_snooze`; new
  `SnoozeTodoReminder` command and `TodoService::snooze` (loads the reminder, resolves via the
  service clock, dispatches); `momentum_domain::time::next_local_day_at` computes the next local
  calendar day at the reminder's wall-clock time in its IANA timezone and steps over DST gaps;
  rows with delivered/snoozed reminders render labeled 10m/1h/Tomorrow controls posting to
  `POST /todos/{id}/snooze` with `?error=snooze` continuation.
- Migrations: none.
- Tests: domain unit tests (next-local-day normal/DST-gap/invalid zone; snooze due/idempotence),
  3 testkit tests (idempotent repeat with unchanged version, redelivery exactly at the snoozed
  instant and not after, local "tomorrow" resolving to a 43.5-hour offset rather than +24h,
  missing-reminder rejection), 2 web tests (delivered-only controls and choice routing incl.
  invalid option, missing reminder error path); full workspace suite green plus fmt, arch-check,
  clippy, WASM build (exit status checked directly after the change-38 masking lesson).
- Bug found and fixed during the change: `MarkTodoReminderDelivered` initially only accepted
  `Scheduled`, so a redelivered snoozed reminder was re-notified every pass; `mark_delivered` now
  accepts `Snoozed` (caught by the new testkit redelivery test).
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known live gap: notification action buttons (snooze from the OS notification) belong to the
  hydrated client; the web row controls are the accessible surface.
- Archive: `2026-09-28-add-reminder-snoozing`; canonical `features/reminder-snoozing` spec created.
- Commit: `feat(39): add reminder snoozing`
- Decisions: "tomorrow" keeps the reminder's own wall-clock time on the next local day; snooze
  transitions only from delivered/snoozed; repeated same-target snoozes are true no-ops.
- Live integrations not verified: notification-button snoozing.

### 40 — `add-recurrence-model` — COMPLETE

- Start: 2026-09-27 19:10 · End: 2026-09-27 19:45
- Proposal validation: strict validation passed before archive.
- Implementation: `momentum_domain::recurrence` provides `RecurrencePattern` (Daily; Weekly with
  selected weekdays as days-from-Monday; Monthly with selected day numbers searching forward up
  to 400 days; Yearly with leap-day clamping; AfterCompletion delays),
  `Recurrence::{new,next_occurrence}` with `RecurrenceEnd::{Never,On,After}` evaluated against the
  occurrence count, and `RecurrenceTemplate` owning title, notes, tags, checklist, heading
  structure, deadline/reminder offsets, and context with `deadline_for` and `copy_for` producing
  independent values.
- Migrations: none (pure domain change; persistence arrives with copy-on-complete).
- Tests: 4 domain unit tests (all patterns with leap-year clamping, after-completion, end
  conditions, template copy independence) and 2 testkit integration tests (rules through the
  public API, template materialization); full workspace suite green plus fmt, arch-check,
  clippy, WASM build (exit checked directly).
- Note: weekdays are `BTreeSet<u32>` days-from-Monday because `chrono::Weekday` lacks `Ord`.
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Archive: `2026-09-28-add-recurrence-model`; canonical `features/recurrence-model` spec created.
- Commit: `feat(40): add recurrence model`
- Decisions: after-completion patterns advance from the completion base; end bounds are
  count-based and date-based; templates are plain owned values.
- Live integrations not verified: none (domain-only change).

### 41 — `add-recurrence-materializer` — COMPLETE

- Start: 2026-09-27 19:50 · End: 2026-09-27 21:20
- Proposal validation: strict validation passed before archive.
- Implementation: recurrence domain types gained serde; `UnitOfWork` gained
  load/exists/record recurrence methods (PG + memory); new idempotent
  `MaterializeRecurrenceOccurrence` command creates the copy inside the dispatcher transaction
  (title, notes, context, occurrence schedule, deadline offset, checklist items, context rank)
  with the `(template_id, occurrence_key)` guard — a conflicting insert rolls the copy back, so a
  concurrent loser cannot duplicate; `momentum_app::materializer` provides `MaterializerStore`,
  `PersistedTemplate`, `RecurrenceMaterializer` with `HORIZON_DAYS = 30` and
  `MAX_OCCURRENCES_PER_RUN = 90`, applying template tags through the tag service and persisting
  `next_occurrence`/`horizon` after each run; `PgMaterializerStore` and memory
  `MemoryMaterializerStore` implement the store; progress lives in `recurrence_templates`
  (schema already had both tables, so no migration).
- Migrations: none.
- Tests: 2 testkit tests (bounded 31-occurrence horizon with template inheritance incl. notes,
  checklist, deadline offset, schedule, then repeat-run no-op, persisted progress, restart
  continuing from stored state; end conditions stopping after the count and pre-existing
  occurrence claims skipped without duplicates); full workspace suite green plus fmt, arch-check,
  clippy, WASM build (exit checked directly).
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Archive: `2026-09-28-add-recurrence-materializer`; canonical `features/recurrence-materializer`
  spec created.
- Commit: `feat(41): add recurrence materializer`
- Decisions: the composite primary key is the concurrency guard (no advisory locks); progress is
  `next_occurrence` + `horizon_date` on the template row; occurrences keyed by ISO date.
- Live integrations not verified: template creation UI/commands (later changes).

### 42 — `add-recurrence-exceptions` — COMPLETE

- Start: 2026-09-28 09:00 · End: 2026-09-28 11:05
- Proposal validation: strict validation passed before archive. (The first gate run was interrupted
  externally; a clean re-run passed all gates, EXIT=0.)
- Implementation: `MaterializerStore` now exposes `active_templates` and `TemplateState`
  (open copy, last completion, copy count) with PG join SQL and memory snapshotting; the
  materializer pauses after-completion templates while a copy is open, anchors the next copy to
  the latest completion date (first copy at today), publishes one at a time, and enforces
  count/date bounds; regular rules realign to rule-matching dates after resets
  (`next_after(date - 1) == date`); new `UpdateRecurrenceRule` command + `TodoService` method + 
  unit-of-work rule persistence (PG update resets `next_occurrence` to today and clears the
  horizon; memory mirrors) distinguishes future-rule edits from per-copy one-time edits.
- Migrations: none.
- Tests: 2 new testkit tests (after-completion pausing while open, completion-anchored next
  occurrence, early completion anchoring day 4 rather than day 6; one-time copy reschedule
  leaving template progress untouched, then rule replacement regenerating on-pattern Mondays
  while existing copies stay unchanged) plus the existing materializer/model suites; full
  workspace suite green plus fmt, arch-check, clippy, WASM build (exit checked directly).
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Archive: `2026-09-28-add-recurrence-exceptions`; canonical `features/recurrence-exceptions`
  spec created.
- Commit: `feat(42): add recurrence exceptions`
- Decisions: date-level anchoring via the completion timestamp's UTC date; rule edits reset
  progress so only future generation changes; one-time edits reuse existing copy commands.
- Live integrations not verified: recurrence editing UI (later changes).

### 43 — `add-quick-find` — COMPLETE

- Start: 2026-09-28 11:10 · End: 2026-09-28 13:05
- Proposal validation: strict validation passed before archive.
- Implementation: `momentum_app::search` provides `SearchKind`, `SearchHit`, `ScoredHit`,
  `SearchService` (static list/settings destinations plus areas, projects, headings from project
  summaries, tags, and `AllOpen` to-do titles), four confidence tiers (exact/prefix/word-start/
  substring) with deterministic tie-breaks by recency, kind, title, and URL, capped at 60 and
  never hiding matches; `RecentsStore` + bounded `MemoryRecentsStore` record visits via
  `GET /search/go` (internal URLs only) and boost equal-tier hits; `/search` page with a labeled
  autofocus form, kind/score data attributes and Recent badges; topbar Quick Find form on every
  page; `AppState::with_recents` provides the store to the Leptos context and the axum handler.
- Migrations: none (recents are process-local navigation state).
- Tests: 1 app unit test (tier scoring incl. word-split and substring cases) and 2 web tests
  (exact-before-prefix-before-word ordering with project/tag/todo mixes, static destinations,
  empty query, topbar presence; recents recording via go, external-URL rejection, and the
  recency tie-break badge); full workspace suite green plus fmt, arch-check, clippy, WASM build
  (exit checked directly). Clippy findings (`iter().any` → `contains`, unused import) fixed.
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known live-browser gap: the type-to-open keyboard gesture (first keystroke focuses Quick Find
  seeded with that character) attaches to this page and input from the hydrated Rust/WASM client,
  consistent with changes 32/33/71/72; the page itself is fully functional without script.
- Archive: `2026-09-28-add-quick-find`; canonical `features/quick-find` spec created.
- Commit: `feat(43): add quick find`
- Decisions: recents are in-process because no durable preference store exists yet; headings
  reuse project summaries; recency only breaks ties within a tier.
- Live integrations not verified: hydrated type-to-open gesture.

### 44 — `add-deep-search` — COMPLETE

- Start: 2026-09-28 13:10 · End: 2026-09-28 14:20
- Proposal validation: strict validation passed before archive.
- Implementation: search now considers to-do notes, every checklist item title, project notes, and
  logged/completed content via `TodoScope::Logbook { logged_only: false }`; `SearchField`
  (title < checklist < notes) breaks ties inside a confidence tier and `data-matched-in` plus a
  "Notes"/"Checklist" badge surface it; one row per destination keeps the best-matching field;
  `normalize` folds case and Latin diacritics (shared by server search and the future hydrated
  client for offline/server parity) while display text stays original.
- Migrations: none.
- Tests: 2 app unit tests (tiers incl. word-split/substring; normalization and diacritic queries)
  and 1 new web test (notes match via "röadmap", checklist match via "cafe" with badge, logged
  completed content via "errand", title match ranking above notes match); full workspace suite
  green plus fmt, arch-check, clippy, WASM build (exit checked directly). Clippy unused-import
  fixed.
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known live gap: the persisted offline index does not exist yet; the shared `normalize`/scoring
  API is the compatibility contract, exercised server-side today.
- Archive: `2026-09-29-add-deep-search`; canonical `features/deep-search` spec created.
- Commit: `feat(44): add deep search`
- Decisions: hand-rolled Latin diacritic folding (no Unicode dependency); best-field-per-URL
  dedup; field priority sorts between score and recency.
- Live integrations not verified: offline index implementation.

### 45 — `add-keyboard-command-system` — COMPLETE

- Start: 2026-09-28 14:30 · End: 2026-09-28 15:35
- Proposal validation: strict validation passed before archive.
- Implementation: `momentum_app::keyboard` declares 24 `CommandDefinition`s across creation,
  status, scheduling, navigation, organization, selection, and ordering with surfaces and
  conflict metadata; helpers `registry`, `command`, `keys_for` (resolves reserved bindings to
  their safe alternative), `commands_for(surface)`, and `conflicts`; reserved mappings documented
  (Cmd+K → `/`, Cmd+Backspace → Shift+Delete, Cmd+E → F2, Cmd+A → Shift+A); the Settings page
  renders the whole reference grouped by category with `<kbd>` keys and reserved badges; Inbox
  row Complete/Delete controls carry `aria-keyshortcuts` resolved through `keys_for`.
- Migrations: none.
- Tests: 3 app unit tests (unique ids/bindings, conflict alternatives, category coverage and
  surface filtering) and 2 web tests (Settings reference content incl. the Quick Find conflict
  note, `aria-keyshortcuts` on row controls); full workspace suite green plus fmt, arch-check,
  clippy, WASM build (exit checked directly). The now-unused generic `ListPage`/macro was removed
  after Settings became bespoke (clippy).
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known live gap: the key-event dispatch loop itself is the hydrated Rust/WASM client's job; it
  consumes this registry via `keys_for`/`commands_for`.
- Archive: `2026-09-29-add-keyboard-command-system`; canonical `features/keyboard-command-system`
  spec created.
- Commit: `feat(45): add keyboard command system`
- Decisions: the registry is compile-time data shared by SSR hints and the hydrated client;
  reserved commands keep their conventional primary key for documentation only.
- Live integrations not verified: hydrated key dispatch.

### 46 — `add-type-travel` — COMPLETE

- Start: 2026-09-28 15:45 · End: 2026-09-28 17:20
- Proposal validation: strict validation passed before archive.
- Implementation: `keyboard::type_travel_seed(key, ctrl, meta, alt, editing)` is the pure decision
  (printable single characters only, no modifier chords, never while an editable control is
  focused); the authenticated layout installs a document-level keydown listener that inspects
  `document.active_element()` (input/textarea/select/contenteditable) and navigates to
  `/search?q=<seed>`; Escape on the Quick Find input steps back in history. The listener is
  compiled only for `all(feature = "hydrate", target_arch = "wasm32")`.
- Migrations: none.
- Tests: 1 new app unit test covering the decision matrix (printable, accented, editing,
  modifiers, multi-character, control, whitespace, empty) plus the existing keyboard registry
  suite; full workspace suite green plus fmt, arch-check, clippy, WASM build (exit checked
  directly).
- Debugging note: the first wiring used `cfg(feature = "hydrate")` alone, which made
  `--all-features` native test builds run hydrated router code and panic
  (`spawn_local` outside a LocalSet) in the anytime suite; gating on `target_arch = "wasm32"` as
  well fixed it. The full all-features suite is green again.
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Archive: `2026-09-29-add-type-travel`; canonical `features/type-travel` spec created.
- Commit: `feat(46): add type travel`
- Decisions: document-level listener (key events on body focus do not bubble through the layout
  div); a leaked closure is intentional for the app-lifetime layout; Escape restores via
  `history.back()`.
- Live integrations not verified: none beyond the browser runtime itself, exercised by the wasm
  build.

### 47 — `add-mobile-gestures` — COMPLETE

- Start: 2026-09-28 17:30 · End: 2026-09-28 19:30
- Proposal validation: strict validation passed before archive.
- Implementation: `momentum_app::gestures` provides thresholds (`SWIPE_THRESHOLD` 60 px,
  `LONG_PRESS_MS` 500 ms, `PULL_THRESHOLD` 80 px, `GESTURE_MAX_MS` 800 ms, `LONG_PRESS_SLOP`
  10 px), `classify_touch` (long press within slop → Open; fast mostly-horizontal swipe →
  configured left/right action; otherwise None so scrolling/tapping wins), a `SwipeConfiguration`
  with the default (left selects, right opens When), and `pull_to_search` (top boundary +
  downward + fast + mostly vertical); the authenticated layout installs document-level
  touchstart/touchend listeners in the hydrate+wasm effect (position/timestamp/top-boundary state
  in an `Rc<RefCell<_>>`, navigation to Quick Find on a qualifying pull).
- Migrations: none.
- Tests: 3 app unit tests covering the full matrix (swipe direction/threshold/slow/vertical,
  long-press slop and duration plus swipe-after-press, pull boundary/distance/duration/angle);
  full workspace suite green plus fmt, arch-check, clippy, WASM build (exit checked directly).
- Debugging notes: view-macro event closures cannot carry hydrate-only DOM code safely, so pull
  wiring moved into the existing hydrate effect; touch APIs required a direct `web-sys`
  dependency with `Touch`/`TouchList`/`TouchEvent` features enabled under the `hydrate` feature
  (not ssr); `Touch::client_y()` is `i32` and is converted to `f64`; the `navigate` handle is
  cloned per closure.
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known live gap: row-level swipe/long-press dispatch consumes the exposed configuration and
  classifiers from the hydrated client; drop targets, selection, and accessible equivalents are
  already server-rendered and tested (changes 32/33).
- Archive: `2026-09-29-add-mobile-gestures`; canonical `features/mobile-gestures` spec created.
- Commit: `feat(47): add mobile gestures`
- Decisions: recognition is pure arithmetic in the app crate; pull requires the top boundary to
  avoid hijacking normal scroll-up; long press winner-takes-all over swipe only within slop.
- Live integrations not verified: row-level touch dispatch.

### 48 — `add-calendar-integration` — COMPLETE

- Start: 2026-09-28 19:40 · End: 2026-09-28 21:20
- Proposal validation: strict validation passed before archive.
- Implementation: `momentum_app::calendar` provides `CalendarEvent` (always read-only),
  `CalendarRange`, `CalendarProvider` trait, typed `CalendarError` with user-facing degraded
  reasons, `CalendarService` with an in-memory per-range cache that degrades to cached events on
  failure (`degraded`/`stale` flags), and `FlakyCalendarProvider` for failure injection;
  `CalendarRow::from_event` and `UpcomingAgenda::with_calendar_rows` integrate the projection;
  Today gained a `CalendarSection` for today's range and Upcoming renders merged calendar rows
  plus a degraded banner; `AppState::with_calendar` provides the service to context and the
  Leptos pages; the testkit gained `FakeCalendarProvider`.
- Migrations: none (cache is process-local, never authoritative).
- Tests: 3 testkit tests (range filtering with read-only flags, degradation to cache plus
  recovery without corruption, failure-without-cache non-blocking) and 3 web tests (Today rows
  with `data-external`, Upcoming rows plus degraded banner after auth expiry, task data surviving
  a network failure); full workspace suite green plus fmt, arch-check, clippy, WASM build (exit
  checked directly).
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known gap: no live Google/Microsoft/CalDAV network adapter or OAuth flow (no credential store
  or HTTP client policy exists yet); adapters implement `CalendarProvider` without core changes.
- Archive: `2026-09-29-add-calendar-integration`; canonical `features/calendar-integration` spec
  created.
- Commit: `feat(48): add calendar integration`
- Decisions: events are read-only by design; degradation never clears the cache; the provider
  contract is the only calendar dependency in core code.
- Live integrations not verified: real provider network calls.

### 49 — `add-offline-local-replica` — COMPLETE

- Start: 2026-09-28 21:30 · End: 2026-09-28 23:10
- Proposal validation: strict validation passed before archive. (A first gate run was aborted by
  the operator; the clean re-run passed all gates, EXIT=0.)
- Implementation: `momentum_app::replica` provides `ReplicaSnapshot` (entities, preferences,
  sync cursor), `ReplicaEntity`, `OutboxEntry` (stable UUIDv7 operation id, serialized command,
  attempts), the `ReplicaStore` persistence port with `MemoryReplicaStore`, and
  `OfflineCoordinator` (`startup` offline restore; `apply_locally` serializes a command and
  enqueues it before acknowledgement; `record_entity` updates the optimistic working set;
  `acknowledge` removes delivered operations; `replay` walks in order, stops at the first
  retryable/offline failure with attempt counts, and is idempotent by operation id); the layout
  shows an outbox-pending indicator via `AppState::with_replica`; `serde_json` moved from
  dev-dependencies to dependencies in the app crate.
- Migrations: none (client-owned replica, never authoritative).
- Tests: 3 testkit tests (optimistic enqueue + reload restore of snapshot and outbox, ordered
  idempotent replay clearing operations, retry/offline stopping the pass with attempt counts then
  completing later) and 2 web tests (pending indicator with a configured replica, absence
  without); full workspace suite green plus fmt, arch-check, clippy, WASM build (exit checked
  directly).
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known gap: the concrete IndexedDB `ReplicaStore` implementation and the HTTP sync client are
  hydrated-client/deployment work — browser-only code that cannot run in the SSR harness; the
  port, coordinator semantics, and durability tests are complete so the adapter is a drop-in.
- Archive: `2026-09-29-add-offline-local-replica`; canonical `features/offline-local-replica`
  spec created.
- Commit: `feat(49): add offline local replica`
- Decisions: operation ids never change across replays; failures leave entries queued rather
  than dropping writes; the replica is client-owned with a sync cursor for later delta sync.
- Live integrations not verified: IndexedDB adapter; HTTP sync transport.

### 50 — `add-cloud-sync-protocol` — COMPLETE

- Start: 2026-09-29 09:00 · End: 2026-09-29 11:30
- Proposal validation: strict validation passed before archive.
- Implementation: `momentum_app::sync` provides `SyncRequest`/`SyncResponse`, `SyncOperation`
  (stable operation ids), `OperationAck` (sequence + replayed), `ChangeEntry`, `SyncConflict`
  with `ConflictRule`/`ConflictResolution`, the `ChangeLogReader` port, and `SyncService`
  (`sync` pushes through the dispatcher with client operation ids and pulls changes after the
  cursor); `classify_conflict` maps `Conflict` to FieldVersion/ServerWins or
  OrderingRelative/Ignored for ordering commands, `NotFound` to DeleteBeatsEdit/ClientRefreshes,
  and other errors to Rejected; a pre-check classifies edits to trashed entities before dispatch
  so tombs are never mutated; completion replay is an idempotent no-op acknowledged without
  conflict; `PgChangeLogReader` reads the existing `change_log`; testkit gained
  `MemoryChangeLogReader` and a `changes_after` accessor; JSON `POST /sync` is wired through
  `AppState::with_sync` and the server bootstrap.
- Migrations: none (`change_log` and its unique operation index already existed).
- Tests: 2 app unit tests (ordering recognition, conflict classification), 3 testkit tests
  (monotonic acknowledgements with operation-id dedupe and monotonic cursor, cursor-bounded
  pulls, monotonic completion + delete-beats-edit + ordering-relative conflicts without
  destructive writes), 2 web tests (JSON route push/pull, 503 without the service); full
  workspace suite green plus fmt, arch-check, clippy, WASM build (exit checked directly).
  Clippy unused-import fixed.
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known gap: hydrated client delta application/cache merge beyond the replica (49) is UI work.
- Archive: `2026-10-01-add-cloud-sync-protocol`; canonical `features/cloud-sync-protocol` spec
  created.
- Commit: `feat(50): add cloud sync protocol`
- Decisions: no generic LWW anywhere; trashed entities are pre-checked; cursors never regress;
  operation ids are the dedupe key on both push and replay.
- Live integrations not verified: real client transport loop.

### 51 — `add-realtime-sync` — COMPLETE

- Start: 2026-09-29 11:40 · End: 2026-09-29 13:30
- Proposal validation: strict validation passed before archive.
- Implementation: `momentum_app::realtime` provides `InvalidationHint`, the `HintSink`
  connection contract, `RecordingHintSink`, and a coalescing `HintHub` (account-scoped
  subscriptions; one pending hint per account; bursts merge to the newest sequence; no
  accumulation without subscribers; `delivered` unblocks the next emit); `SyncService::with_hints`
  publishes after every acknowledged push; `GET /sync/events` (axum `Sse`) subscribes the
  connection, emits an immediate connect hint for reconnect-safe pulls, streams coalesced
  `invalidation` events with keepalive, and unsubscribes on drop via an RAII guard;
  `AppState::with_hints` and the server bootstrap wire the hub (tokio `sync` feature and
  `tokio-stream` added).
- Migrations: none (hints are ephemeral; cursors remain authoritative).
- Tests: 2 app unit tests (coalescing until delivered with newest-sequence retention and
  unsubscribe; account scoping plus reconnect emission), a testkit assertion that a pushed batch
  emits exactly one coalesced hint carrying the first acknowledged sequence, and 2 web tests
  (SSE headers plus initial connect frame read from the body stream; 503 without the hub); full
  workspace suite green plus fmt, arch-check, clippy, WASM build (exit checked directly).
  Clippy map-values iteration fixed.
- Verification: manual artifact-to-implementation verification and record; every requirement
  scenario covered.
- Known gap: WebSocket transport is not built (SSE is the initial adapter; the hint contract is
  transport-agnostic) and client-side pull scheduling is hydrated-client work.
- Archive: `2026-10-01-add-realtime-sync`; canonical `features/realtime-sync` spec created.
- Commit: `feat(51): add realtime sync`
- Decisions: hints are idempotent pull-now signals, never data; one pending hint per account;
  reconnect emits a connect hint so cursors reconcile.
- Live integrations not verified: browser EventSource loop.

### 52 — `add-multitab-coordination` — COMPLETE

- Start: 2026-09-29 13:40 · End: 2026-09-29 15:20
- Proposal validation: strict validation passed before archive.
- Implementation: `momentum_app::tabs` provides `TabId`, `TabMessage` (Committed,
  LeadershipClaimed, OutboxChanged), the `TabBroadcaster` port with a recording implementation,
  `LeadershipState` (lowest live lease leads; expired leases pruned), `TabCoordinator`
  (`claim_or_renew` claims/broadcasts when none is live, `on_message` adopts sibling claims and
  turns remote commits into local pulls, `propagate_committed`/`propagate_outbox` broadcast), and
  `HintHub::notify` for direct delivery bypassing local coalescing bookkeeping.
- Migrations: none (leases are in-memory views; outbox durability lives in the replica store).
- Tests: 2 app unit tests (election with lowest-live-id and expiry; claim adoption, takeover,
  hint delivery, outbox broadcasting) and 2 testkit tests (leader recovery after lease expiry
  preserving pending outbox operations from the shared replica store, with simulated
  BroadcastChannel delivery of the claim; sibling commit triggering a local pull and local
  propagation); full workspace suite green plus fmt, arch-check, clippy, WASM build (exit checked
  directly).
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known gap: the concrete BroadcastChannel/Web Locks adapter is hydrated-client work; the port
  and deterministic logic are complete, and Web Locks can replace the lease without observable
  changes.
- Archive: `2026-10-01-add-multitab-coordination`; canonical `features/multitab-coordination`
  spec created.
- Commit: `feat(52): add multitab coordination`
- Decisions: deterministic min-id election with lease expiry; remote commits always notify the
  local hint sink; leadership state never touches outbox data.
- Live integrations not verified: browser BroadcastChannel adapter.

### 53 — `add-quick-entry` — COMPLETE

- Start: 2026-09-29 15:30 · End: 2026-09-29 17:00
- Proposal validation: strict validation passed before archive.
- Implementation: a `#quick-entry` overlay (CSS `:target` open/close, no handwritten JS) lives in
  the authenticated layout with a topbar trigger carrying `aria-keyshortcuts` from the new
  registry command `quick_entry` (`q`); the form captures title, notes, when
  (none/today/evening/tomorrow/someday/anytime/on-date), schedule date, deadline, destination
  (Inbox or project), tag, and checklist (one item per line), seeds `return_to` from the current
  pathname, and cancels via `href="#"`; `POST /todos/quick` validates every field before
  creating, applies schedule/deadline/tag/checklist in order, and deletes the fresh item on any
  post-create failure (compensation) before redirecting with `?error=quick`.
- Migrations: none.
- Tests: 3 web tests (overlay/trigger markup on every page, full-metadata save verifying notes,
  schedule, deadline, project context, tag, and two checklist items, and two pre-create rejection
  paths proving no item is created for an invalid date or a tag request without a tag service);
  full workspace suite green plus fmt, arch-check, clippy, WASM build (exit checked directly).
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known gap: the hydrated client's shortcut handling opens the overlay; the registry binding and
  target overlay are ready for it.
- Archive: `2026-10-01-add-quick-entry`; canonical `features/quick-entry` spec created.
- Commit: `feat(53): add quick entry`
- Decisions: `:target` overlay avoids JS dialogs; metadata application is compensated rather
  than transactional across commands, guaranteeing no half-created item.
- Live integrations not verified: hydrated overlay open shortcut.

### 54 — `add-browser-capture` — COMPLETE

- Start: 2026-09-29 17:10 · End: 2026-09-29 18:40
- Proposal validation: strict validation passed before archive.
- Implementation: `momentum_app::capture` sanitizes untrusted shared data (`sanitize_url` keeps
  http/https only, strips controls, caps at 2048; `sanitize_text` removes controls, collapses
  whitespace, caps at 4000; `build_capture` falls back to "Captured page" and puts the URL first
  in notes); `GET/POST /capture` authenticates, sanitizes, creates the Inbox to-do, and redirects
  (`return_to` for POST adapters, `/login` without a session, `?error=capture` on write
  failure); a PWA manifest at `/manifest.webmanifest` declares `share_target` action `/capture`
  (GET, params title/text/url) and is linked from the shell — the documented command URL that an
  optional extension/bookmarklet uses without duplicating domain logic.
- Migrations: none.
- Tests: 3 app unit tests (URL allow-list incl. javascript:/data:/file: rejection and length cap,
  text control stripping/collapse/cap, draft composition with fallback) and 4 web tests (share
  capture with sanitized notes, malicious URL dropped and controls stripped, manifest content
  type and share target, unauthenticated rejection with no item); full workspace suite green plus
  fmt, arch-check, clippy, WASM build (exit checked directly).
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known gap: a packaged browser extension is not shipped; the command URL is the documented
  boundary and the PWA share target works natively.
- Archive: `2026-10-02-add-browser-capture`; canonical `features/browser-capture` spec created.
- Commit: `feat(54): add browser capture`
- Decisions: GET share target (no script); invalid URLs dropped rather than failing the capture;
  captures land in the Inbox and flow through the normal command/outbox path.
- Live integrations not verified: OS share sheet on a device.

### 55 — `add-mail-to-inbox` — COMPLETE

- Start: 2026-09-29 18:50 · End: 2026-09-30 20:40
- Proposal validation: strict validation passed before archive.
- Implementation: migration 0006 adds `inbound_aliases` (token pk, account, enabled) and
  `inbound_messages` (account+message_id pk ledger); `momentum_app::inbound` provides the
  `AliasStore`/`MessageLedger` ports, CSPRNG `RandomTokenSource` (Uuid v4, 32 hex chars),
  deterministic `map_mail` (quoted lines dropped, whitespace collapsed, "Email" fallback,
  bounded), `InboundLimits`, and `InboundService` (`ensure_alias`, `rotate`, `disable`, `ingest`
  with size checks, enabled-state distinction, persisted dedupe, and a sliding hourly rate
  limit); `PgInboundStore` implements both ports; `POST /inbound/mail` (secret-guarded webhook)
  maps outcomes to deterministic statuses; Settings shows the private address with rotate/disable
  actions; `AppState::with_inbound` wires service + `MOMENTUM_INBOUND_SECRET`.
- Migrations: 0006 `inbound-mail` (additive, reversible).
- Tests: 2 app unit tests (deterministic mapping incl. quote stripping/fallback; subject bound)
  and 3 testkit tests (alias lifecycle with rotation invalidating the old token and disable
  blocking; mapping + message-id dedupe with a persisted ledger; oversized rejection and rate
  limiting with window recovery) and 3 web tests (webhook delivery/duplicate/disabled, secret
  enforcement incl. missing header, settings address + rotate/disable flows); full workspace
  suite green plus fmt, arch-check, clippy, WASM build (exit checked directly).
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known gap: no mail transfer agent is run; the provider webhook plus shared secret is the
  documented boundary, and HTML mail is not rendered (plain text only).
- Archive: `2026-10-02-add-mail-to-inbox`; canonical `features/mail-to-inbox` spec created.
- Commit: `feat(55): add mail to inbox`
- Decisions: aliases are 128-bit random tokens; duplicates bypass rate charges; disabled aliases
  are distinguished from unknown tokens; notes reuse the capture sanitizer.
- Live integrations not verified: real provider webhook delivery.

### 56 — `add-command-urls` — COMPLETE

- Start: 2026-09-30 20:50 · End: 2026-10-01 22:30
- Proposal validation: strict validation passed before archive.
- Implementation: `GET/POST /add` reuses the extracted Quick Entry handler
  (`apply_quick_entry`), so validation/compensation semantics match the UI; `GET /add-project`
  parses an optional area id, creates the project, applies notes, and compensates by deleting the
  fresh project on failure; `GET /update` applies only provided fields in a fixed order with
  parse-before-write; `GET /show` requires exactly one of todo/project/area/list, verifies
  entities through account-scoped services (foreign/missing → not-found redirect), maps lists
  through a fixed allow-list, and `/search` remains the documented stable search URL.
- Migrations: none.
- Tests: 4 web tests (add with metadata, add-project with notes, update applying typed fields and
  rejecting an unsupported `when` without partial writes, show redirects for lists and entities
  plus malformed/conflicting/unsupported reference rejection, and search stability); full
  workspace suite green plus fmt, arch-check, clippy, WASM build (exit checked directly).
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known gap: GET command URLs are intentionally non-idempotent for automation parity with
  `/capture`; no URL signing (sessions authorize).
- Archive: `2026-10-02-add-command-urls`; canonical `features/command-urls` spec created.
- Commit: `feat(56): add command urls`
- Decisions: one shared capture path for `/add` and `/todos/quick`; strict allow-lists for lists
  and schedule kinds; entity scoping through services rather than a separate ACL check.
- Live integrations not verified: none beyond normal HTTP behavior.

### 57 — `add-automation-api` — COMPLETE

- Start: 2026-10-01 22:40 · End: 2026-10-02 21:15
- Proposal validation: strict validation passed before archive.
- Implementation: migration 0007 adds `api_tokens` (id, account, name, unique hash, scopes,
  created/used/revoked timestamps); `momentum_app::api_tokens` provides `ApiScope` (write implies
  read), view-once `CreatedApiToken`, `ApiTokenStore`, `ApiPrincipal`, and `ApiTokenService`
  (create/list/revoke/rotate/authenticate) reusing the auth token generator so only deterministic
  hashes persist; `PgApiTokenStore` implements the port; `/api/v1/todos` supports list (fixed
  scope allow-list), create (with schedule/deadline and compensation), show, patch, and
  complete/cancel/reopen/delete actions, all delegating to `TodoService` with typed JSON errors
  (`unauthorized`, `forbidden`, `invalid_request`, `not_found`, `unavailable`, `internal`);
  Settings gained token management with view-once display via redirect, listing with scopes and
  revoked state, and revoke/rotate actions.
- Migrations: 0007 `api-tokens` (additive, reversible).
- Tests: 1 app unit test (scope parse and write-implies-read), 2 testkit tests (view-once
  creation with last-used tracking and authentication; rotation invalidating the old token,
  revocation blocking use, refused rotation of revoked tokens), 4 web tests (bearer enforcement
  and read-vs-write scopes; CRUD/actions with typed errors incl. unsupported values/actions and
  404s; rotation/revocation taking effect; settings view-once flow with plaintext absent from the
  persisted list); full workspace suite green plus fmt, arch-check, clippy, WASM build (exit
  checked directly). Clippy sort_by_key fixed.
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known gap: no OAuth client registration or webhook subscriptions; bearer tokens only.
- Archive: `2026-10-02-add-automation-api`; canonical `features/automation-api` spec created.
- Commit: `feat(57): add automation api`
- Decisions: deterministic token hashing for lookup (entropy from the generator); view-once
  plaintext via redirect parameter, never stored; write implies read.
- Live integrations not verified: external automation clients.

### 58 — `add-pwa` — COMPLETE

- Start: 2026-10-02 21:25 · End: 2026-10-03 20:40
- Proposal validation: strict validation passed before archive.
- Implementation: the manifest gained `theme_color`, `background_color`, an SVG icon
  (`sizes: any`, maskable), and shortcuts over command URLs (`/add`, `/search`, `/today`) while
  keeping the share target; `momentum-web::pwa` generates the service worker from Rust
  (versioned `momentum-shell-<version>` cache precaching `/`, `/today`, styles, WASM package,
  manifest, and icon; cache-first static assets with refresh-on-miss; network-first navigations
  with a cached-shell fallback; `/sync` and `/api/` excluded) and the registration snippet
  (register after load, reload once on `controllerchange`); the worker skips waiting and claims
  clients on activation while deleting older caches; routes serve `/service-worker.js`
  (uncached, application/javascript) and `/static/icon.svg`; the shell links the manifest and
  icon and embeds the registration.
- Migrations: none.
- Tests: 2 Rust unit tests (versioned worker with precache, skipWaiting/claim, cache deletion,
  endpoint exclusions, and version isolation; registration reload guard) and 4 web tests
  (manifest installability fields, shortcuts, share target; worker headers/content; icon content
  type; shell manifest/icon/registration links); full workspace suite green plus fmt, arch-check,
  clippy, WASM build (exit checked directly).
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known gap: Web Push via service workers and background sync are not wired (the server-side
  push plumbing and the replica/outbox cover those needs); the worker/registration are generated
  from Rust per the no-handwritten-JS rule.
- Archive: `2026-10-03-add-pwa`; canonical `features/pwa` spec created.
- Commit: `feat(58): add pwa`
- Decisions: versioned cache with activation cleanup and one-time reload; live endpoints never
  cached; SVG icon avoids bitmap assets.
- Live integrations not verified: real browser install prompt and offline start.

### 59 — `add-web-companion-views` — COMPLETE

- Start: 2026-10-03 20:50 · End: 2026-10-03 21:50
- Proposal validation: strict validation passed before archive.
- Implementation: `/companion` renders a compact single-column surface with Today rows carrying
  the canonical completion form, derived `today_progress` values exposed as
  `data-completed`/`data-total` plus a "N of M done" line, a quick-add form posting to the normal
  create route (Inbox), the read-only calendar section, and an empty state; compact CSS adds
  large tap targets and a narrow layout.
- Migrations: none.
- Tests: 2 web tests (compact markup with progress matching the derived calculation after one of
  two scheduled items is completed; canonical completion posting and quick-add creating an Inbox
  item, plus the empty state); full workspace suite green plus fmt, arch-check, clippy, WASM
  build (exit checked directly).
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known gap: no separate installable companion app or OS widget integration.
- Archive: `2026-10-03-add-web-companion-views`; canonical `features/web-companion-views` spec
  created.
- Commit: `feat(59): add web companion views`
- Decisions: the companion is presentation-only over existing reads/commands; completion keeps
  the canonical item-page redirect rather than a companion-specific response.
- Live integrations not verified: real small-screen rendering.

### 60 — `add-multiple-window-support` — COMPLETE

- Start: 2026-10-03 22:00 · End: 2026-10-03 23:20
- Proposal validation: strict validation passed before archive.
- Implementation: `AppLayout` reads `?popout=1`; in popout mode the topbar, sidebar, drawer scrim,
  bottom navigation, and footer are omitted (real omission) and a minimal exit header links back
  to the full app; the root exposes `data-window`; the full layout adds an "Open in window" anchor
  (`target="_blank"`, noopener) appending the parameter to the current pathname; commands remain
  stateless HTTP forms shared by all windows.
- Migrations: none.
- Tests: 2 web tests (popout omits chrome, shows shared data and the exit link, full layout keeps
  chrome and the open-in-window link; project popout renders; a completion issued from a popout is
  reflected on the next read in every window); full workspace suite green plus fmt, arch-check,
  clippy, WASM build (exit checked directly).
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known gap: no `window.open` scripting (native `target="_blank"` links suffice) and no
  per-window preferences.
- Archive: `2026-10-03-add-multiple-window-support`; canonical `features/multiple-window-support`
  spec created.
- Commit: `feat(60): add multiple window support`
- Decisions: popout is a query parameter on existing routes (every destination works); windows are
  ordinary tabs using the multi-tab/replica mechanisms.
- Live integrations not verified: real browser window behavior.

### 61 — `add-import-export` — COMPLETE

- Start: 2026-10-03 23:30 · End: 2026-10-04 21:05
- Proposal validation: strict validation passed before archive. (A first gate run was interrupted
  externally; the clean re-run passed, EXIT=0.)
- Implementation: `momentum_app::backup` provides `BACKUP_VERSION` (1), the layout-independent
  `BackupDocument` (areas/projects/tags/to-dos with status, schedule, evening, deadline,
  checklist), `build_backup` over open+logged+trashed reads, deterministic `todos_to_csv` (RFC
  4180 quoting) and `todos_to_markdown` (checkbox rows with when/deadline and nested checklist),
  `validate_backup` (wholesale version rejection plus per-record planning and skip reporting), and
  `apply_import` (additive creation through the normal commands, restoring schedule/deadline/
  checklist and terminal statuses); routes `GET /export/backup.json`, `GET /export/todos.csv`,
  `GET /export/todos.md`, and `POST /import/backup.json` with a typed `unsupported_version`
  error before any application.
- Migrations: none.
- Tests: 3 app unit tests (version rejection, plan validation with skip reporting, deterministic
  CSV/Markdown mapping), 2 testkit tests (additive import preserving existing data and restoring
  completed/deleted states with unsupported-record reporting; export→import round trip preserving
  schedule, deadline, and checklist), and 2 web tests (all three export endpoints with content
  types and deterministic content; import summary plus typed version error); full workspace suite
  green plus fmt, arch-check, clippy, WASM build (exit checked directly). Clippy double-must-use
  fixed.
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known gap: this version imports to-dos only and reports areas/projects/tags as unsupported
  records; replace-mode imports are deliberately absent so imports are never destructive.
- Archive: `2026-10-04-add-import-export`; canonical `features/import-export` spec created.
- Commit: `feat(61): add import export`
- Decisions: validate-then-apply; additive-only; deterministic exports independent of database
  layout.
- Live integrations not verified: none beyond normal HTTP behavior.

### 62 — `add-sharing-and-printing` — COMPLETE

- Start: 2026-10-04 21:15 · End: 2026-10-05 21:40
- Proposal validation: strict validation passed before archive. (A first gate run was interrupted
  externally; the clean re-run passed, EXIT=0.)
- Implementation: `momentum_app::share` provides `escape_markdown`/`escape_html` and deterministic
  models (`share_todo_markdown` with notes/when/deadline/status/checklist, `share_project_markdown`
  with counts, `share_list_markdown` with checkbox rows) plus standalone print documents
  (`print_list_html`, `print_project_html`) with inlined styles and escaped content; routes
  `GET /share/todos/{id}`, `/share/projects/{id}`, `/share/lists/{list}` (text/markdown) and
  `GET /print/lists/{list}`, `/print/projects/{id}` (text/html) with a fixed list allow-list and
  session enforcement; `@media print` rules hide chrome/actions/editors/overlays on interactive
  pages.
- Migrations: none.
- Tests: 2 app unit tests (deterministic markdown with escaped controls; print HTML escaping of
  script tags and entities) and 2 web tests (share markdown determinism for todos/lists/projects
  and 404s; print HTML titles and escaping, project print, 404s, and session enforcement); full
  workspace suite green plus fmt, arch-check, clippy, WASM build (exit checked directly).
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known gap: no anonymous/public share links (session required) and no PDF generation.
- Archive: `2026-10-05-add-sharing-and-printing`; canonical `features/sharing-and-printing` spec
  created.
- Commit: `feat(62): add sharing and printing`
- Decisions: render models live in the app crate (pure/deterministic); print pages are dedicated
  documents; visibility comes from account-scoped open scopes.
- Live integrations not verified: real print rendering.

### 63 — `add-settings` — COMPLETE

- Start: 2026-10-05 21:50 · End: 2026-10-05 23:30
- Proposal validation: strict validation passed before archive.
- Implementation: `momentum_app::preferences` declares 12 preferences across all nine domains
  (general, appearance, today, calendar, notifications, quick_entry, logbook, account_sync,
  data) with typed values (bool/choice), defaults, choice allow-lists, and scopes
  (account/device/browser); `parse_value` rejects unknown keys/values, `sanitize` falls back to
  the typed default, and `effective` resolves account-scoped values through the new
  `PreferenceStore` port (PG implementation over the existing `preferences` table; memory for
  tests); Settings renders a Preferences section grouped by domain with scope badges,
  current/default selects, and validated submission; `POST /settings/preferences` persists
  account-scoped values and acknowledges device/browser scopes for the hydrated client.
- Migrations: none (`preferences` already had the account/device/browser scope constraint).
- Tests: 2 app unit tests (registry uniqueness/defaults/domain coverage/scope declarations;
  parsing rejection and sanitize fallbacks) and 3 web tests (all domains and scopes rendered;
  account persistence with display, invalid and unknown rejection without writes; browser-scope
  acknowledgement without server storage); full workspace suite green plus fmt, arch-check,
  clippy, WASM build (exit checked directly).
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known gap: the hydrated client applies browser/device preferences (theme, density) locally; the
  server validates and stores only account-scoped values.
- Archive: `2026-10-05-add-settings`; canonical `features/settings` spec created (Purpose expanded
  after a validation warning about brevity).
- Commit: `feat(63): add settings`
- Decisions: typed registry as the single source; sanitize-on-read; device/browser scopes never
  persisted server-side.
- Live integrations not verified: client-side preference application.

### 64 — `add-theme-and-text-sizing` — COMPLETE

- Start: 2026-10-05 23:40 · End: 2026-10-06 00:40
- Proposal validation: strict validation passed before archive.
- Implementation: the registry gained `appearance.text_size` (normal/large/extra_large) and
  `appearance.reduced_motion` (bool) alongside `appearance.theme`, all browser-local; design.css
  gained explicit `[data-theme="dark"]`/`[data-theme="light"]` token sets with the system media
  query scoped to `:root:not([data-theme="light"])`, rem-based `html[data-text-size]` scaling,
  `prefers-contrast: more` strengthening for both themes, and an app-level
  `[data-reduced-motion="true"]` override layered on the existing system reduced-motion queries.
- Migrations: none.
- Tests: registry unit tests cover the new definitions automatically, and 2 web tests assert the
  rendered preference keys/scopes (including extra_large) with browser-scope acknowledgement
  without server storage, plus the presence of theme/text-size/contrast/reduced-motion rules in
  the served stylesheets; full workspace suite green plus fmt, arch-check, clippy, WASM build
  (exit checked directly).
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known gap: the hydrated client applies the root attributes from browser-local preferences; the
  server ships the complete CSS contract.
- Archive: `2026-10-06-add-theme-and-text-sizing`; canonical `features/theme-and-text-sizing`
  spec created with an expanded Purpose.
- Commit: `feat(64): add theme and text sizing`
- Decisions: forced light wins over a dark OS preference; text size scales the root only;
  contrast/motion remain system-compatible with an explicit opt-in override.
- Live integrations not verified: client-side theme application and OS preference following.

### 65 — `add-localization` — COMPLETE

- Start: 2026-10-06 00:50 · End: 2026-10-07 03:45
- Proposal validation: strict validation passed before archive. (A first gate run was killed by the
  tool timeout after passing fmt/arch-check/openspec/clippy and 49 test binaries; a clean
  workspace test re-run passed all 99 suites, EXIT=0.)
- Implementation: `momentum_domain::time::is_valid_timezone` (chrono-tz) backs a new validated
  text preference kind (`PreferenceDefault::Text`/`PreferenceValue::Text`); the registry gained
  account-synced `general.locale` (en-US/en-GB/de-DE/fr-FR), `general.hour_format` (h24/h12),
  `general.timezone` (validated IANA text, default UTC), and `general.parser_locale`
  (en-US/en-GB) alongside `general.week_start`; the schedule route's natural branch resolves the
  parser locale through `effective`, the reminder route defaults an omitted timezone to
  `general.timezone` while conversion stays DST-safe, and Settings renders text preferences as
  labeled text inputs.
- Migrations: none.
- Tests: 1 domain unit test (timezone validation), registry tests cover text defaults, and 3 web
  tests (locale preferences rendered/persisted with invalid timezone rejection; en-GB parser
  reading of the ambiguous 05/06 rolling to 2027-06-05; timezone preference used as the reminder
  default with the correct UTC conversion); all 99 workspace suites green plus fmt, arch-check,
  clippy, WASM build (exit checked directly).
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known gap: UI strings are not translated (locale metadata only); per-entity timezones are out
  of scope.
- Archive: `2026-10-07-add-localization`; canonical `features/localization` spec created with an
  expanded Purpose.
- Commit: `feat(65): add localization`
- Decisions: timezone validation in the domain crate; parser locale independent of storage;
  date-only values never gain time components.
- Live integrations not verified: localized formatting in the hydrated client.

### 66 — `add-accessibility` — COMPLETE

- Start: 2026-10-07 03:55 · End: 2026-10-07 05:40
- Proposal validation: strict validation passed before archive.
- Implementation: the layout gained a skip link (`href="#app-main"`) as the first focusable
  element and `id="app-main"` on the main landmark; CSS gained `.skip-link` reveal-on-focus,
  `:focus-visible` accent outlines, and 44px minimum targets under `pointer: coarse` (reduced
  motion and contrast rules already existed from 64); the companion progress exposes
  `role="progressbar"` with aria-valuemin/max/now, Today keeps its native labeled `<progress>`,
  the Quick Entry overlay declares `aria-modal="true"`, and the audit suite verifies nav
  landmark/current semantics plus labeled inputs/selects/textareas/buttons on core pages.
- Migrations: none.
- Tests: 3 web audit tests (skip link/landmark/labels across /inbox, /today, /companion,
  /settings with a markup-scanning helper for unlabeled controls; progress/dialog/nav semantics;
  CSS contract for focus, coarse targets, reduced motion, and contrast); full workspace suite
  green plus fmt, arch-check, clippy, WASM build (exit checked directly).
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known gap: no scripted focus trap on the CSS `:target` dialog (aria-modal documents intent and
  the hydrated client can add trapping without markup changes); no lab screen-reader testing.
- Archive: `2026-10-07-add-accessibility`; canonical `features/accessibility` spec created with an
  expanded Purpose.
- Commit: `feat(66): add accessibility`
- Decisions: prefer native semantics (progress, landmarks, labels) over ARIA patches; enforce the
  contract with an automated audit.
- Live integrations not verified: real assistive-technology behavior.

### 67 — `add-animation-system` — COMPLETE

- Start: 2026-10-07 05:50 · End: 2026-10-07 07:10
- Proposal validation: strict validation passed before archive.
- Implementation: `momentum_app::motion` declares six primitives (insert, remove, editor,
  popover, drag, navigation) with stable ids, CSS classes, design-token durations, and
  essential flags, with `primitive`/`class_for`/`nonessential` lookups; app.css defines the
  keyframes/transitions once and wires classes into markup (rows carry `motion-insert
  motion-drag`, expanded editors `motion-editor`, the Quick Entry panel `motion-popover`, main
  content `motion-nav`; `motion-remove` is applied by the client as rows leave); reduced motion
  suppresses every nonessential primitive via both the media query and the explicit
  `[data-reduced-motion="true"]` override, and motion remains presentation-only with no gating.
- Migrations: none.
- Tests: 2 app unit tests (registry uniqueness/token prefixes/lookups; suppression coverage) and
  2 web tests (markup carries every primitive class except client-only removal; stylesheet
  declares all rules plus reduced-motion suppression); full workspace suite green plus fmt,
  arch-check, clippy, WASM build (exit checked directly).
- Verification: manual artifact-to-implementation pass; every requirement scenario covered.
- Known gap: JS-driven layout animations and spring physics are out of scope; all primitives are
  nonessential and therefore removed under reduced motion.
- Archive: `2026-10-07-add-animation-system`; canonical `features/animation-system` spec created
  with an expanded Purpose.
- Commit: `feat(67): add animation system`
- Decisions: one registry and one CSS source; reduced-motion sets are enforced by tests;
  removal motion stays client-applied but centrally declared.
- Live integrations not verified: real browser animation behavior.

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
| 36 | add-date-pickers | COMPLETE | `feat(36)` |
| 37 | add-natural-language-dates | COMPLETE | `feat(37)` |
| 38 | add-reminders | COMPLETE | `feat(38)` |
| 39 | add-reminder-snoozing | COMPLETE | `feat(39)` |
| 40 | add-recurrence-model | COMPLETE | `feat(40)` |
| 41 | add-recurrence-materializer | COMPLETE | `feat(41)` |
| 42 | add-recurrence-exceptions | COMPLETE | `feat(42)` |
| 43 | add-quick-find | COMPLETE | `feat(43)` |
| 44 | add-deep-search | COMPLETE | `feat(44)` |
| 45 | add-keyboard-command-system | COMPLETE | `feat(45)` |
| 46 | add-type-travel | COMPLETE | `feat(46)` |
| 47 | add-mobile-gestures | COMPLETE | `feat(47)` |
| 48 | add-calendar-integration | COMPLETE | `feat(48)` |
| 49 | add-offline-local-replica | COMPLETE | `feat(49)` |
| 50 | add-cloud-sync-protocol | COMPLETE | `feat(50)` |
| 51 | add-realtime-sync | COMPLETE | `feat(51)` |
| 52 | add-multitab-coordination | COMPLETE | `feat(52)` |
| 53 | add-quick-entry | COMPLETE | `feat(53)` |
| 54 | add-browser-capture | COMPLETE | `feat(54)` |
| 55 | add-mail-to-inbox | COMPLETE | `feat(55)` |
| 56 | add-command-urls | COMPLETE | `feat(56)` |
| 57 | add-automation-api | COMPLETE | `feat(57)` |
| 58 | add-pwa | COMPLETE | `feat(58)` |
| 59 | add-web-companion-views | COMPLETE | `feat(59)` |
| 60 | add-multiple-window-support | COMPLETE | `feat(60)` |
| 61 | add-import-export | COMPLETE | `feat(61)` |
| 62 | add-sharing-and-printing | COMPLETE | `feat(62)` |
| 63 | add-settings | COMPLETE | `feat(63)` |
| 64 | add-theme-and-text-sizing | COMPLETE | `feat(64)` |
| 65 | add-localization | COMPLETE | `feat(65)` |
| 66 | add-accessibility | COMPLETE | `feat(66)` |
| 67 | add-animation-system | COMPLETE | `feat(67)` |

- Completed proposal range: 01–67 (67 of 74).
- Incomplete: 68–74 (not started; sequential order means each depends on the previous state).
- Blocked: none. Change 14 was next and unblocked when the session's working context ended.
- Total commits on `agent/things-web-overnight`: 67 through change 46 (bootstrap, 46 numbered
  changes, spec/report docs commits; HEAD will be the change-46 commit).
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

1. Changes 68–74, strictly in order; the workspace, command layer, projection scopes,
   move engine, drag/drop drop-target operations, selection/bulk layer, Magic Plus creation
   composition, inline editor/picker/natural-date composition, reminder scheduling/snoozing/push
   plumbing, recurrence model/materializer/exceptions, Quick Find and deep search, keyboard
   registry, Type Travel, touch gesture machines, calendar projection, offline replica/outbox,
   cloud sync protocol, realtime hints, multi-tab coordination, Quick Entry, browser capture,
   inbound mail, command URLs, the automation API, the PWA shell, companion views, popout windows,
   import/export, share/print models, the typed settings registry, appearance theming,
   localization, the accessibility audit and the motion system, read-store patterns, and test
   harnesses are in place to make each one mechanical. Next: 68.
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
- Current state at HEAD (change 67): all quality gates green
  (`cargo fmt --check`, `xtask arch-check`, `clippy -D warnings`, `cargo test --workspace
  --all-features`, WASM client build); `openspec validate --specs/--archived --strict` reports
  46/46 passing; `git status` clean.


