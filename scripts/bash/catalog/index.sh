#!/usr/bin/env sh
set -eu

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
cmd=${1:-}
[ $# -gt 0 ] && shift

usage() {
  cat <<'EOF'
usage: scripts/bash/catalog/index.sh <support-report|catalog-report|adversarial-report|parity-catalog> [args...]

Public entrypoint to the catalog domain. Run from any directory; logic
scripts resolve the repository root from their own location.

Commands:
  support-report [--check]
  catalog-report --output PATH [--binary PATH] [--catalog PATH] [--tested-ref REF]
  adversarial-report --output PATH [--tested-ref REF]
  parity-catalog DEV-BINARY STAGED-ROOT OUTPUT-DIR
EOF
}

case "$cmd" in
  support-report) exec "$here/logic/generate-support-report.sh" "$@" ;;
  catalog-report) exec "$here/logic/catalog-report.sh" "$@" ;;
  adversarial-report) exec "$here/logic/adversarial-report.sh" "$@" ;;
  parity-catalog) exec "$here/logic/parity-catalog.sh" "$@" ;;
  -h | --help) usage; exit 0 ;;
  "")
    usage >&2
    exit 2
    ;;
  *)
    echo "catalog index: unknown command $cmd" >&2
    usage >&2
    exit 2
    ;;
esac
