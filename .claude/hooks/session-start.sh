#!/bin/bash
set -euo pipefail

# Remote-mode (Claude Code cloud) bootstrap
if [ "${CLAUDE_CODE_REMOTE:-}" = "true" ]; then
  if ! command -v cargo >/dev/null 2>&1; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile minimal
    # shellcheck disable=SC1091
    source "$HOME/.cargo/env"
  fi
  rustup component add rustfmt clippy --toolchain stable >/dev/null 2>&1 || true
fi

# Warm the build cache so the first edit-verify loop is fast.
cargo build --quiet 2>/dev/null || true
