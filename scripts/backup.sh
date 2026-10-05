#!/usr/bin/env bash
# Database backup with retention and restore verification.
#
# Usage:
#   scripts/backup.sh backup            # dump to $BACKUP_DIR with retention
#   scripts/backup.sh verify <dump>     # restore into a scratch database and sanity-check
#
# Environment: DATABASE_URL (required), BACKUP_DIR (default ./backups),
# BACKUP_RETENTION (default 14 dumps).
set -euo pipefail

command="${1:-backup}"
backup_dir="${BACKUP_DIR:-./backups}"
retention="${BACKUP_RETENTION:-14}"

require_url() {
  if [[ -z "${DATABASE_URL:-}" ]]; then
    echo "DATABASE_URL is required" >&2
    exit 1
  fi
}

case "$command" in
  backup)
    require_url
    mkdir -p "$backup_dir"
    stamp="$(date -u +%Y%m%dT%H%M%SZ)"
    dump="$backup_dir/momentum-$stamp.dump"
    pg_dump --format=custom --no-owner --file "$dump" "$DATABASE_URL"
    echo "wrote $dump"
    # Retention: keep the newest $retention dumps.
    ls -1t "$backup_dir"/momentum-*.dump 2>/dev/null | tail -n +$((retention + 1)) | while read -r old; do
      rm -f "$old"
      echo "pruned $old"
    done
    ;;
  verify)
    require_url
    dump="${2:-}"
    if [[ -z "$dump" || ! -f "$dump" ]]; then
      echo "usage: scripts/backup.sh verify <dump>" >&2
      exit 1
    fi
    scratch="momentum_restore_check_$$"
    psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -c "drop database if exists $scratch"
    psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -c "create database $scratch"
    restore_url="${DATABASE_URL%/*}/$scratch"
    pg_restore --no-owner --exit-on-error --dbname "$restore_url" "$dump"
    accounts="$(psql "$restore_url" -Atc "select count(*) from accounts")"
    migrations="$(psql "$restore_url" -Atc "select count(*) from schema_migrations")"
    psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -c "drop database $scratch"
    echo "restore verified: $accounts accounts, $migrations migrations"
    ;;
  *)
    echo "usage: scripts/backup.sh [backup|verify <dump>]" >&2
    exit 1
    ;;
esac
