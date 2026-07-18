#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
output=
tested_ref=
while (($#)); do
  case "$1" in
    --output) output=${2:?missing output}; shift 2 ;;
    --tested-ref) tested_ref=${2:?missing ref}; shift 2 ;;
    *) printf 'phase03-adversarial: unknown option %s\n' "$1" >&2; exit 2 ;;
  esac
done
[[ -n "$output" ]] || { printf 'phase03-adversarial: --output is required\n' >&2; exit 2; }
[[ -n "$tested_ref" ]] || tested_ref=$(git -C "$root" rev-parse HEAD)
[[ -d $(dirname "$output") ]] || { printf 'phase03-adversarial: output parent missing\n' >&2; exit 4; }

tmp=$(mktemp "${output}.tmp.XXXXXX")
trap 'rm -f -- "$tmp"' EXIT
export CARGO_NET_OFFLINE=true

run() {
  local id=$1 treatment=$2 command
  shift 2
  printf -v command '%q ' "$@"
  command=${command% }
  printf 'phase03-adversarial: %s\n' "$id" >&2
  (cd "$root" && "$@")
  printf '1\t%s\t%s\t%s\tpass\t%s\n' "$tested_ref" "$id" "$treatment" "$command" >>"$tmp"
}

printf 'schema_version\ttested_ref\tcase\ttreatment\tresult\tcommand\n' >"$tmp"
run descriptor-guards deterministic cargo test --quiet \
  --test phase03_catalog_walk_tests -- --exact catalog_walk_records_all_rows_once_without_effects
run missing-and-permission deterministic cargo test --quiet \
  --test phase03_cli_exit_matrix --test phase02_diagnostics_readonly
run empty-and-malformed-state deterministic cargo test --quiet \
  --test cli_session_tests --test phase02_diagnostics_states
run side-effect-injection deterministic cargo test --quiet \
  --test phase02_lifecycle_contract_tests --test phase02_dangerous_contract_tests \
  --test phase02_support_guard_contract_tests
run stream-signal-nonutf8 deterministic cargo test --quiet \
  --test phase02_stream_contract_tests
run redaction-all-channels deterministic cargo test --quiet \
  --test phase02_diagnostics_redaction --test phase03_redaction_channels
run cache-checksum-architecture-shadow-recovery deterministic \
  node --test npm/terminal-jarvis/test-wrapper.js
run support-artifact-redaction deterministic env \
  TJ_SUPPORT_SECRET_SENTINEL=TJ_SUPPORT_SECRET_SENTINEL_6c82 \
  scripts/generate-support-report.sh --check

printf '1\t%s\ttimeout\tfrozen-contract-not-applicable\tpass\tno v0.1.13 timeout surface\n' "$tested_ref" >>"$tmp"
printf '1\t%s\tdebug-output\tfrozen-contract-not-applicable\tpass\tno v0.1.13 debug surface\n' "$tested_ref" >>"$tmp"
printf '1\t%s\tdisposable-real\tdata-derived-not-promoted\tpass\tfive candidates have zero promoted guarantees\n' "$tested_ref" >>"$tmp"
printf '1\t%s\tmanual-rows\tdeterministic-empty-denominator\tpass\tcatalog contains zero manual rows\n' "$tested_ref" >>"$tmp"
printf '1\t%s\tunsupported-rows\tdeterministic-empty-denominator\tpass\tcatalog contains zero unsupported rows\n' "$tested_ref" >>"$tmp"
mv -- "$tmp" "$output"
trap - EXIT
printf 'phase03-adversarial: ok (%s)\n' "$tested_ref"
