#!/bin/sh
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
report=0
case ${1:-} in
  "") ;;
  --report) report=1 ;;
  *) echo "evaluation: only the optional --report flag is accepted" >&2; exit 2 ;;
esac
test "$#" -le 1 || { echo "evaluation: no command input is accepted" >&2; exit 2; }

case $(uname -s):$(uname -m) in
  Linux:x86_64 | Linux:amd64) target=linux-x64-gnu ;;
  Linux:aarch64 | Linux:arm64) target=linux-arm64-gnu ;;
  Darwin:x86_64 | Darwin:amd64) target=macos-x64 ;;
  Darwin:arm64 | Darwin:aarch64) target=macos-arm64 ;;
  MINGW*:x86_64 | MSYS*:x86_64 | CYGWIN*:x86_64) target=win32-x64 ;;
  *) cat "$root/unsupported-transcript.txt"; exit 3 ;;
esac

tmp=$(mktemp -d "${TMPDIR:-/tmp}/terminal-jarvis-evaluation.XXXXXX")
terminal_state=$(stty -g 2>/dev/null || true)
cleanup() {
  test -z "$terminal_state" || stty "$terminal_state" 2>/dev/null || true
  rm -rf "$tmp"
}
trap cleanup EXIT HUP INT TERM
binary=$root/payloads/$target/terminal-jarvis
test "$target" != win32-x64 || binary=$binary.exe
test -x "$binary" || { echo "evaluation: verified payload missing for $target" >&2; exit 4; }
export TERMINAL_JARVIS_HOME=$tmp/home
export TERMINAL_JARVIS_CATALOG=$root/catalogs/harnesses
export TERMINAL_JARVIS_GATES=$root/catalogs/gates
export TERMINAL_JARVIS_GATE=off

echo "SIMULATED EVALUATION — no coding-agent harness will be executed"
"$binary" --plain version --verbose
"$binary" --plain list
"$binary" --plain show codex
"$binary" --plain plan codex version

if test "$report" = 1; then
  manifest=$(sha256sum "$root/manifest-v1.json" 2>/dev/null | awk '{print $1}' || shasum -a 256 "$root/manifest-v1.json" | awk '{print $1}')
  version=$("$binary" --version | awk '{print $2}')
  ref=$("$binary" --plain version --verbose | sed -n 's/^git commit: //p')
  printf '{"schema_version":1,"kit_digest":"sha256:%s","selected_target":"%s","terminal_jarvis_version":"%s","terminal_jarvis_ref":"%s","scenario_results":[{"code":"TJ-EVAL-001","result":"pass"}]}\n' "$manifest" "$target" "$version" "$ref"
fi
