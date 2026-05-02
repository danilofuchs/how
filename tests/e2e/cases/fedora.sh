#!/usr/bin/env bash
# E2E assertions for the fedora image.
set -uo pipefail

fail=0

assert_how() {
  local cmd=$1 expected=$2 out
  if ! out=$(how "$cmd" 2>&1); then
    echo "FAIL: how $cmd exited non-zero"
    echo "$out" | sed 's/^/  | /'
    fail=$((fail + 1))
    return
  fi
  if echo "$out" | grep -qE "installed by ${expected}\b"; then
    echo "OK:   how $cmd → $expected"
  else
    echo "FAIL: how $cmd did not report '$expected'"
    echo "$out" | sed 's/^/  | /'
    fail=$((fail + 1))
  fi
}

# jq is installed via dnf in the Dockerfile.
assert_how jq dnf

if (( fail > 0 )); then
  echo "$fail assertion(s) failed"
  exit 1
fi
echo "all assertions passed"
