#!/usr/bin/env bash
# E2E assertions for the fat Debian image.
#
# Each block: install a known package via one manager, then assert that
# `how <cmd>` reports that manager. We deliberately pick a different
# package per manager to avoid cross-manager attribution noise (except
# `node`, which is the shadowing test).
set -uo pipefail

fail=0

assert_how() {
  # assert_how <cmd> <expected-manager> [extra PATH prefix]
  local cmd=$1 expected=$2 path_prefix=${3:-} out
  if [[ -n $path_prefix ]]; then
    out=$(env PATH="$path_prefix:$PATH" how "$cmd" 2>&1) || true
  else
    out=$(how "$cmd" 2>&1) || true
  fi
  if echo "$out" | grep -qE "installed by ${expected}\b"; then
    echo "OK:   how $cmd → $expected"
  else
    echo "FAIL: how $cmd did not report '$expected'"
    echo "$out" | sed 's/^/  | /'
    fail=$((fail + 1))
  fi
}

section() { echo; echo "── $1 ──"; }

section "apt"
# jq comes preinstalled in the image via apt.
assert_how jq apt

section "npm"
npm install -g --silent cowsay >/dev/null
assert_how cowsay npm

section "pnpm"
export PNPM_HOME="${PNPM_HOME:-$HOME/.local/share/pnpm}"
export PATH="$PNPM_HOME:$PATH"
pnpm add -g json
assert_how json pnpm

section "bun"
# `tsc` (from typescript) is a real npm-published CLI; we install it via
# bun-only and check the resolver attributes it to bun, not npm.
bun add -g typescript
assert_how tsc bun

section "pip3"
pip3 install --break-system-packages --quiet black
assert_how black pip3

section "pipx"
pipx install --quiet pycowsay
export PATH="$HOME/.local/bin:$PATH"
assert_how pycowsay pipx

section "uv"
uv tool install --quiet ruff
assert_how ruff uv

section "gem"
gem install --no-document --silent pry >/dev/null
assert_how pry gem

section "go"
export GOBIN="${GOBIN:-$HOME/go/bin}"
mkdir -p "$GOBIN"
export PATH="$GOBIN:$PATH"
go install github.com/rakyll/hey@latest
assert_how hey go

section "nvm + node shadowing"
# Both apt-node (/usr/bin/node) and nvm-node ($NVM_DIR/versions/node/.../bin/node)
# exist. Whichever manager owns the resolved path wins. nvm's init prepends
# to PATH, and `how` itself invokes `bash -ic` which re-sources ~/.bashrc
# (where the nvm installer hooked itself), so node should attribute to nvm.
export NVM_DIR="${NVM_DIR:-$HOME/.nvm}"
# shellcheck disable=SC1091
. "$NVM_DIR/nvm.sh"
nvm install 24 >/dev/null
nvm use 24 >/dev/null
assert_how node nvm

echo
if (( fail > 0 )); then
  echo "$fail assertion(s) failed"
  exit 1
fi
echo "all assertions passed"
