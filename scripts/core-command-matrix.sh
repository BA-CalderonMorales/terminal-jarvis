#!/usr/bin/env sh
set -eu

binary=
catalog=harnesses

usage() {
  cat <<'EOF'
usage: scripts/core-command-matrix.sh [--binary PATH] [--catalog PATH]

Exercises every public Terminal Jarvis command without installing, updating, or
launching third-party coding agents. It validates rich default output and the
stable --plain form used by automation.
EOF
}

fail() {
  echo "core-command-matrix: $*" >&2
  exit 1
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --binary) binary=${2:?missing binary}; shift ;;
    --catalog) catalog=${2:?missing catalog}; shift ;;
    -h | --help) usage; exit 0 ;;
    *) fail "unknown option $1" ;;
  esac
  shift
done

if [ -z "$binary" ]; then
  cargo build --quiet
  binary=target/debug/terminal-jarvis
fi
test -x "$binary" || fail "binary is missing or not executable: $binary"
test -d "$catalog" || fail "catalog directory is missing: $catalog"

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
home=$tmp/home
expected=$(find "$catalog" -mindepth 1 -maxdepth 1 -type d | wc -l | tr -d ' ')

tj() {
  TERMINAL_JARVIS_CATALOG="$catalog" TERMINAL_JARVIS_HOME="$home" "$binary" "$@"
}

plain() {
  tj --plain "$@"
}

ok() {
  label=$1
  shift
  tj "$@" >"$tmp/$label.out" 2>"$tmp/$label.err" || fail "$label failed: $(cat "$tmp/$label.err")"
}

outcome() {
  label=$1
  expected_code=$2
  shift 2
  if tj "$@" >"$tmp/$label.out" 2>"$tmp/$label.err"; then
    actual_code=0
  else
    actual_code=$?
  fi
  test "$actual_code" = "$expected_code" ||
    fail "$label exited $actual_code, expected $expected_code: $(cat "$tmp/$label.err")"
}

bad() {
  label=$1
  shift
  if tj "$@" >"$tmp/$label.out" 2>"$tmp/$label.err"; then
    fail "$label unexpectedly succeeded"
  fi
}

contains() {
  grep -F "$2" "$1" >/dev/null || fail "$1 missing: $2"
}

table() {
  contains "$tmp/$1.out" "+"
  contains "$tmp/$1.out" "|"
}

for command in --help -h help; do
  label=help-$(printf '%s' "$command" | tr -d '-')
  ok "$label" "$command"
  contains "$tmp/$label.out" "Terminal Jarvis"
  table "$label"
done

for command in --version -v version; do
  label=version-$(printf '%s' "$command" | tr -d '-')
  ok "$label" "$command"
  contains "$tmp/$label.out" "terminal-jarvis"
done
ok version-verbose version --verbose
ok version-info --info
table version-verbose
table version-info

ok list list
ok tools tools
table list
table tools
plain list >"$tmp/plain-list.out"
test "$(wc -l <"$tmp/plain-list.out" | tr -d ' ')" = "$expected" || fail "plain list count changed"

outcome check 4 check
outcome status 4 status
table check
table status
ok current-before current
table current-before
plain current >"$tmp/plain-current-before.out"
contains "$tmp/plain-current-before.out" "active harness = none"
ok use use codex
ok current current
table use
table current
plain current >"$tmp/plain-current.out"
contains "$tmp/plain-current.out" "active harness = codex"

ok show show codex
ok info info codex
table show
table info
for capability in download update headless version stats models security yolo ui; do
  label=plan-$capability
  ok "$label" plan codex "$capability"
  table "$label"
  contains "$tmp/$label.out" "Plan: codex $capability"
done

ok update-summary update
table update-summary
ok auth auth
ok auth-manage auth manage
ok auth-help auth help codex
outcome auth-set 4 auth set codex
table auth
table auth-manage
table auth-help
contains "$tmp/auth-set.err" "does not persist credentials"
ok config config
ok config-show config show
ok config-path config path
outcome config-reset 4 config reset
table config
table config-show
table config-path
contains "$tmp/config-reset.err" "guidance-only"
ok cache cache
ok cache-status cache status
outcome cache-clear 4 cache clear
outcome cache-refresh 4 cache refresh
table cache
table cache-status
contains "$tmp/cache-clear.err" "guidance-only"
contains "$tmp/cache-refresh.err" "guidance-only"

ok security security
ok security-status security status
ok security-audit security audit
ok security-harness security codex
table security
table security-status
table security-audit
table security-harness
ok gate gate status
ok gate-list gate list
ok gate-enable gate enable trivy
ok gate-enabled gate status
ok gate-disable gate disable
table gate
table gate-list
table gate-enable
table gate-enabled
table gate-disable
plain gate status >"$tmp/plain-gate.out"
contains "$tmp/plain-gate.out" "gate: disabled"
if TERMINAL_JARVIS_CATALOG="$catalog" TERMINAL_JARVIS_HOME="$home" PATH="" \
  "$binary" gate run trivy >"$tmp/gate-run.out" 2>"$tmp/gate-run.err"; then
  fail "gate run unexpectedly succeeded without Trivy"
fi
contains "$tmp/gate-run.err" "optional gate 'trivy'"

bad experimental-disabled experimental dashboard
TERMINAL_JARVIS_CATALOG="$catalog" TERMINAL_JARVIS_HOME="$home" \
  TERMINAL_JARVIS_EXPERIMENTAL_UI=1 "$binary" experimental dashboard >"$tmp/experimental.out"
contains "$tmp/experimental.out" "Dashboard"
contains "$tmp/experimental.out" "+"
outcome templates 4 templates
outcome db 4 db
contains "$tmp/templates.err" "removed"
contains "$tmp/db.err" "removed"

TERMINAL_JARVIS_CATALOG="$tmp/missing" TERMINAL_JARVIS_HOME="$home" \
  TERMINAL_JARVIS_DISTRIBUTION=source "$binary" --update --dry-run >"$tmp/update-dry-run.out"
contains "$tmp/update-dry-run.out" "Self-Update Plan"
contains "$tmp/update-dry-run.out" "cargo install terminal-jarvis"
if TERMINAL_JARVIS_CATALOG="$catalog" TERMINAL_JARVIS_HOME="$tmp/no-active" \
  "$binary" run >"$tmp/run-no-active.out" 2>"$tmp/run-no-active.err"; then
  fail "run unexpectedly succeeded without an active harness"
fi
contains "$tmp/run-no-active.err" "no active harness"
bad direct-unknown missing-harness
bad install-unknown install missing-harness
bad update-unknown update missing-harness
bad cache-unknown cache unknown

TERMINAL_JARVIS_CATALOG="$catalog" TERMINAL_JARVIS_HOME="$home" \
  "$binary" --no-color list >"$tmp/no-color.out"
if grep -F "$(printf '\033')" "$tmp/no-color.out" >/dev/null; then
  fail "--no-color emitted ANSI escape sequences"
fi

echo "core-command-matrix: ok ($expected harnesses, rich and plain output)"
