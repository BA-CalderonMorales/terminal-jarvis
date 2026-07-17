#!/usr/bin/env sh
set -eu

line_limit=100
coverage_target=90
mutation_target=90

fail() {
  echo "verify: $*" >&2
  exit 1
}

echo "[1/12] format"
cargo fmt --all -- --check

echo "[2/12] lint"
cargo clippy --all-targets -- -D warnings

echo "[3/12] tests"
cargo test

echo "[4/12] rust file length"
over_limit=$(find src tests -name '*.rs' -print0 |
  xargs -0 wc -l |
  awk -v max="$line_limit" '$1 > max && $2 != "total" {print}')
test -z "$over_limit" || fail "Rust files over ${line_limit} lines:\n$over_limit"

echo "[5/12] harness catalog shape"
harnesses=$(find harnesses -mindepth 1 -maxdepth 1 -type d | wc -l | tr -d ' ')
indexes=$(find harnesses -path '*/index.toml' | wc -l | tr -d ' ')
expected=$((harnesses * 10))
test "$harnesses" -gt 0 || fail "no harnesses found"
test "$indexes" -eq "$expected" ||
  fail "expected $expected harness index files, found $indexes"

echo "[6/12] generated support report"
scripts/generate-support-report.sh --check

echo "[7/12] cli smoke"
cargo run -- --plain list >/tmp/terminal-jarvis-list.txt
cargo run -- --plain plan codex headless >/tmp/terminal-jarvis-plan.txt
TERMINAL_JARVIS_HOME=/tmp/terminal-jarvis-verify cargo run -- --plain use codex >/dev/null
TERMINAL_JARVIS_HOME=/tmp/terminal-jarvis-verify cargo run -- --plain current |
  grep 'active harness = codex' >/dev/null

echo "[8/12] integration hardening"
scripts/integration-hardening.sh

echo "[9/12] security"
scripts/security-check.sh

echo "[10/12] distribution smoke"
scripts/release-preflight.sh
if command -v node >/dev/null 2>&1 && command -v npm >/dev/null 2>&1; then
  npm --prefix npm/terminal-jarvis test
  npm --prefix npm/terminal-jarvis run smoke
  scripts/check-distribution-payloads.sh --npm-stage npm/terminal-jarvis
else
  echo "node/npm not installed; skipping npm wrapper smoke"
fi

if command -v ruby >/dev/null 2>&1; then
  ruby -c homebrew/Formula/terminal-jarvis.rb
else
  echo "ruby not installed; skipping Homebrew formula syntax check"
fi
scripts/package-release.sh --check

echo "[11/12] coverage"
if command -v cargo-llvm-cov >/dev/null 2>&1; then
  cargo llvm-cov --fail-under-lines "$coverage_target"
else
  echo "cargo-llvm-cov not installed; skipping ${coverage_target}% line coverage gate"
fi

echo "[12/12] mutation"
if command -v cargo-mutants >/dev/null 2>&1 && test "${TJ_MUTATION:-0}" = "1"; then
  cargo mutants --config mutants.toml --minimum-test-timeout 30 --jobs 2
else
  echo "cargo-mutants not run; install it and set TJ_MUTATION=1 for ${mutation_target}% mutation work"
fi

echo "verify: ok"
