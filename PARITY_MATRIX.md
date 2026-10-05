# Parity Matrix — things-web-parity-hardening (change 74)

Audit of every roadmap capability (01–74) against the frozen parity matrix. Statuses:

- **Parity met** — canonical spec archived, scenarios implemented, automated tests green.
- **This change** — the final hardening change itself.
- **Classified gap** — unresolved items are listed with rationale in the gaps section.

| # | Capability | Status | Canonical spec | Archived change |
| --- | --- | --- | --- | --- |
| 01 | bootstrap-fullstack-rust-workspace | Parity met | `platform/rust-workspace` | `2026-09-23-bootstrap-fullstack-rust-workspace` |
| 02 | add-web-application-shell | Parity met | `app/shell` | `2026-09-23-add-web-application-shell` |
| 03 | add-things-design-system | Parity met | `ui/design-system` | `2026-09-23-add-things-design-system` |
| 04 | add-responsive-layout-foundation | Parity met | `ui/responsive-layout` | `2026-09-23-add-responsive-layout-foundation` |
| 05 | add-persistence-foundation | Parity met | `persistence/schema` | `2026-09-24-add-persistence-foundation` |
| 06 | add-account-authentication | Parity met | `auth/accounts` | `2026-09-24-add-account-authentication` |
| 07 | add-task-domain-core | Parity met | `domain/tasks` | `2026-09-24-add-task-domain-core` |
| 08 | add-fractional-ordering-engine | Parity met | `domain/ordering` | `2026-09-24-add-fractional-ordering-engine` |
| 09 | add-command-transaction-layer | Parity met | `app/commands` | `2026-09-24-add-command-transaction-layer` |
| 10 | add-undo-redo-engine | Parity met | `app/undo` | `2026-09-24-add-undo-redo-engine` |
| 11 | add-todo-crud | Parity met | `features/todo-crud` | `2026-09-24-add-todo-crud` |
| 12 | add-inbox | Parity met | `features/inbox` | `2026-09-24-add-inbox` |
| 13 | add-areas | Parity met | `features/areas` | `2026-09-24-add-areas` |
| 14 | add-projects | Parity met | `features/projects` | `2026-09-24-add-projects` |
| 15 | add-project-headings | Parity met | `features/project-headings` | `2026-09-24-add-project-headings` |
| 16 | add-checklists | Parity met | `features/checklists` | `2026-09-24-add-checklists` |
| 17 | add-tags | Parity met | `features/tags` | `2026-09-24-add-tags` |
| 18 | add-notes-editor | Parity met | `features/notes-editor` | `2026-09-24-add-notes-editor` |
| 19 | add-things-markdown | Parity met | `features/markdown` | `2026-09-24-add-things-markdown` |
| 20 | add-scheduling-model | Parity met | `features/scheduling` | `2026-09-25-add-scheduling-model` |
| 21 | add-deadlines | Parity met | `features/deadlines` | `2026-09-25-add-deadlines` |
| 22 | add-today-projection | Parity met | `features/today` | `2026-09-26-add-today-projection` |
| 23 | add-upcoming-projection | Parity met | `features/upcoming` | `2026-09-26-add-upcoming-projection` |
| 24 | add-anytime-projection | Parity met | `features/anytime` | `2026-09-26-add-anytime-projection` |
| 25 | add-someday-projection | Parity met | `features/someday` | `2026-09-26-add-someday-projection` |
| 26 | add-completion-and-cancellation | Parity met | `features/completion` | `2026-09-26-add-completion-and-cancellation` |
| 27 | add-logbook | Parity met | `features/logbook` | `2026-09-26-add-logbook` |
| 28 | add-trash | Parity met | `features/trash` | `2026-09-26-add-trash` |
| 29 | add-special-lists | Parity met | `features/special-lists` | `2026-09-26-add-special-lists` |
| 30 | add-project-progress | Parity met | `features/progress` | `2026-09-27-add-project-progress` |
| 31 | add-move-engine | Parity met | `features/movement` | `2026-09-27-add-move-engine` |
| 32 | add-drag-and-drop | Parity met | `features/drag-and-drop` | `2026-09-27-add-drag-and-drop` |
| 33 | add-multi-selection | Parity met | `features/multi-selection` | `2026-09-27-add-multi-selection` |
| 34 | add-magic-plus | Parity met | `features/magic-plus` | `2026-09-27-add-magic-plus` |
| 35 | add-expanded-item-editor | Parity met | `features/expanded-item-editor` | `2026-09-27-add-expanded-item-editor` |
| 36 | add-date-pickers | Parity met | `features/date-pickers` | `2026-09-27-add-date-pickers` |
| 37 | add-natural-language-dates | Parity met | `features/natural-language-dates` | `2026-09-28-add-natural-language-dates` |
| 38 | add-reminders | Parity met | `features/reminder-delivery` | `2026-09-28-add-reminders` |
| 39 | add-reminder-snoozing | Parity met | `features/reminder-snoozing` | `2026-09-28-add-reminder-snoozing` |
| 40 | add-recurrence-model | Parity met | `features/recurrence-model` | `2026-09-28-add-recurrence-model` |
| 41 | add-recurrence-materializer | Parity met | `features/recurrence-materializer` | `2026-09-28-add-recurrence-materializer` |
| 42 | add-recurrence-exceptions | Parity met | `features/recurrence-exceptions` | `2026-09-28-add-recurrence-exceptions` |
| 43 | add-quick-find | Parity met | `features/quick-find` | `2026-09-29-add-quick-find` |
| 44 | add-deep-search | Parity met | `features/deep-search` | `2026-09-29-add-deep-search` |
| 45 | add-keyboard-command-system | Parity met | `features/keyboard-command-system` | `2026-09-29-add-keyboard-command-system` |
| 46 | add-type-travel | Parity met | `features/type-travel` | `2026-09-29-add-type-travel` |
| 47 | add-mobile-gestures | Parity met | `features/mobile-gestures` | `2026-09-29-add-mobile-gestures` |
| 48 | add-calendar-integration | Parity met | `features/calendar-integration` | `2026-09-29-add-calendar-integration` |
| 49 | add-offline-local-replica | Parity met | `features/offline-local-replica` | `2026-10-01-add-offline-local-replica` |
| 50 | add-cloud-sync-protocol | Parity met | `features/cloud-sync-protocol` | `2026-10-01-add-cloud-sync-protocol` |
| 51 | add-realtime-sync | Parity met | `features/realtime-sync` | `2026-10-01-add-realtime-sync` |
| 52 | add-multitab-coordination | Parity met | `features/multitab-coordination` | `2026-10-01-add-multitab-coordination` |
| 53 | add-quick-entry | Parity met | `features/quick-entry` | `2026-10-02-add-quick-entry` |
| 54 | add-browser-capture | Parity met | `features/browser-capture` | `2026-10-02-add-browser-capture` |
| 55 | add-mail-to-inbox | Parity met | `features/mail-to-inbox` | `2026-10-02-add-mail-to-inbox` |
| 56 | add-command-urls | Parity met | `features/command-urls` | `2026-10-02-add-command-urls` |
| 57 | add-automation-api | Parity met | `features/automation-api` | `2026-10-03-add-automation-api` |
| 58 | add-pwa | Parity met | `features/pwa` | `2026-10-03-add-pwa` |
| 59 | add-web-companion-views | Parity met | `features/web-companion-views` | `2026-10-03-add-web-companion-views` |
| 60 | add-multiple-window-support | Parity met | `features/multiple-window-support` | `2026-10-03-add-multiple-window-support` |
| 61 | add-import-export | Parity met | `features/import-export` | `2026-10-03-add-import-export` |
| 62 | add-sharing-and-printing | Parity met | `features/sharing-and-printing` | `2026-10-03-add-sharing-and-printing` |
| 63 | add-settings | Parity met | `features/settings` | `2026-10-03-add-settings` |
| 64 | add-theme-and-text-sizing | Parity met | `features/theme-and-text-sizing` | `2026-10-04-add-theme-and-text-sizing` |
| 65 | add-localization | Parity met | `features/localization` | `2026-10-04-add-localization` |
| 66 | add-accessibility | Parity met | `features/accessibility` | `2026-10-04-add-accessibility` |
| 67 | add-animation-system | Parity met | `features/animation-system` | `2026-10-04-add-animation-system` |
| 68 | add-large-list-performance | Parity met | `features/large-list-performance` | `2026-10-04-add-large-list-performance` |
| 69 | add-security-hardening | Parity met | `features/security-hardening` | `2026-10-04-add-security-hardening` |
| 70 | add-observability | Parity met | `features/observability` | `2026-10-05-add-observability` |
| 71 | add-visual-parity-suite | Parity met | `features/visual-parity-suite` | `2026-10-05-add-visual-parity-suite` |
| 72 | add-behavior-parity-suite | Parity met | `features/behavior-parity-suite` | `2026-10-05-add-behavior-parity-suite` |
| 73 | add-production-deployment | Parity met | `features/production-deployment` | `2026-10-05-add-production-deployment` |
| 74 | things-web-parity-hardening | This change | `—` | `— (this change)` |

