#!/usr/bin/env sh
# Prove the npm publisher against a fake registry that serves a write only
# after N reads -- the eventual consistency that made the v0.1.20 release
# publish successfully and then fail its own verification. The new half runs
# the shipped publisher; with `--from REF` the pre-fix body is extracted from
# that ref's workflow `run:` step and must publish and still fail. Never
# touches the real registry, git tags, or the network.
#
#   --from REF    also run the pre-fix body from REF and show it fails.
#   --version V   version to rehearse (default: Cargo.toml version).
#   --reads N     reads before a written fact is served (default: 3).
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/../../../../" && pwd)
cd "$root"
from= version= reads=3
while [ "$#" -gt 0 ]; do
  case "$1" in
    --from) from=${2:?}; shift ;;
    --version) version=${2:?}; shift ;;
    --reads) reads=${2:?}; shift ;;
    -h | --help) sed -n '2,10p' "$0"; exit 0 ;;
    *) echo "publish-rehearsal: unknown option $1" >&2; exit 2 ;;
  esac
  shift
done
fail() { echo "publish-rehearsal: $*" >&2; exit 1; }
[ -n "$version" ] || version=$(sed -n 's/^version = "\([^"]*\)"/\1/p' Cargo.toml | head -n 1)
test -n "$version" || fail "Cargo.toml version missing"

# The rehearsed bytes must be the shipped bytes.
grep -q 'scripts/bash/release/index.sh publish-npm' .github/workflows/cd-multiplatform.yml ||
  fail "cd-multiplatform.yml no longer calls the rehearsed publisher"

work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
mkdir -p "$work/bin"
cp "$root/scripts/bash/verify/constants/fake-npm.sh" "$work/bin/npm"
chmod +x "$work/bin/npm"
extract_run=$root/scripts/bash/verify/constants/extract-run.rb
stage() { # dir
  pkg="$1/artifacts/$version/linux-x64-gnu/npm/terminal-jarvis"
  mkdir -p "$pkg/bin"
  printf '{"name":"terminal-jarvis","version":"%s"}\n' "$version" >"$pkg/package.json"
  printf '#!/bin/sh\nexit 0\n' >"$pkg/bin/terminal-jarvis"
  chmod +x "$pkg/bin/terminal-jarvis"
}

publish() { # registry dir
  TERMINAL_JARVIS_REHEARSAL_REGISTRY="$1" TERMINAL_JARVIS_REHEARSAL_READS="$reads" \
    PATH="$work/bin:$PATH" scripts/bash/release/index.sh publish-npm \
    --version "$version" --artifacts-dir "$2/artifacts" --poll-interval 0
}

echo "publish-rehearsal: new publisher vs a registry that serves after $reads reads"
stage "$work/new"
publish "$work/reg-new" "$work/new" || fail "the new publisher failed against the fake registry"
echo "  new: converged in $(cat "$work/reg-new/reads") reads"
publish "$work/reg-new" "$work/new" >"$work/retag.out" 2>&1 ||
  fail "the retag path failed against an already-published version"
grep -q "already published; moving tags only" "$work/retag.out" ||
  fail "the second run did not take the retag path"
echo "  retag: converged on the already-published version"

echo "publish-rehearsal: npm toolchain guard"
for npm_version in 11.20.9 11.21.0 12.1.9 12.2.0 13.0.0; do
  mkdir -p "$work/reg-guard"
  printf '%s' "$npm_version" >"$work/reg-guard/npm-version"
  expected=refused
  case "$npm_version" in 11.21.0 | 12.2.0 | 13.0.0) expected=accepted ;; esac
  got=refused
  if TERMINAL_JARVIS_REHEARSAL_REGISTRY="$work/reg-guard" PATH="$work/bin:$PATH" \
    scripts/bash/release/index.sh publish-npm --check-toolchain >/dev/null 2>&1; then
    got=accepted
  fi
  test "$got" = "$expected" || fail "npm $npm_version: expected $expected, got $got"
  echo "  npm $npm_version: $got"
done

if [ -z "$from" ]; then
  echo "publish-rehearsal: no --from ref; skipping the pre-fix half"
  echo "publish-rehearsal: ok"
  exit 0
fi

echo "publish-rehearsal: pre-fix body from $from vs the same registry"
git show "$from:.github/workflows/cd-multiplatform.yml" >"$work/pre.yml" ||
  fail "cannot read the workflow at $from"
ruby "$extract_run" "$work/pre.yml" "Publish or retag npm channels" >"$work/pre.sh" ||
  fail "no 'Publish or retag npm channels' step at $from"
stage "$work/pre"
if (cd "$work/pre" && TERMINAL_JARVIS_REHEARSAL_REGISTRY="$work/reg-pre" \
  TERMINAL_JARVIS_REHEARSAL_READS="$reads" PATH="$work/bin:$PATH" \
  NODE_AUTH_TOKEN=rehearsal VERSION="$version" sh "$work/pre.sh") >"$work/pre.out" 2>&1; then
  cat "$work/pre.out"
  fail "the pre-fix body passed; the 0.1.20 failure is no longer reproduced"
fi
test -f "$work/reg-pre/version" ||
  fail "the pre-fix body failed before publishing; that is not the 0.1.20 shape"
echo "  pre-fix: published $(cat "$work/reg-pre/version") then failed its own verification"
echo "publish-rehearsal: ok"
