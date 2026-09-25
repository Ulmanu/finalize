//! Canonical PostgreSQL schema (migration 0001).
//!
//! Conventions:
//! * UUIDv7 primary keys so offline clients can create records before synchronizing.
//! * `account_id` on every user-owned row for account-scoped authorization.
//! * `version` for optimistic concurrency and `updated_at` for diagnostics.
//! * Date-only concepts use `date`; instants use `timestamptz` plus an IANA timezone.
//! * `placements.scope` keeps contextual ordering independent from projection ordering.

/// Forward SQL for migration 0001.
pub const CANONICAL_SCHEMA_UP: &str = r"
create table if not exists accounts (
    id uuid primary key,
    email text not null,
    email_normalized text not null unique,
    display_name text,
    password_hash text,
    email_verified_at timestamptz,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    version bigint not null default 1,
    deleted_at timestamptz
);

create table if not exists sessions (
    id uuid primary key,
    account_id uuid not null references accounts(id) on delete cascade,
    token_hash text not null unique,
    created_at timestamptz not null default now(),
    expires_at timestamptz not null,
    revoked_at timestamptz,
    last_seen_at timestamptz,
    user_agent text
);

create index if not exists sessions_account_idx on sessions (account_id);

create table if not exists entities (
    id uuid primary key,
    account_id uuid not null references accounts(id) on delete cascade,
    kind text not null check (kind in ('todo', 'project', 'area', 'heading', 'tag')),
    title text not null default '',
    notes text not null default '',
    status text not null default 'open' check (status in ('open', 'completed', 'canceled', 'deleted')),
    schedule_kind text not null default 'anytime' check (schedule_kind in ('anytime', 'someday', 'date')),
    schedule_date date,
    schedule_evening boolean not null default false,
    deadline date,
    completed_at timestamptz,
    canceled_at timestamptz,
    logged_at timestamptz,
    deleted_at timestamptz,
    archived_at timestamptz,
    recurrence_rule jsonb,
    reminder_rule jsonb,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    version bigint not null default 1
);

create index if not exists entities_account_status_idx on entities (account_id, status);
create index if not exists entities_account_kind_idx on entities (account_id, kind);
create index if not exists entities_account_schedule_idx on entities (account_id, schedule_date);

create table if not exists placements (
    id uuid primary key,
    account_id uuid not null references accounts(id) on delete cascade,
    entity_id uuid not null references entities(id) on delete cascade,
    scope text not null check (scope in ('context', 'today')),
    parent_kind text not null check (parent_kind in ('inbox', 'area', 'project', 'heading')),
    parent_id uuid,
    rank text not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    version bigint not null default 1
);

create index if not exists placements_scope_idx on placements (account_id, scope, parent_kind, parent_id, rank);
create unique index if not exists placements_entity_scope_idx on placements (entity_id, scope);

create table if not exists tag_assignments (
    account_id uuid not null references accounts(id) on delete cascade,
    entity_id uuid not null references entities(id) on delete cascade,
    tag_id uuid not null references entities(id) on delete cascade,
    created_at timestamptz not null default now(),
    primary key (entity_id, tag_id)
);

create index if not exists tag_assignments_tag_idx on tag_assignments (account_id, tag_id);

create table if not exists checklist_items (
    id uuid primary key,
    account_id uuid not null references accounts(id) on delete cascade,
    entity_id uuid not null references entities(id) on delete cascade,
    title text not null,
    completed boolean not null default false,
    rank text not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    version bigint not null default 1
);

create index if not exists checklist_items_entity_idx on checklist_items (entity_id, rank);

