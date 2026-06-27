#!/usr/bin/env sh
set -eu

binary=
catalog=harnesses
formula=
npm_wrapper=
capabilities="download update headless version stats models security yolo ui"

usage() {
  cat <<'EOF'
usage: scripts/integration-hardening.sh [--binary PATH] [--catalog PATH]
                                      [--npm-wrapper PATH]
                                      [--homebrew-formula PATH]

Exercises the Terminal Jarvis CLI surface across harness and capability paths.
It plans lifecycle commands but does not run harness download, update,
headless, or yolo commands.
EOF
}

fail() {
  echo "integration-hardening: $*" >&2
  exit 1
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --binary) binary=$2; shift ;;
    --catalog) catalog=$2; shift ;;
    --npm-wrapper) npm_wrapper=$2; shift ;;
    --homebrew-formula) formula=$2; shift ;;
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
harnesses=$tmp/harnesses

for path in "$catalog"/*; do
  test -d "$path" || continue
  basename "$path"
done | sort >"$harnesses"
expected=$(wc -l <"$harnesses" | tr -d ' ')
test "$expected" -gt 0 || fail "catalog has no harnesses"

run_tj() {
  TERMINAL_JARVIS_CATALOG="$catalog" TERMINAL_JARVIS_HOME="$home" "$binary" "$@"
}

contains() {
  grep -F "$2" "$1" >/dev/null || fail "$1 missing expected text: $2"
}

line_count_is() {
  actual=$(wc -l <"$1" | tr -d ' ')
  test "$actual" = "$2" || fail "$1 has $actual lines, expected $2"
}

run_tj --help >"$tmp/help"
contains "$tmp/help" "terminal-jarvis"
contains "$tmp/help" "terminal-jarvis run [harness] [capability] [args...]"

run_tj list >"$tmp/list"
line_count_is "$tmp/list" "$expected"

run_tj check >"$tmp/check"
line_count_is "$tmp/check" "$expected"

run_tj use codex >"$tmp/use"
run_tj current >"$tmp/current"
contains "$tmp/current" "active harness = codex"

while IFS= read -r harness; do
  run_tj show "$harness" >"$tmp/show"
  contains "$tmp/show" "setup:"
  contains "$tmp/show" "agent loop:"
  for capability in $capabilities; do
    contains "$tmp/show" "$capability:"
  done
done <"$harnesses"

for capability in $capabilities; do
  run_tj plan codex "$capability" >"$tmp/plan"
  contains "$tmp/plan" "codex:$capability"
  contains "$tmp/plan" "command:"
  contains "$tmp/plan" "env:"
done
run_tj plan headless >"$tmp/active-plan"
contains "$tmp/active-plan" "codex:headless"

if run_tj unknown-command >"$tmp/unknown.out" 2>"$tmp/unknown.err"; then
  fail "unknown command unexpectedly succeeded"
fi
contains "$tmp/unknown.err" "unknown command"

if [ -z "$npm_wrapper" ] && [ -f npm/terminal-jarvis/bin/terminal-jarvis ]; then
  npm_wrapper=npm/terminal-jarvis/bin/terminal-jarvis
fi
if [ -n "$npm_wrapper" ]; then
  test -f "$npm_wrapper" || fail "npm wrapper is missing: $npm_wrapper"
  if command -v node >/dev/null 2>&1; then
    TERMINAL_JARVIS_BIN="$binary" TERMINAL_JARVIS_CATALOG="$catalog" \
      TERMINAL_JARVIS_HOME="$tmp/npm-home" node "$npm_wrapper" list >"$tmp/npm-list"
    line_count_is "$tmp/npm-list" "$expected"
  else
    echo "node not installed; skipping npm wrapper integration"
  fi
fi

if [ -z "$formula" ] && [ -f homebrew/Formula/terminal-jarvis.rb ]; then
  formula=homebrew/Formula/terminal-jarvis.rb
fi
if [ -n "$formula" ]; then
  test -f "$formula" || fail "Homebrew formula is missing: $formula"
  if command -v ruby >/dev/null 2>&1; then
    ruby -c "$formula" >/dev/null
  else
    echo "ruby not installed; skipping Homebrew formula syntax"
  fi
  contains "$formula" 'pkgshare.install "harnesses"'
  contains "$formula" 'terminal-jarvis --help'
fi

echo "integration-hardening: ok ($expected harnesses, 9 capabilities)"
