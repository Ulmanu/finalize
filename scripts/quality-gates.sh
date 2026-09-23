#!/usr/bin/env bash
# Baseline quality gates for the Momentum workspace.
#
# Runs, in order:
#   1. cargo fmt --all -- --check
#   2. cargo clippy --workspace --all-targets --all-features -- -D warnings
#   3. cargo test --workspace --all-features
#   4. openspec validate --specs --strict --no-interactive
#   5. cargo xtask arch-check
#
# Usage: scripts/quality-gates.sh [--quick]
#   --quick skips clippy and tests (formatting + architecture + OpenSpec only).
set -euo pipefail

cd "$(dirname "$0")/.."

# Prefer a rustup-managed toolchain when present so rustfmt/clippy/wasm targets resolve
# consistently; fall back to whatever cargo is on PATH.
if [[ -x "$HOME/.cargo/bin/cargo" ]]; then
  export PATH="$HOME/.cargo/bin:$PATH"
fi

QUICK=0
if [[ "${1:-}" == "--quick" ]]; then
  QUICK=1
fi

step() {
  echo ""
  echo "==> $1"
}

step "cargo fmt --all -- --check"
cargo fmt --all -- --check

step "cargo xtask arch-check"
cargo xtask arch-check

step "openspec validate --specs --strict --no-interactive"
openspec validate --specs --strict --no-interactive

if [[ "$QUICK" -eq 0 ]]; then
  step "cargo clippy --workspace --all-targets --all-features -- -D warnings"
  cargo clippy --workspace --all-targets --all-features -- -D warnings

  step "cargo test --workspace --all-features"
  cargo test --workspace --all-features
fi

echo ""
echo "All quality gates passed."
