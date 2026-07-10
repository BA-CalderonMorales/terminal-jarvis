#!/usr/bin/env sh
set -eu

mode=${1:-build}
out_root=${2:-dist}

fail() {
  echo "package-release: $*" >&2
  exit 1
}

value_from_cargo() {
  sed -n "s/^$1 = \"\\([^\"]*\\)\"/\\1/p" Cargo.toml | head -n 1
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

powershell_literal() {
  printf "'%s'" "$(printf '%s' "$1" | sed "s/'/''/g")"
}

name=$(value_from_cargo name)
repo=$(value_from_cargo repository)
version=$(value_from_cargo version)
host_target=$(rustc -vV | sed -n 's/^host: //p')
target=${CARGO_BUILD_TARGET:-${TARGET:-}}
target_explicit=1
if test -z "$target"; then
  target=$host_target
  target_explicit=0
fi
case "$target" in
  x86_64-unknown-linux-gnu) platform=linux-x64-gnu ;;
  aarch64-unknown-linux-gnu) platform=linux-arm64-gnu ;;
  x86_64-unknown-linux-musl) platform=linux-x64-musl ;;
  aarch64-unknown-linux-musl) platform=linux-arm64-musl ;;
  x86_64-apple-darwin) platform=macos-x64 ;;
  aarch64-apple-darwin) platform=macos-arm64 ;;
  x86_64-pc-windows-msvc) platform=win32-x64 ;;
  *) platform=$target ;;
esac
archive=$name-$version-$platform.tar.gz
binary_name=$name
native_asset=$name-$version-$platform
zip_archive=
case "$target" in
  *-pc-windows-*)
    binary_name=$name.exe
    native_asset=$native_asset.exe
    zip_archive=$name-$version-$platform.zip
    ;;
esac

test -n "$name" || fail "Cargo.toml name missing"
test -n "$repo" || fail "Cargo.toml repository missing"
test -n "$version" || fail "Cargo.toml version missing"
test -n "$host_target" || fail "rustc host target missing"
test -n "$target" || fail "rustc build target missing"
test -d harnesses || fail "harnesses directory missing"
test -d gates || fail "gates directory missing"
test -x npm/terminal-jarvis/bin/terminal-jarvis || fail "npm wrapper missing"

scripts/release-preflight.sh

if command -v ruby >/dev/null 2>&1; then
  ruby -c homebrew/Formula/terminal-jarvis.rb >/dev/null
fi

if test "$mode" = "--check"; then
  echo "$name $version $platform ($target)"
  exit 0
fi
test "$mode" = "build" || fail "usage: scripts/package-release.sh [--check|build] [out-dir]"

git_sha=$(git rev-parse HEAD 2>/dev/null || echo unknown)
if test "$target_explicit" = "1"; then
  TERMINAL_JARVIS_GIT_SHA=$git_sha cargo build --release --locked --target "$target"
  release_dir=target/$target/release
else
  TERMINAL_JARVIS_GIT_SHA=$git_sha cargo build --release --locked
  release_dir=target/release
fi

dist=$out_root/$version/$platform
stage=$dist/package/$name-$version-$platform
npm_stage=$dist/npm/$name
formula_dir=$dist/homebrew/Formula
rm -rf "$dist"
mkdir -p "$stage/bin" "$npm_stage/bin" "$formula_dir"

cp "$release_dir/$binary_name" "$stage/bin/$binary_name"
cp "$release_dir/$binary_name" "$dist/$native_asset"
cp README.md LICENSE CHANGELOG.md "$stage/"
cp -R harnesses "$stage/"
cp -R gates "$stage/"
chmod +x "$stage/bin/$binary_name"
chmod +x "$dist/$native_asset"

(cd "$dist/package" && tar -czf "../$archive" "$name-$version-$platform")
(cd "$dist" && sha256_file "$archive" | sed 's/ \*/  /' >"$archive.sha256")
(cd "$dist" && sha256_file "$native_asset" | sed 's/ \*/  /' >"$native_asset.sha256")
if test -n "$zip_archive"; then
  command -v powershell.exe >/dev/null 2>&1 || fail "powershell.exe is required for Windows ZIP packaging"
  source=$(powershell_literal "$dist/package/$name-$version-$platform")
  destination=$(powershell_literal "$dist/$zip_archive")
  powershell.exe -NoProfile -NonInteractive -Command \
    "Compress-Archive -LiteralPath $source -DestinationPath $destination -Force"
  (cd "$dist" && sha256_file "$zip_archive" | sed 's/ \*/  /' >"$zip_archive.sha256")
