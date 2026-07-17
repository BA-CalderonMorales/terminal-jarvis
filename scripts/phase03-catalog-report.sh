#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage: scripts/phase03-catalog-report.sh --output PATH [options]

Generate the deterministic Phase 03 catalog coverage report.

Options:
  --output PATH      Required TSV destination; replaced atomically.
  --binary PATH      Probe this Terminal Jarvis binary instead of the dev binary.
  --catalog PATH     Load this catalog instead of the source catalog.
  --tested-ref REF   Record this tested ref instead of the current commit.
  -h, --help         Show this help.
EOF
}

output=
binary=
catalog=
tested_ref=
while (($#)); do
  case "$1" in
    --output|--binary|--catalog|--tested-ref)
      (($# >= 2)) || { printf 'error: %s requires a value\n' "$1" >&2; exit 2; }
      value=$2
      case "$1" in
        --output) output=$value ;;
        --binary) binary=$value ;;
        --catalog) catalog=$value ;;
        --tested-ref) tested_ref=$value ;;
      esac
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      printf 'error: unknown option: %s\n' "$1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

[[ -n "$output" ]] || { printf 'error: --output is required\n' >&2; exit 2; }
[[ -z "$binary" || -x "$binary" ]] || {
  printf 'error: binary is not executable: %s\n' "$binary" >&2
  exit 4
}
[[ -z "$catalog" || -d "$catalog" ]] || {
  printf 'error: catalog is not a directory: %s\n' "$catalog" >&2
  exit 4
}

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
if [[ -z "$tested_ref" ]]; then
  tested_ref=$(git -C "$root" rev-parse --verify HEAD)
  [[ -z $(git -C "$root" status --porcelain) ]] || tested_ref+="-dirty"
fi
parent=$(dirname "$output")
[[ -d "$parent" ]] || { printf 'error: output directory is missing: %s\n' "$parent" >&2; exit 4; }
temporary=$(mktemp "${output}.tmp.XXXXXX")
trap 'rm -f -- "$temporary"' EXIT

export TJ_PHASE03_TESTED_REF=$tested_ref
export TJ_PHASE03_REPORT_PATH=$temporary
export CARGO_NET_OFFLINE=true
[[ -z "$binary" ]] || export TJ_PHASE03_BIN=$binary
[[ -z "$catalog" ]] || export TJ_PHASE03_CATALOG=$catalog

printf 'Generating Phase 03 catalog report for %s\n' "$tested_ref" >&2
cargo test --quiet --manifest-path "$root/Cargo.toml" \
  --test phase03_catalog_walk_tests -- --exact catalog_walk_records_all_rows_once_without_effects
mv -- "$temporary" "$output"
trap - EXIT
printf '%s\n' "$output"
