#!/usr/bin/env sh
# Publish (or retag) the npm channels for one release version. The registry is
# eventually consistent, so this polls to convergence; it lives here, not in
# the workflow's `run:` block, so `publish-rehearsal` runs these exact bytes.
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/../../../../" && pwd)
cd "$root"
check_toolchain=0
version=
artifacts_dir=artifacts
dist_tags="latest stable beta"
timeout=180; poll=10
provenance=

fail() { echo "publish-npm: $*" >&2; exit 1; }
usage() {
  cat <<'EOF'
usage: scripts/bash/release/index.sh publish-npm [options]

  --check-toolchain   Guard only and exit.
  --version X.Y.Z     Version to publish (default: Cargo.toml).
  --artifacts-dir D   Holds D/<version>/<platform>/npm.
  --dist-tags "a b"   Tags to move; the first is the publish tag.
  --provenance        Publish with an explicit attestation.
EOF
}
while [ "$#" -gt 0 ]; do
  case "$1" in
    --check-toolchain) check_toolchain=1 ;;
    --version) version=${2:?}; shift ;;
    --artifacts-dir) artifacts_dir=${2:?}; shift ;;
    --dist-tags) dist_tags=${2:?}; shift ;;
    --provenance) provenance=--provenance ;;
    --timeout) timeout=${2:?}; shift ;;
    --poll-interval) poll=${2:?}; shift ;;
    -h | --help) usage; exit 0 ;;
    *) fail "unknown option $1" ;;
  esac
  shift
done

# dist-tags over OIDC need npm 11.21.0+ (11.x) or 12.2.0+ (12.x); a client that
# can publish but not retag would ship a release with stale channels.
dist_tag_capable() { # npm version
  major=${1%%.*}; minor=${1#*.}; minor=${minor%%.*}
  case "$major" in
    11) [ "$minor" -ge 21 ] ;;
    12) [ "$minor" -ge 2 ] ;;
    *) [ "$major" -ge 13 ] ;;
  esac
}
require_npm() {
  npm_version=$(npm --version)
  echo "npm $npm_version"
  dist_tag_capable "$npm_version" ||
    fail "npm 11.21.0+/12.2.0+ required for OIDC dist-tags, found $npm_version"
}
# A write is acknowledged before it is served: poll, and fail only on timeout.
wait_for_version() {
  elapsed=0
  until npm view "terminal-jarvis@$version" version >/dev/null 2>&1; do
    [ "$elapsed" -lt "$timeout" ] || fail "timed out waiting for terminal-jarvis@$version"
    sleep "$poll"; elapsed=$((elapsed + poll))
  done
  echo "confirmed: terminal-jarvis@$version is readable"
}
wait_for_tag() {
  elapsed=0
  until npm dist-tag ls terminal-jarvis 2>/dev/null | grep -q "^$1: $version\$"; do
    [ "$elapsed" -lt "$timeout" ] || fail "timed out waiting for dist-tag $1 -> $version"
    sleep "$poll"; elapsed=$((elapsed + poll))
  done
  echo "confirmed: $1 -> $version"
}
require_npm
if [ "$check_toolchain" = "1" ]; then
  echo "publish-npm: toolchain ok"
  exit 0
fi

[ -n "$version" ] || version=$(sed -n 's/^version = "\([^"]*\)"/\1/p' Cargo.toml | head -n 1)
test -n "$version" || fail "Cargo.toml version missing"
first_tag=${dist_tags%% *}
pkg="$artifacts_dir/$version/linux-x64-gnu/npm/terminal-jarvis"
test -f "$pkg/package.json" || fail "npm stage missing: $pkg/package.json"
chmod +x "$pkg/bin/terminal-jarvis" && test -x "$pkg/bin/terminal-jarvis" || fail "npm wrapper not executable: $pkg/bin/terminal-jarvis"
(cd "$pkg" && npm pack --dry-run --loglevel error)

if npm view "terminal-jarvis@$version" version >/dev/null 2>&1; then
  echo "terminal-jarvis@$version is already published; moving tags only"
else
  (cd "$pkg" && npm publish --tag "$first_tag" $provenance)
fi
wait_for_version
for tag in $dist_tags; do npm dist-tag add "terminal-jarvis@$version" "$tag"; done
for tag in $dist_tags; do wait_for_tag "$tag"; done
echo "publish-npm: terminal-jarvis@$version live on $dist_tags"