create table if not exists recurrence_templates (
    entity_id uuid primary key references entities(id) on delete cascade,
    account_id uuid not null references accounts(id) on delete cascade,
    rule jsonb not null,
    template jsonb not null,
    next_occurrence date,
    horizon_date date,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create table if not exists recurrence_instances (
    template_id uuid not null references recurrence_templates(entity_id) on delete cascade,
    account_id uuid not null references accounts(id) on delete cascade,
    occurrence_key text not null,
    entity_id uuid not null references entities(id) on delete cascade,
    materialized_at timestamptz not null default now(),
    primary key (template_id, occurrence_key)
);

create table if not exists reminders (
    id uuid primary key,
    account_id uuid not null references accounts(id) on delete cascade,
    entity_id uuid not null references entities(id) on delete cascade,
    remind_at timestamptz not null,
    timezone text not null,
    state text not null default 'scheduled' check (state in ('scheduled', 'delivered', 'snoozed', 'canceled')),
    delivered_at timestamptz,
    snooze_count integer not null default 0,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    version bigint not null default 1
);

create index if not exists reminders_due_idx on reminders (state, remind_at);
create index if not exists reminders_entity_idx on reminders (entity_id);

create table if not exists preferences (
    account_id uuid not null references accounts(id) on delete cascade,
    key text not null,
    scope text not null check (scope in ('account', 'device', 'browser')),
    value jsonb not null,
    updated_at timestamptz not null default now(),
    primary key (account_id, key)
);

create table if not exists sync_metadata (
    account_id uuid primary key references accounts(id) on delete cascade,
    sequence bigint not null default 0,
    updated_at timestamptz not null default now()
);

create table if not exists change_log (
    account_id uuid not null references accounts(id) on delete cascade,
    sequence bigint not null,
    operation_id uuid not null,
    entity_id uuid,
    entity_kind text,
    kind text not null,
    payload jsonb not null default '{}'::jsonb,
    created_at timestamptz not null default now(),
    primary key (account_id, sequence)
);

create unique index if not exists change_log_operation_idx on change_log (account_id, operation_id);

create table if not exists tombstones (
    id uuid primary key,
    account_id uuid not null references accounts(id) on delete cascade,
    entity_id uuid not null,
    entity_kind text not null,
    deleted_at timestamptz not null default now(),
    purge_after timestamptz
);

create index if not exists tombstones_account_idx on tombstones (account_id, deleted_at);
";

/// Rollback SQL for migration 0001.
///
/// This destroys all persisted data and is intended only for isolated development and test
/// databases.
pub const CANONICAL_SCHEMA_DOWN: &str = r"
drop table if exists tombstones;
drop table if exists change_log;
drop table if exists sync_metadata;
drop table if exists preferences;
drop table if exists reminders;
drop table if exists recurrence_instances;
drop table if exists recurrence_templates;
drop table if exists checklist_items;
drop table if exists tag_assignments;
drop table if exists placements;
drop table if exists entities;
drop table if exists sessions;
drop table if exists accounts;
";

/// Forward SQL for migration 0002: single-use authentication tokens.
pub const AUTH_TOKENS_UP: &str = r"
create table if not exists auth_tokens (
    id uuid primary key,
    account_id uuid not null references accounts(id) on delete cascade,
    kind text not null check (kind in ('email_verification', 'password_reset')),
    token_hash text not null unique,
    expires_at timestamptz not null,
    consumed_at timestamptz,
    created_at timestamptz not null default now()
);

create index if not exists auth_tokens_account_idx on auth_tokens (account_id, kind);
";

/// Rollback SQL for migration 0002.
pub const AUTH_TOKENS_DOWN: &str = "drop table if exists auth_tokens;";

/// Forward SQL for migration 0003: persist entity archive state.
///
/// Archive hides an entity from active views without deleting it or its children, so a
/// non-destructive `archived` flag is stored on the shared entity row.
pub const ENTITY_ARCHIVED_UP: &str = r"
-- Archive is a non-destructive state: headings disappear from active project order but
-- keep their identity, children, and change history.
alter table entities add column if not exists archived boolean not null default false;
";

/// Rollback SQL for migration 0003.
pub const ENTITY_ARCHIVED_DOWN: &str = "alter table entities drop column if exists archived;";

/// Forward SQL for migration 0004: persist tag hierarchy and sibling order.
pub const TAG_HIERARCHY_UP: &str = r"
-- Tags nest through tag_parent and order among siblings through tag_position; both live on the
-- shared entity row so tag reads stay single-table.
alter table entities add column if not exists tag_parent uuid;
alter table entities add column if not exists tag_position integer not null default 0;
";

/// Rollback SQL for migration 0004.
pub const TAG_HIERARCHY_DOWN: &str = "
alter table entities drop column if exists tag_parent;
alter table entities drop column if exists tag_position;
";

/// Forward SQL for migration 0005: an independent This Evening ordering bucket.
///
/// Today's day section and This Evening section order independently, so the placement scope
/// check gains the `today_evening` bucket alongside `today`.
pub const PROJECTION_SCOPE_EVENING_UP: &str = r"
alter table placements drop constraint if exists placements_scope_check;
delete from placements where scope = 'today_evening';
alter table placements add constraint placements_scope_check
    check (scope in ('context', 'today', 'today_evening'));
";

/// Rollback SQL for migration 0005.
pub const PROJECTION_SCOPE_EVENING_DOWN: &str = "
delete from placements where scope = 'today_evening';
alter table placements drop constraint if exists placements_scope_check;
alter table placements add constraint placements_scope_check
    check (scope in ('context', 'today'));
";