fi
sha=$(cut -d ' ' -f 1 "$dist/$archive.sha256")

cp npm/terminal-jarvis/package.json "$npm_stage/"
cp npm/terminal-jarvis/postinstall.js "$npm_stage/"
cp README.md "$npm_stage/"
cp npm/terminal-jarvis/bin/README.txt "$npm_stage/bin/"
cp npm/terminal-jarvis/bin/terminal-jarvis "$npm_stage/bin/"
chmod +x "$npm_stage/bin/terminal-jarvis"

cat >"$formula_dir/terminal-jarvis.rb" <<EOF
class TerminalJarvis < Formula
  desc "Data-driven harness switcher for AI coding agents"
  homepage "$repo"
  url "$repo/releases/download/v$version/$archive"
  sha256 "$sha"
  license "MIT"

  def install
    bin.install "bin/$binary_name" => "$name"
    pkgshare.install "harnesses", "gates"
  end

  test do
    assert_match "terminal-jarvis", shell_output("#{bin}/terminal-jarvis --help")
  end
end
EOF

if command -v npm >/dev/null 2>&1; then
  (cd "$npm_stage" && npm pack --dry-run --loglevel error >/dev/null)
fi
if command -v ruby >/dev/null 2>&1; then
  ruby -c "$formula_dir/terminal-jarvis.rb" >/dev/null
fi
scripts/check-distribution-payloads.sh --npm-stage "$npm_stage"

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
tar -xzf "$dist/$archive" -C "$tmp"
test -x "$tmp/$name-$version-$platform/bin/$binary_name" ||
  fail "archive missing executable bin/$binary_name"
test -d "$tmp/$name-$version-$platform/harnesses" ||
  fail "archive missing harnesses"
test -d "$tmp/$name-$version-$platform/gates" ||
  fail "archive missing gates"

if test "$target" = "$host_target"; then
  expected=$(find harnesses -mindepth 1 -maxdepth 1 -type d | wc -l | tr -d ' ')
  actual=$("$tmp/$name-$version-$platform/bin/$binary_name" --plain list | wc -l | tr -d ' ')
  test "$actual" = "$expected" || fail "archive smoke listed $actual of $expected harnesses"
  native=$(cd "$dist" && pwd)/$native_asset
  actual=$(cd "$tmp" && TERMINAL_JARVIS_CATALOG= "$native" --plain list | wc -l | tr -d ' ')
  test "$actual" = "$expected" || fail "standalone binary listed $actual of $expected harnesses"

  if command -v node >/dev/null 2>&1; then
    npm_stage_abs=$(cd "$npm_stage" && pwd)
    TERMINAL_JARVIS_BIN="$stage/bin/$binary_name" TERMINAL_JARVIS_CATALOG="$stage/harnesses" \
      node "$npm_stage_abs/bin/terminal-jarvis" --plain list >/dev/null
  fi

  scripts/integration-hardening.sh \
    --binary "$stage/bin/$binary_name" \
    --catalog "$stage/harnesses" \
    --npm-wrapper "$npm_stage/bin/terminal-jarvis" \
    --homebrew-formula "$formula_dir/terminal-jarvis.rb"
  scripts/core-command-matrix.sh \
    --binary "$stage/bin/$binary_name" \
    --catalog "$stage/harnesses"
else
  echo "package-release: skipped execution smoke for cross target $target on $host_target"
fi

echo "$dist/$archive"
echo "$dist/$archive.sha256"
echo "$dist/$native_asset"
echo "$dist/$native_asset.sha256"
test -z "$zip_archive" || echo "$dist/$zip_archive"
test -z "$zip_archive" || echo "$dist/$zip_archive.sha256"
echo "$npm_stage"
echo "$formula_dir/terminal-jarvis.rb"
