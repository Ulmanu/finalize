//! Rust-managed database migrations.
//!
//! Migrations are defined in Rust (SQL strings compiled into the binary), applied in strictly
//! increasing version order inside transactions, and recorded with a checksum so that edited
//! migrations are detected. Rollback is an explicit operator/test action.

use sqlx::PgPool;
use thiserror::Error;

use super::schema;

/// A single forward migration with its optional rollback.
#[derive(Debug, Clone, Copy)]
pub struct Migration {
    /// Strictly increasing version number.
    pub version: i64,
    /// Human-readable name recorded in the migration table.
    pub name: &'static str,
    /// Forward SQL.
    pub up: &'static str,
    /// Rollback SQL, when a safe rollback exists.
    pub down: Option<&'static str>,
    /// Documented reason when no rollback exists.
    pub irreversible_reason: Option<&'static str>,
}

/// All migrations in application order.
pub const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        name: "canonical-schema",
        up: schema::CANONICAL_SCHEMA_UP,
        down: Some(schema::CANONICAL_SCHEMA_DOWN),
        irreversible_reason: None,
    },
    Migration {
        version: 2,
        name: "auth-tokens",
        up: schema::AUTH_TOKENS_UP,
        down: Some(schema::AUTH_TOKENS_DOWN),
        irreversible_reason: None,
    },
    Migration {
        version: 3,
        name: "entity-archived",
        up: schema::ENTITY_ARCHIVED_UP,
        down: Some(schema::ENTITY_ARCHIVED_DOWN),
        irreversible_reason: None,
    },
    Migration {
        version: 4,
        name: "tag-hierarchy",
        up: schema::TAG_HIERARCHY_UP,
        down: Some(schema::TAG_HIERARCHY_DOWN),
        irreversible_reason: None,
    },
];

/// Migration failures.
#[derive(Debug, Error)]
pub enum MigrationError {
    /// The database connection or query failed.
    #[error("database error while running migration {version}: {source}")]
    Database {
        /// Version being applied when the failure occurred.
        version: i64,
        /// Underlying sqlx error.
        #[source]
        source: sqlx::Error,
    },
    /// A previously applied migration's SQL changed.
    #[error("migration {version} checksum mismatch: recorded {recorded}, computed {computed}")]
    ChecksumMismatch {
        /// Migration version.
        version: i64,
        /// Checksum recorded when the migration was applied.
        recorded: String,
        /// Checksum computed from the current binary.
        computed: String,
    },
    /// The last applied migration has no rollback script.
    #[error("migration {version} has no rollback script: {reason}")]
    Irreversible {
        /// Migration version.
        version: i64,
        /// Documented reason.
        reason: String,
    },
    /// The migration table contains a version that is not in the compiled list.
    #[error("database records unknown migration version {version}")]
    UnknownVersion {
        /// Unknown version.
        version: i64,
    },
}

const MIGRATION_TABLE: &str = r"
create table if not exists schema_migrations (
    version bigint primary key,
    name text not null,
    checksum text not null,
    applied_at timestamptz not null default now()
)";

/// Computes the stable FNV-1a 64-bit checksum of a migration's forward SQL.
#[must_use]
pub fn checksum(sql: &str) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in sql.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}")
}

/// Returns applied migrations ordered by version.
///
/// # Errors
///
/// Returns [`MigrationError::Database`] when the migration table cannot be read.
pub async fn applied(pool: &PgPool) -> Result<Vec<(i64, String, String)>, MigrationError> {
    sqlx::query(MIGRATION_TABLE)
        .execute(pool)
        .await
        .map_err(|source| MigrationError::Database { version: 0, source })?;

    let rows = sqlx::query_as::<_, (i64, String, String)>(
        "select version, name, checksum from schema_migrations order by version",
    )
    .fetch_all(pool)
    .await
    .map_err(|source| MigrationError::Database { version: 0, source })?;
    Ok(rows)
}