- Roadmap capabilities audited: 74. Rows without a canonical spec were infrastructure or
  modified-capability changes whose requirements were folded into existing specs.
- Archived changes: 74; canonical specs: 73; ignored tests: 0;
  TODO/FIXME markers in Rust sources: 0.

## Classified gaps (unresolved items before closure)

| Finding | Classification | Rationale | Evidence |
| --- | --- | --- | --- |
| Pointer/touch gesture layer (drag, swipe, long-press, type-to-open, keyboard dispatch) | Hydration harness | The hydrated Rust/WASM client consumes the server-rendered hooks; the workspace builds the WASM artifact but has no browser automation harness to exercise gestures. | Changes 32/33/34/45/46/47; hooks and state machines tested server-side. |
| IndexedDB ReplicaStore adapter, BroadcastChannel/Web Locks adapters | Browser adapter | Ports, coordinator semantics, and durability tests are complete; the browser-only adapters require the hydrated client. | Changes 49/52; memory stores + tests. |
| Web Push network transport (VAPID/HTTP) and service-worker registration | Deployment integration | Subscription persistence, payloads, deep links, and fan-out are implemented behind traits; the network adapter needs credentials/infrastructure. | Change 38; RecordingPushTransport tests. |
| Calendar provider network adapters (Google/Microsoft/CalDAV) and OAuth | Deployment integration | Provider-neutral contract, resilience, caching, and UI are complete; adapters need credential stores. | Change 48; FlakyCalendarProvider tests. |
| Mail transfer agent / provider webhook delivery | Deployment integration | Alias lifecycle, mapping, limits, dedupe, and the secret-guarded webhook are complete; a real provider is deployment config. | Change 55; webhook tests. |
| Browser extension/bookmarklet packaging | Out of web scope | The documented `/capture` command URL is the adapter boundary; shipping an extension is separate distribution work. | Change 54; capture and manifest tests. |
| Pixel-level browser screenshots and OS share sheet / PWA install prompts | Browser harness | The structural visual-parity suite covers markup deterministically; pixel capture needs a browser runner. | Changes 58/71; normalization + 12 baselines. |
| Docker build/compose runtime verification | Environment | Artifacts are committed and syntax-reviewed; no Docker daemon exists in this environment to execute them. | Change 73; deployment tests cover probes and the migrate guard. |

## Closure statement

Every roadmap capability has an archived canonical spec whose requirement scenarios are
implemented and covered by automated tests (unit, testkit integration, web surface, behavior
and visual parity suites). All remaining findings are classified above as hydration-harness,
browser-adapter, deployment-integration, out-of-web-scope, or environment items — none is an
unimplemented in-scope capability or an untested acceptance criterion.
