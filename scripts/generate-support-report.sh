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
report=$(mktemp)
trap 'rm -f "$tmp" "$report"' EXIT
"$root/scripts/phase03-catalog-report.sh" \
  --output "$report" --tested-ref support-matrix >/dev/null

render() {
  printf '%s\n' '# Terminal Jarvis support matrix' ''
  printf '%s\n' 'Generated from the validated harness catalog. Do not edit by hand.' ''
  printf '%s\n' 'Catalog presence is not a support claim. Each row reports its own guard state and primary evidence tier.' ''
  printf '%s\n' '| Harness | Capability | Support | Evidence | Effect | Network | Interaction | Platforms | Executable | Source | Freshness |' \
    '|---|---|---|---|---|---|---|---|---|---|---|'
  while IFS=$'\t' read -r schema tested_ref name capability support evidence guard argv effect platforms executable source verified_at summary result; do
    [[ $schema == 1 ]] || continue
    network=${effect#*;network=}
    network=${network%%;*}
    interaction=${effect##*;interaction=}
    effect=${effect%%;*}
    printf '| %s | %s | %s | %s | %s | %s | %s | %s | `%s` | `%s` | %s |\n' \
      "$name" "$capability" "$support" "$evidence" "$effect" "$network" \
      "$interaction" "$platforms" "$executable" "$source" "$verified_at"
  done <"$report"
  printf '%s\n' '' '## First-class decisions' ''
  printf '%s\n' 'Promotion is fail-closed. Catalog data may prove a candidate is not eligible; it cannot replace the required Phase 03 deterministic and disposable-real evidence.' ''
  printf '%s\n' '| Candidate | Rows | Verified | Disposable-real | Blocking states | Decision |' '|---|---:|---:|---:|---:|---|'
  for candidate in opencode codex claude gemini hermes; do
    rows=$(awk -F '\t' -v name="$candidate" '$3 == name { count++ } END { print count + 0 }' "$report")
    verified=$(awk -F '\t' -v name="$candidate" '$3 == name && $5 == "verified" { count++ } END { print count + 0 }' "$report")
    disposable=$(awk -F '\t' -v name="$candidate" '$3 == name && $6 == "disposable-real" { count++ } END { print count + 0 }' "$report")
    blocking=$(awk -F '\t' -v name="$candidate" '$3 == name && $5 ~ /^(stub|unsupported|disabled|unknown)$/ { count++ } END { print count + 0 }' "$report")
    if [[ $rows -ne 9 || $verified -eq 0 || $disposable -eq 0 || $blocking -ne 0 ]]; then
      decision='not promoted; catalog prerequisites fail'
    else
      decision='not promoted; Phase 03 promotion evidence still required'
    fi
    printf '| %s | %s | %s | %s | %s | %s |\n' \
      "$candidate" "$rows" "$verified" "$disposable" "$blocking" "$decision"
  done
  printf '%s\n' '' 'The five decisions above are derived from current catalog rows. Fake execution alone never promotes a candidate.'
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