/// Returns versions that are compiled but not yet applied.
///
/// # Errors
///
/// Propagates [`MigrationError`] from [`applied`].
pub async fn pending(pool: &PgPool) -> Result<Vec<i64>, MigrationError> {
    let applied = applied(pool).await?;
    let applied_versions: std::collections::BTreeSet<i64> =
        applied.iter().map(|(version, _, _)| *version).collect();

    for (version, _, _) in &applied {
        if !MIGRATIONS
            .iter()
            .any(|migration| migration.version == *version)
        {
            return Err(MigrationError::UnknownVersion { version: *version });
        }
    }

    Ok(MIGRATIONS
        .iter()
        .filter(|migration| !applied_versions.contains(&migration.version))
        .map(|migration| migration.version)
        .collect())
}

/// Applies every pending migration in version order.
///
/// Returns the versions that were applied by this call.
///
/// # Errors
///
/// Returns [`MigrationError::ChecksumMismatch`] when a previously applied migration's SQL has
/// changed, or [`MigrationError::Database`] when SQL execution fails.
pub async fn apply(pool: &PgPool) -> Result<Vec<i64>, MigrationError> {
    let applied = applied(pool).await?;
    let mut applied_map = std::collections::BTreeMap::new();
    for (version, _, recorded) in applied {
        applied_map.insert(version, recorded);
    }

    for migration in MIGRATIONS {
        if let Some(recorded) = applied_map.get(&migration.version) {
            let computed = checksum(migration.up);
            if recorded != &computed {
                return Err(MigrationError::ChecksumMismatch {
                    version: migration.version,
                    recorded: recorded.clone(),
                    computed,
                });
            }
            continue;
        }
        apply_one(pool, migration).await?;
    }

    Ok(MIGRATIONS
        .iter()
        .filter(|migration| !applied_map.contains_key(&migration.version))
        .map(|migration| migration.version)
        .collect())
}

async fn apply_one(pool: &PgPool, migration: &Migration) -> Result<(), MigrationError> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|source| MigrationError::Database {
            version: migration.version,
            source,
        })?;

    sqlx::raw_sql(migration.up)
        .execute(&mut *tx)
        .await
        .map_err(|source| MigrationError::Database {
            version: migration.version,
            source,
        })?;

    sqlx::query("insert into schema_migrations (version, name, checksum) values ($1, $2, $3)")
        .bind(migration.version)
        .bind(migration.name)
        .bind(checksum(migration.up))
        .execute(&mut *tx)
        .await
        .map_err(|source| MigrationError::Database {
            version: migration.version,
            source,
        })?;

    tx.commit()
        .await
        .map_err(|source| MigrationError::Database {
            version: migration.version,
            source,
        })
}

