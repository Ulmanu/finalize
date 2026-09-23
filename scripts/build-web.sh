#!/usr/bin/env bash
# Builds the Rust/WASM client for Momentum.
#
# Delegates to `cargo xtask web-build`, which compiles the same Rust view tree as the server
# for wasm32-unknown-unknown with the `hydrate` feature. When the target is not installed in
# the active toolchain, the task reports an explicit non-fatal limitation (exit 0) so that
# unattended runs record the environmental gap instead of failing silently; genuine compiler
# errors still fail the command.
set -euo pipefail

cd "$(dirname "$0")/.."

# Prefer a rustup-managed toolchain when present so rustfmt/clippy/wasm targets resolve
# consistently; fall back to whatever cargo is on PATH.
if [[ -x "$HOME/.cargo/bin/cargo" ]]; then
  export PATH="$HOME/.cargo/bin:$PATH"
fi
exec cargo xtask web-build "$@"
