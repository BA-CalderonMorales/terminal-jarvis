#!/usr/bin/env sh
# Publish (or retag) the npm channels for one release version. The registry is
# eventually consistent -- npm acknowledges a write before it serves it -- so
# this polls to convergence. It lives here, not in the workflow's `run:` block,
# so `scripts/bash/verify/index.sh publish-rehearsal` runs these exact bytes.
# npm >= 11.5.1 is required for OIDC trusted publishing (docs/development.md).
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/../../../../" && pwd)
cd "$root"
check_toolchain=0
version=
artifacts_dir=artifacts
dist_tags="latest stable beta"
required_npm=11.5.1
timeout=180
poll=10

fail() { echo "publish-npm: $*" >&2; exit 1; }
usage() {
  cat <<'EOF'
usage: scripts/bash/release/index.sh publish-npm [options]

  --check-toolchain      Run only the npm version guard and exit.
  --version X.Y.Z        Version to publish (default: Cargo.toml).
  --artifacts-dir DIR    Holds DIR/<version>/<platform>/npm (default: artifacts).
  --dist-tags "a b c"    Dist-tags to move (default: "latest stable beta").
  --required-npm X.Y.Z   Minimum npm version (default: 11.5.1).
  --timeout SECONDS      Convergence timeout per fact (default: 180).
  --poll-interval SEC    Delay between convergence reads (default: 10).
EOF
}
while [ "$#" -gt 0 ]; do
  case "$1" in
    --check-toolchain) check_toolchain=1 ;;
    --version) version=${2:?}; shift ;;
    --artifacts-dir) artifacts_dir=${2:?}; shift ;;
    --dist-tags) dist_tags=${2:?}; shift ;;
    --required-npm) required_npm=${2:?}; shift ;;
    --timeout) timeout=${2:?}; shift ;;
    --poll-interval) poll=${2:?}; shift ;;
    -h | --help) usage; exit 0 ;;
    *) fail "unknown option $1" ;;
  esac
  shift
done

require_npm() {
  npm_version=$(npm --version)
  echo "npm $npm_version"
  oldest=$(printf '%s\n%s\n' "$required_npm" "$npm_version" | sort -V | head -n 1)
  [ "$oldest" = "$required_npm" ] ||
    fail "npm >= $required_npm required for OIDC, found $npm_version"
}
# A write is acknowledged before it is served, so a single read proves nothing.
# Poll to convergence and fail only on timeout.
wait_for_version() {
  elapsed=0
  until npm view "terminal-jarvis@$version" version >/dev/null 2>&1; do
    [ "$elapsed" -lt "$timeout" ] || fail "timed out waiting for terminal-jarvis@$version"
    sleep "$poll"
    elapsed=$((elapsed + poll))
  done
  echo "confirmed: terminal-jarvis@$version is readable"
}

wait_for_tag() {
  elapsed=0
  until npm dist-tag ls terminal-jarvis 2>/dev/null | grep -q "^$1: $version\$"; do
    [ "$elapsed" -lt "$timeout" ] || fail "timed out waiting for dist-tag $1 -> $version"
    sleep "$poll"
    elapsed=$((elapsed + poll))
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

pkg="$artifacts_dir/$version/linux-x64-gnu/npm/terminal-jarvis"
test -f "$pkg/package.json" || fail "npm stage missing: $pkg/package.json"
chmod +x "$pkg/bin/terminal-jarvis"
test -x "$pkg/bin/terminal-jarvis" || fail "npm wrapper not executable: $pkg/bin/terminal-jarvis"
(cd "$pkg" && npm pack --dry-run --loglevel error)

if npm view "terminal-jarvis@$version" version >/dev/null 2>&1; then
  echo "terminal-jarvis@$version is already published; moving tags only"
else
  (cd "$pkg" && npm publish --tag latest --provenance)
fi
wait_for_version
for tag in $dist_tags; do npm dist-tag add "terminal-jarvis@$version" "$tag"; done
for tag in $dist_tags; do wait_for_tag "$tag"; done
echo "publish-npm: terminal-jarvis@$version live on $dist_tags"
