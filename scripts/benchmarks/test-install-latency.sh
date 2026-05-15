#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
SCRIPT="$SCRIPT_DIR/install-latency.sh"
TEST_DIR="$PROJECT_ROOT/tmp/install-latency-tests"

assert_contains() {
  local haystack="$1"
  local needle="$2"
  local message="$3"

  if ! grep -Fq -- "$needle" <<< "$haystack"; then
    echo "[ERROR] $message" >&2
    echo "[ERROR] Missing: $needle" >&2
    exit 1
  fi
}

test_help() {
  local output
  output="$("$SCRIPT" --help)"
  assert_contains "$output" "npx --yes terminal-jarvis@stable --version" "help must list stable npx benchmark"
  assert_contains "$output" "--include-brew" "help must document opt-in Homebrew benchmark"
  assert_contains "$output" "Recommended latency targets" "help must include latency targets"
}

test_matrix() {
  local output
  output="$("$SCRIPT" --list-matrix)"
  assert_contains "$output" $'npm\tcold\tnpx-terminal-jarvis@beta' "matrix must include beta npx cold benchmark"
  assert_contains "$output" $'npm\twarm\tnpx-terminal-jarvis@beta' "matrix must include beta npx warm benchmark"
  assert_contains "$output" $'npm\twarm\tnpm-install-g-terminal-jarvis' "matrix must include warm npm global install"
  assert_contains "$output" $'cargo\tcold\tcargo-install-terminal-jarvis' "matrix must include cargo baseline"
}

test_self_test() {
  "$SCRIPT" --work-dir "$TEST_DIR/self" --self-test >/dev/null
}

test_matrix_mutation_is_caught() {
  local mutant_dir="$TEST_DIR/mutant"
  local mutant="$mutant_dir/install-latency-mutant.sh"

  rm -rf "$mutant_dir"
  mkdir -p "$mutant_dir"
  cp "$SCRIPT" "$mutant"
  awk '$0 != "  \"terminal-jarvis@beta\"" { print }' "$mutant" > "$mutant.tmp"
  mv "$mutant.tmp" "$mutant"
  chmod +x "$mutant"

  if "$mutant" --work-dir "$mutant_dir/work" --self-test >/dev/null 2>&1; then
    echo "[ERROR] mutation smoke test survived after removing beta benchmark entry" >&2
    exit 1
  fi
}

main() {
  rm -rf "$TEST_DIR"
  mkdir -p "$TEST_DIR"

  test_help
  test_matrix
  test_self_test
  test_matrix_mutation_is_caught

  echo "[install-latency-test] all tests passed"
}

main "$@"
