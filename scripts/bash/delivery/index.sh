#!/usr/bin/env sh
set -eu

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
cmd=${1:-}
[ $# -gt 0 ] && shift

usage() {
  cat <<'EOF'
usage: scripts/bash/delivery/index.sh <matrix> [args...]

Public entrypoint to the delivery domain. Run from any directory; logic
scripts resolve the repository root from their own location.

Commands:
  matrix --platform PLATFORM --dist PATH --output PATH --ref REF
EOF
}

case "$cmd" in
  matrix) exec "$here/logic/delivery-matrix.sh" "$@" ;;
  -h | --help) usage; exit 0 ;;
  "")
    usage >&2
    exit 2
    ;;
  *)
    echo "delivery index: unknown command $cmd" >&2
    usage >&2
    exit 2
    ;;
esac