/// Rolls back the most recently applied migration.
///
/// Returns the rolled-back version, or `None` when no migration is applied.
///
/// # Errors
///
/// Returns [`MigrationError::Irreversible`] when the migration has no rollback script.
pub async fn rollback_last(pool: &PgPool) -> Result<Option<i64>, MigrationError> {
    let applied = applied(pool).await?;
    let Some((version, _, _)) = applied.last() else {
        return Ok(None);
    };

    let migration = MIGRATIONS
        .iter()
        .find(|migration| migration.version == *version)
        .ok_or(MigrationError::UnknownVersion { version: *version })?;

    let down = migration.down.ok_or_else(|| MigrationError::Irreversible {
        version: *version,
        reason: migration
            .irreversible_reason
            .unwrap_or("no rollback documented")
            .to_string(),
    })?;

    let mut tx = pool
        .begin()
        .await
        .map_err(|source| MigrationError::Database {
            version: *version,
            source,
        })?;

    sqlx::raw_sql(down)
        .execute(&mut *tx)
        .await
        .map_err(|source| MigrationError::Database {
            version: *version,
            source,
        })?;

    sqlx::query("delete from schema_migrations where version = $1")
        .bind(version)
        .execute(&mut *tx)
        .await
        .map_err(|source| MigrationError::Database {
            version: *version,
            source,
        })?;

    tx.commit()
        .await
        .map_err(|source| MigrationError::Database {
            version: *version,
            source,
        })?;

    Ok(Some(*version))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn versions_are_strictly_increasing_and_unique() {
        let mut previous = 0;
        for migration in MIGRATIONS {
            assert!(
                migration.version > previous,
                "version {} must be greater than {previous}",
                migration.version
            );
            previous = migration.version;
        }
    }

    #[test]
    fn names_are_unique_and_non_empty() {
        let mut names = std::collections::BTreeSet::new();
        for migration in MIGRATIONS {
            assert!(!migration.name.is_empty());
            assert!(
                names.insert(migration.name),
                "duplicate name {}",
                migration.name
            );
        }
    }

    #[test]
    fn every_migration_has_forward_sql_and_documented_rollback() {
        for migration in MIGRATIONS {
            assert!(
                migration.up.trim().len() > 100,
                "migration {} forward SQL looks empty",
                migration.version
            );
            assert!(
                migration.down.is_some() || migration.irreversible_reason.is_some(),
                "migration {} must document rollback or irreversibility",
                migration.version
            );
        }
    }

    #[test]
    fn checksum_is_deterministic_and_sensitive() {
        let first = checksum("select 1");
        assert_eq!(first, checksum("select 1"));
        assert_ne!(first, checksum("select 2"));
        assert_eq!(first.len(), 16);
    }

    #[test]
    fn canonical_schema_uses_date_columns_for_date_only_concepts() {
        let sql = schema::CANONICAL_SCHEMA_UP;
        assert!(
            sql.contains("schedule_date date"),
            "schedule date must be a date"
        );
        assert!(sql.contains("deadline date"), "deadline must be a date");
        assert!(
            !sql.contains("schedule_date timestamp"),
            "date-only values must not be timestamps"
        );
    }

    #[test]
    fn canonical_schema_covers_required_persistence_domains() {
        let sql = schema::CANONICAL_SCHEMA_UP;
        for table in [
            "accounts",
            "sessions",
            "entities",
            "placements",
            "tag_assignments",
            "checklist_items",
            "recurrence_templates",
            "recurrence_instances",
            "reminders",
            "preferences",
            "sync_metadata",
            "change_log",
            "tombstones",
        ] {
            assert!(
                sql.contains(&format!("create table if not exists {table}")),
                "missing table {table}"
            );
        }
    }

    fn table_definition(sql: &str, table: &str) -> String {
        let start = sql
            .find(&format!("create table if not exists {table}"))
            .unwrap_or_else(|| panic!("table {table} present"));
        let end = sql[start..]
            .find(");")
            .map(|offset| start + offset)
            .expect("table terminator");
        sql[start..end].to_string()
    }

    #[test]
    fn every_user_owned_table_is_account_scoped() {
        let sql = schema::CANONICAL_SCHEMA_UP;
        for table in [
            "entities",
            "placements",
            "checklist_items",
            "reminders",
            "tombstones",
            "recurrence_templates",
            "tag_assignments",
        ] {
            let definition = table_definition(sql, table);
            assert!(
                definition.contains("account_id uuid not null"),
                "{table} must be account-scoped"
            );
        }
    }

    #[test]
    fn mutable_entity_tables_carry_optimistic_versions() {
        let sql = schema::CANONICAL_SCHEMA_UP;
        for table in ["entities", "placements", "checklist_items", "reminders"] {
            let definition = table_definition(sql, table);
            assert!(
                definition.contains("version bigint not null default 1"),
                "{table} must carry optimistic version metadata"
            );
            assert!(
                definition.contains("updated_at timestamptz not null"),
                "{table} must carry an update timestamp"
            );
        }
    }

    #[test]
    fn placements_keep_context_and_projection_ordering_independent() {
        let sql = schema::CANONICAL_SCHEMA_UP;
        assert!(sql.contains("scope text not null check (scope in ('context', 'today'))"));
        assert!(sql.contains("create unique index if not exists placements_entity_scope_idx"));
    }

    #[test]
    fn change_log_and_tombstones_support_sync() {
        let sql = schema::CANONICAL_SCHEMA_UP;
        assert!(sql.contains("create unique index if not exists change_log_operation_idx"));
        assert!(sql.contains("purge_after timestamptz"));
    }
}
