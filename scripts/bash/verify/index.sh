#!/usr/bin/env sh
set -eu

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
cmd=${1:-}
[ $# -gt 0 ] && shift

usage() {
  cat <<'EOF'
usage: scripts/bash/verify/index.sh <verify|local-ci|security-check|integration-hardening|distribution-payloads|core-command-matrix> [args...]

Public entrypoint to the verify domain. Run from the repository root; the
verify gate truth is produced here.

Commands:
  verify
  local-ci [--strict] [--mutation] [--no-package]
  security-check
  integration-hardening [--binary PATH] [--catalog PATH]
  distribution-payloads --npm-stage PATH
  core-command-matrix [--binary PATH] [--catalog PATH]
  pre-commit
  install-hooks
EOF
}

case "$cmd" in
  verify) exec "$here/logic/verify.sh" "$@" ;;
  local-ci) exec "$here/logic/local-ci.sh" "$@" ;;
  security-check) exec "$here/logic/security-check.sh" "$@" ;;
  integration-hardening) exec "$here/logic/integration-hardening.sh" "$@" ;;
  distribution-payloads) exec "$here/logic/check-distribution-payloads.sh" "$@" ;;
  core-command-matrix) exec "$here/logic/core-command-matrix.sh" "$@" ;;
  pre-commit) exec "$here/logic/pre-commit.sh" "$@" ;;
  install-hooks) exec "$here/logic/install-hooks.sh" "$@" ;;
  -h | --help) usage; exit 0 ;;
  "")
    usage >&2
    exit 2
    ;;
  *)
    echo "verify index: unknown command $cmd" >&2
    usage >&2
    exit 2
    ;;
esac
