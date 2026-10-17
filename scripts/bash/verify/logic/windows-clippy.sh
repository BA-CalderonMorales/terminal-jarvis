#!/usr/bin/env sh
# Prove the Windows strict-clippy gate with the same standard as claim-earned:
# the pre-fix tree warns under `cargo clippy --all-targets -- -D warnings`, and
# the fixed tree is clean. The repository's only `-D warnings` used to run on
# Linux, so a warning that fires only on Windows -- dead code behind a
# unix-only user -- reached the release. `.github/workflows/ci.yml` now runs
# this exact command in a `windows-latest` job; this script is the local twin.
#
#   --from REF   the pre-fix ref to rebuild and compare (required).
set -eu

root=$(cd "$(dirname -- "$0")/../../../../" && pwd)
from=
while [ $# -gt 0 ]; do
  case "$1" in
    --from) from=${2:?--from needs a ref}; shift 2 ;;
    *) echo "windows-clippy: unknown option $1" >&2; exit 2 ;;
  esac
done
[ -n "$from" ] || { echo "windows-clippy: --from REF is required" >&2; exit 2; }

fail() { echo "windows-clippy: $*" >&2; exit 1; }
work=$(mktemp -d)
cleanup() {
  [ -d "$work/old" ] && git -C "$root" worktree remove --force "$work/old" >/dev/null 2>&1 || true
  rm -rf "$work"
}
trap cleanup EXIT

echo "windows-clippy: building the pre-fix tree at $from"
git -C "$root" worktree add --detach "$work/old" "$from" >/dev/null 2>&1 ||
  fail "cannot create a worktree at $from"

check() { # path label
  if (cd "$1" && cargo clippy --all-targets -- -D warnings) >"$work/out" 2>&1; then
    echo "$2: clean (strict clippy passes)"
  else
    echo "$2: refused ($(grep -c '^error' "$work/out") deny-warnings errors)"
    grep -o 'due to [0-9]* previous errors\?' "$work/out" | sed 's/^/    /' | sort -u
  fi
}

check "$work/old" "old ($from)"
check "$root" "new (working tree)"
echo "windows-clippy: ok"
