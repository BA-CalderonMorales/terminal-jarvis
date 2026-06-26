#!/usr/bin/env sh
set -eu

repo=terminal-jarvis
out_root=${TJ_LOCAL_CD_OUT:-}
auth=0

usage() {
  cat <<'EOF'
usage: scripts/local-cd.sh [--out-dir PATH] [--check-auth]

Builds the local release package shape without tagging, pushing, or publishing.
Artifacts default to the workspace testing/ area when this checkout is there.

Options:
  --out-dir PATH  Write package and release-assets output under PATH.
  --check-auth    Report GitHub, npm, and Cargo auth boundaries without secrets.
  -h, --help      Show this help.
EOF
}

fail() {
  echo "local-cd: $*" >&2
  exit 1
}

version() {
  sed -n 's/^version = "\([^"]*\)"/\1/p' Cargo.toml | head -n 1
}

sha256_file() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1"
  elif command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$1"
  else
    fail "sha256sum or shasum is required"
  fi
}

check_auth() {
  if command -v gh >/dev/null 2>&1 && gh auth status >/dev/null 2>&1; then
    echo "github auth: ok for draft release checks"
  else
    echo "auth boundary: renew GitHub auth with repo contents write before release upload"
  fi

  if command -v npm >/dev/null 2>&1 && npm whoami >/dev/null 2>&1; then
    echo "npm auth: ok for registry identity"
  else
    echo "auth boundary: renew npm automation token with package publish rights"
  fi

  if test -n "${CARGO_REGISTRY_TOKEN:-}"; then
    echo "cargo auth: CARGO_REGISTRY_TOKEN is present"
  else
    echo "auth boundary: set CARGO_REGISTRY_TOKEN with crates.io publish scope"
  fi
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --out-dir) out_root=${2:?missing path}; shift ;;
    --check-auth) auth=1 ;;
    -h | --help) usage; exit 0 ;;
    *) echo "local-cd: unknown option $1" >&2; usage >&2; exit 2 ;;
  esac
  shift
done

cd "$(dirname "$0")/.."

if [ -z "$out_root" ]; then
  if [ -d ../../../testing ]; then
    out_root=../../../testing/$repo/local-cd
  else
    out_root=dist/local-cd
  fi
fi

version=$(version)
test -n "$version" || fail "Cargo.toml version missing"
tag=v$version

scripts/package-release.sh build "$out_root"

asset_dir=$out_root/release-assets/$tag
rm -rf "$asset_dir"
mkdir -p "$asset_dir"
find "$out_root/$version" -type f \( \
  -name "$repo-$version-*.tar.gz" -o \
  -name "$repo-$version-*.tar.gz.sha256" \
\) -exec cp {} "$asset_dir/" \;

archives=$(find "$asset_dir" -maxdepth 1 -name "$repo-$version-*.tar.gz" | wc -l | tr -d ' ')
checksums=$(find "$asset_dir" -maxdepth 1 -name "$repo-$version-*.tar.gz.sha256" | wc -l | tr -d ' ')
test "$archives" -gt 0 || fail "no release archives collected"
test "$archives" = "$checksums" || fail "$archives archives but $checksums checksums"

for archive in "$asset_dir"/$repo-$version-*.tar.gz; do
  checksum=$archive.sha256
  expected=$(cut -d ' ' -f 1 "$checksum")
  actual=$(sha256_file "$archive" | cut -d ' ' -f 1)
  test "$expected" = "$actual" || fail "checksum mismatch for $(basename "$archive")"
done

if [ "$auth" = "1" ]; then
  check_auth
fi

echo "local-cd: assets ready in $asset_dir"
