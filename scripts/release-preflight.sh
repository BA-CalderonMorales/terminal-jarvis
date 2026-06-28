#!/usr/bin/env sh
set -eu

cd "$(dirname "$0")/.."

tag=
expected_main_ref=

fail() {
  echo "release-preflight: $*" >&2
  exit 1
}

usage() {
  cat <<'EOF'
usage: scripts/release-preflight.sh [--tag vX.Y.Z] [--expected-main-ref REF]

Checks release metadata before packaging or publishing.
EOF
}

value_from_cargo() {
  sed -n "s/^$1 = \"\\([^\"]*\\)\"/\\1/p" Cargo.toml | head -n 1
}

json_version() {
  sed -n 's/.*"version": "\([^"]*\)".*/\1/p' "$1" | head -n 1
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --tag) tag=${2:?missing tag}; shift ;;
    --expected-main-ref) expected_main_ref=${2:?missing ref}; shift ;;
    -h | --help) usage; exit 0 ;;
    *) fail "unknown option $1" ;;
  esac
  shift
done

cargo_version=$(value_from_cargo version)
npm_version=$(json_version npm/terminal-jarvis/package.json)
lock_version=$(json_version npm/terminal-jarvis/package-lock.json)

test -n "$cargo_version" || fail "Cargo.toml version missing"
test -n "$npm_version" || fail "npm package version missing"
test -n "$lock_version" || fail "npm package-lock version missing"
test "$npm_version" = "$cargo_version" ||
  fail "npm package version $npm_version does not match Cargo $cargo_version"
test "$lock_version" = "$cargo_version" ||
  fail "npm package-lock version $lock_version does not match Cargo $cargo_version"
grep -q "## \\[$cargo_version\\]" CHANGELOG.md ||
  fail "CHANGELOG.md missing ## [$cargo_version]"

if [ -n "$expected_main_ref" ] && [ -z "$tag" ]; then
  fail "--expected-main-ref requires --tag"
fi

if [ -n "$tag" ]; then
  printf '%s\n' "$tag" | grep -Eq '^v[0-9]+\.[0-9]+\.[0-9]+$' ||
    fail "release tag must look like vX.Y.Z: $tag"
  version=${tag#v}
  test "$version" = "$cargo_version" ||
    fail "release tag $tag does not match Cargo version $cargo_version"

  tag_commit=$(git rev-parse -q --verify "refs/tags/$tag^{commit}") ||
    fail "release tag not found: $tag"
  head_commit=$(git rev-parse -q --verify "HEAD^{commit}") ||
    fail "git HEAD is not a commit"
  test "$head_commit" = "$tag_commit" ||
    fail "checked-out HEAD $head_commit does not match $tag at $tag_commit"

  if [ -n "$expected_main_ref" ]; then
    main_commit=$(git rev-parse -q --verify "$expected_main_ref^{commit}") ||
      fail "expected main ref not found: $expected_main_ref"
    test "$tag_commit" = "$main_commit" ||
      fail "$tag points to $tag_commit but $expected_main_ref is $main_commit"
  fi
fi

echo "release preflight ok: $cargo_version"
