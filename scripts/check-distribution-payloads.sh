#!/usr/bin/env sh
set -eu

npm_stage=

fail() {
  echo "distribution-payloads: $*" >&2
  exit 1
}

usage() {
  echo "usage: scripts/check-distribution-payloads.sh --npm-stage PATH" >&2
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --npm-stage) npm_stage=${2:?missing path}; shift ;;
    -h | --help) usage; exit 0 ;;
    *) usage; exit 2 ;;
  esac
  shift
done

test -n "$npm_stage" || { usage; exit 2; }
test -d "$npm_stage" || fail "npm stage missing: $npm_stage"

test ! -e "$npm_stage/bin/terminal-jarvis-bin" ||
  fail "npm package must not include terminal-jarvis-bin"

for name in opencode codex claude gemini aider goose jules; do
  if find "$npm_stage" -type f -name "$name" | grep . >/dev/null; then
    fail "npm package must not include harness binary: $name"
  fi
done

if command -v npm >/dev/null 2>&1; then
  : "${NPM_CONFIG_CACHE:=${TMPDIR:-/tmp}/terminal-jarvis-npm-cache}"
  export NPM_CONFIG_CACHE
  files=$(cd "$npm_stage" && npm pack --dry-run --json --loglevel error)
  if echo "$files" | grep -q '"path": "bin/terminal-jarvis-bin"'; then
    fail "npm pack includes terminal-jarvis-bin"
  fi
fi
