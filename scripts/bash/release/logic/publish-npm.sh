#!/usr/bin/env sh
# Publish (or retag) the npm channels for one release version. The registry is
# eventually consistent, so this polls to convergence. It lives in the release
# domain, not in a workflow `run:` block, so the rehearsal runs these exact
# bytes; the CD job and the three manual channel workflows all call it.
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/../../../../" && pwd)
cd "$root"

fail() { echo "publish-npm: $*" >&2; exit 1; }
check_toolchain=0
version=
artifacts_dir=artifacts
dist_tags="latest stable beta"
timeout=600; poll=10
provenance=
dry_run=0

usage() {
  cat <<'EOF'
usage: scripts/bash/release/index.sh publish-npm [options]

  --check-toolchain   Guard only and exit.
  --version X.Y.Z     Version to publish (default: Cargo.toml).
  --artifacts-dir D   Holds D/<version>/<platform>/npm.
  --dist-tags "a b"   Tags to move; the first is the publish tag.
  --provenance        Publish with an explicit attestation.
  --dry-run           Validate and print the action; touch no registry.
  --timeout SECONDS   Serve-convergence window (default 600).
EOF
}
while [ "$#" -gt 0 ]; do
  case "$1" in
    --check-toolchain) check_toolchain=1 ;;
    --version) version=${2:?}; shift ;;
    --artifacts-dir) artifacts_dir=${2:?}; shift ;;
    --dist-tags) dist_tags=${2:?}; shift ;;
    --provenance) provenance=--provenance ;;
    --dry-run) dry_run=1 ;;
    --timeout) timeout=${2:?}; shift ;;
    --poll-interval) poll=${2:?}; shift ;;
    -h | --help) usage; exit 0 ;;
    *) fail "unknown option $1" ;;
  esac
  shift
done

. "$root/scripts/bash/release/logic/npm-registry.sh"
require_npm
if [ "$check_toolchain" = "1" ]; then
  echo "publish-npm: toolchain ok"
  exit 0
fi

cargo_version=$(sed -n 's/^version = "\([^"]*\)"/\1/p' Cargo.toml | head -n 1)
test -n "$cargo_version" || fail "Cargo.toml version missing"
[ -n "$version" ] || version=$cargo_version
test "$version" = "$cargo_version" ||
  fail "version $version does not match Cargo $cargo_version"
first_tag=${dist_tags%% *}
pkg="$artifacts_dir/$version/linux-x64-gnu/npm/terminal-jarvis"
test -f "$pkg/package.json" || fail "npm stage missing: $pkg/package.json"
chmod +x "$pkg/bin/terminal-jarvis" && test -x "$pkg/bin/terminal-jarvis" ||
  fail "npm wrapper not executable: $pkg/bin/terminal-jarvis"
(cd "$pkg" && npm pack --dry-run --loglevel error)

if npm view "terminal-jarvis@$version" version >/dev/null 2>&1; then
  if [ "$dry_run" = "1" ]; then
    echo "dry-run: published; would move $dist_tags to $version"
    exit 0
  fi
  echo "terminal-jarvis@$version is already published; moving tags only"
else
  if [ "$dry_run" = "1" ]; then
    echo "dry-run: would publish $version (tag $first_tag) and move $dist_tags"
    exit 0
  fi
  (cd "$pkg" && npm publish --tag "$first_tag" $provenance)
fi
wait_for_version "$version"
for tag in $dist_tags; do npm dist-tag add "terminal-jarvis@$version" "$tag"; done
for tag in $dist_tags; do wait_for_tag "$tag" "$version"; done
echo "publish-npm: terminal-jarvis@$version live on $dist_tags"
