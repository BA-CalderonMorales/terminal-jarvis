#!/usr/bin/env sh
# Prove the npm publisher against an eventually-consistent fake registry: a
# write is served only after N reads. Covers publish, retag, single-tag, and
# dry-run; with --from REF the pre-fix body must publish and then fail.
# No network. Options: --from REF --version V --reads N.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/../../../../" && pwd)
cd "$root"
from= version= reads=3
while [ "$#" -gt 0 ]; do
  case "$1" in
    --from) from=${2:?}; shift ;;
    --version) version=${2:?}; shift ;;
    --reads) reads=${2:?}; shift ;;
    -h | --help) sed -n '2,5p' "$0"; exit 0 ;;
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

publish() { # registry dir [extra args]
  reg=$1; dir=$2; shift 2
  TERMINAL_JARVIS_REHEARSAL_REGISTRY="$reg" TERMINAL_JARVIS_REHEARSAL_READS="$reads" \
    PATH="$work/bin:$PATH" scripts/bash/release/index.sh publish-npm \
    --version "$version" --artifacts-dir "$dir/artifacts" --poll-interval 0 "$@"
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

echo "publish-rehearsal: single-tag path (manual channel workflow)"
stage "$work/single"
publish "$work/reg-single" "$work/single" --dist-tags stable ||
  fail "the single-tag publisher failed"
test "$(cat "$work/reg-single/tags/stable")" = "$version" ||
  fail "stable did not converge to $version"
echo "  single: stable -> $version"

echo "publish-rehearsal: dry run writes nothing"
stage "$work/dry"
publish "$work/reg-dry" "$work/dry" --dry-run >"$work/dry.out" 2>&1 ||
  fail "the dry run failed"
test ! -f "$work/reg-dry/version" || fail "the dry run wrote to the registry"
grep -q "dry-run: would publish" "$work/dry.out" || fail "the dry run did not say what it would do"
echo "  dry run: no write"

echo "publish-rehearsal: npm toolchain guard"
scripts/bash/verify/logic/npm-toolchain-rehearsal.sh "$work/bin" "$work" ||
  fail "the toolchain guard matrix failed"

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
