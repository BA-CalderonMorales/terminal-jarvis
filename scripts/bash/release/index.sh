#!/usr/bin/env sh
set -eu

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
cmd=${1:-}
[ $# -gt 0 ] && shift

usage() {
  cat <<'EOF'
usage: scripts/bash/release/index.sh <preflight|package|local-cd|rollback-rehearsal> [args...]

Public entrypoint to the release domain. Run from any directory; the logic
scripts resolve the repository root from their own location.

Commands:
  preflight [--tag vX.Y.Z] [--expected-main-ref REF]
  package [build|--check] [out-dir]
  local-cd [--out-dir PATH] [--check-auth]
  rollback-rehearsal [--output PATH] [--current-ref REF]
EOF
}

case "$cmd" in
  preflight) exec "$here/logic/release-preflight.sh" "$@" ;;
  package) exec "$here/logic/package-release.sh" "$@" ;;
  local-cd) exec "$here/logic/local-cd.sh" "$@" ;;
  rollback-rehearsal) exec "$here/logic/rollback-rehearsal.sh" "$@" ;;
  -h | --help) usage; exit 0 ;;
  "")
    usage >&2
    exit 2
    ;;
  *)
    echo "release index: unknown command $cmd" >&2
    usage >&2
    exit 2
    ;;
esac
