#!/usr/bin/env sh
# Toolchain-guard rehearsal for publish-npm.sh. Uses the fake npm on PATH so
# the accepted/refused matrix is exercised without a registry.
#   usage: npm-toolchain-rehearsal.sh <fake-bin-dir> <work-dir>
set -eu
bin=$1
work=$2
fail() { echo "npm-toolchain-rehearsal: $*" >&2; exit 1; }
for npm_version in 11.20.9 11.21.0 12.1.9 12.2.0 13.0.0; do
  mkdir -p "$work/reg-guard"
  printf '%s' "$npm_version" >"$work/reg-guard/npm-version"
  expected=refused
  case "$npm_version" in 11.21.0 | 12.2.0 | 13.0.0) expected=accepted ;; esac
  got=refused
  if TERMINAL_JARVIS_REHEARSAL_REGISTRY="$work/reg-guard" PATH="$bin:$PATH" \
    scripts/bash/release/index.sh publish-npm --check-toolchain >/dev/null 2>&1; then
    got=accepted
  fi
  test "$got" = "$expected" || fail "npm $npm_version: expected $expected, got $got"
  echo "  npm $npm_version: $got"
done
