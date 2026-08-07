#!/usr/bin/env sh
set -eu

cargo metadata --format-version 1 >/dev/null
require_tools=${TJ_REQUIRE_SECURITY_TOOLS:-0}

missing() {
  if test "$require_tools" = "1"; then
    echo "$1 is required but not installed" >&2
    exit 1
  fi
  echo "$1 not installed; skipping $2"
}

if command -v cargo-audit >/dev/null 2>&1; then
  cargo audit
else
  missing cargo-audit "advisory scan"
fi

if command -v cargo-deny >/dev/null 2>&1; then
  cargo deny check
else
  missing cargo-deny "dependency policy scan"
fi

if command -v npm >/dev/null 2>&1 && test -f npm/terminal-jarvis/package-lock.json; then
  npm --prefix npm/terminal-jarvis audit --omit=dev --audit-level=moderate
else
  echo "npm audit unavailable; skipping npm package scan"
fi
