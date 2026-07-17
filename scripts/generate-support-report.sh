#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
mode=write
if [[ ${1:-} == --check ]]; then
  mode=check
  shift
fi
output=${1:-"$root/docs/support-matrix.md"}
tmp=$(mktemp)
trap 'rm -f "$tmp"' EXIT

render() {
  printf '%s\n' '# Terminal Jarvis support matrix' ''
  printf '%s\n' 'Generated from the validated harness catalog. Do not edit by hand.' ''
  printf '%s\n' '| Harness | Capability states | Description |' '|---|---|---|'
  while IFS= read -r line; do
    name=${line%% support=*}
    rest=${line#* support=}
    support=${rest%% - *}
    description=${rest#* - }
    description=${description//|/\\|}
    printf '| %s | %s | %s |\n' "$name" "$support" "$description"
  done < <(cd "$root" && cargo run --quiet -- --plain list)
  printf '%s\n' '' '## First-class decisions' ''
  printf '%s\n' 'No candidate is promoted by v0.1.13 catalog evidence.' ''
  printf '%s\n' '| Candidate | Decision |' '|---|---|'
  for candidate in opencode codex claude gemini hermes; do
    printf '| %s | not promoted; zero verified capability guarantees |\n' "$candidate"
  done
  printf '%s\n' '' 'Capability-level evidence, source, platform scope, and freshness are shown by `terminal-jarvis show <harness>` and `terminal-jarvis plan <harness> <capability>`.'
}

render >"$tmp"
if [[ $mode == check ]]; then
  if ! cmp -s "$tmp" "$output"; then
    diff -u "$output" "$tmp" || true
    printf 'support report is stale; run scripts/generate-support-report.sh\n' >&2
    exit 1
  fi
  printf 'support report matches the catalog\n'
else
  mv "$tmp" "$output"
  trap - EXIT
fi
