#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
platform= dist= output= ref=
while (($#)); do
  case "$1" in
    --platform) platform=${2:?missing platform}; shift 2 ;;
    --dist) dist=${2:?missing dist}; shift 2 ;;
    --output) output=${2:?missing output}; shift 2 ;;
    --ref) ref=${2:?missing ref}; shift 2 ;;
    *) printf 'delivery-matrix: unknown option %s\n' "$1" >&2; exit 2 ;;
  esac
done
[[ -n "$platform" && -n "$dist" && -n "$output" && -n "$ref" ]] || {
  printf 'delivery-matrix: platform, dist, output, and ref are required\n' >&2
  exit 2
}
[[ -d "$dist" && -d $(dirname "$output") ]] || exit 4
dist=$(cd "$dist" && pwd)
version=0.1.13
binary=terminal-jarvis
[[ "$platform" != win32-x64 ]] || binary=terminal-jarvis.exe
stage="$dist/package/terminal-jarvis-$version-$platform"
npm_stage="$dist/npm/terminal-jarvis"
archive="$dist/terminal-jarvis-$version-$platform.tar.gz"
[[ "$platform" != win32-x64 ]] || archive="$dist/terminal-jarvis-$version-$platform.zip"
tmp=$(mktemp -d)
trap 'test -z "${brew_installed:-}" || brew uninstall --force terminal-jarvis >/dev/null 2>&1 || true; rm -rf -- "$tmp"' EXIT

check() {
  local path=$1 text
  text=$("$path" --plain version --verbose)
  grep -F "terminal-jarvis $version" <<<"$text" >/dev/null
  grep -F "git commit: $ref" <<<"$text" >/dev/null
  [[ $("$path" --plain list | wc -l) -eq 25 ]]
  "$path" --plain show codex | grep -F 'support:' >/dev/null
}
record() { printf '1\t%s\t%s\t%s\t%s\n' "$ref" "$platform" "$1" "$2" >>"$output"; }

printf 'schema_version\tref\tplatform\tchannel\tresult\n' >"$output"
CARGO_NET_OFFLINE=true TERMINAL_JARVIS_GIT_SHA="$ref" cargo install --quiet \
  --path "$root" --root "$tmp/cargo" --locked --force
check "$tmp/cargo/bin/$binary"
record cargo pass

npm install --silent --offline --ignore-scripts --global --prefix "$tmp/npm-global" "$npm_stage"
npm_wrapper=$(find "$tmp/npm-global" -type f -path '*/terminal-jarvis/bin/terminal-jarvis' | head -1)
[[ -n "$npm_wrapper" ]]
TERMINAL_JARVIS_BIN="$stage/bin/$binary" node "$npm_wrapper" --version | grep " $version" >/dev/null
record npm-global pass

npm install --silent --offline --ignore-scripts --prefix "$tmp/npx" "$npm_stage"
TERMINAL_JARVIS_BIN="$stage/bin/$binary" npm exec --offline --prefix "$tmp/npx" -- \
  terminal-jarvis --version | grep " $version" >/dev/null
record npx pass

if [[ "$platform" == win32-x64 ]]; then
  record homebrew unsupported
else
  [[ "${CI:-}" == true && -n "${RUNNER_TEMP:-}" ]] || {
    printf 'delivery-matrix: Homebrew install requires a disposable CI runner\n' >&2
    exit 4
  }
  command -v brew >/dev/null || { printf 'delivery-matrix: brew is required\n' >&2; exit 4; }
  formula="$tmp/terminal-jarvis.rb"
  cp "$dist/homebrew/Formula/terminal-jarvis.rb" "$formula"
  escaped=${archive//\/\\}; escaped=${escaped//&/\\&}
  sed -i.bak "s|^  url .*|  url \"file://$escaped\"|" "$formula"
  HOMEBREW_NO_AUTO_UPDATE=1 HOMEBREW_NO_INSTALL_CLEANUP=1 \
    brew install --formula "$formula" >/dev/null
  brew_installed=1
  check "$(brew --prefix terminal-jarvis)/bin/terminal-jarvis"
  record homebrew pass
fi

mkdir "$tmp/archive"
if [[ "$platform" == win32-x64 ]]; then
  powershell.exe -NoProfile -NonInteractive -Command \
    "Expand-Archive -LiteralPath '$archive' -DestinationPath '$tmp/archive'"
else
  tar -xzf "$archive" -C "$tmp/archive"
fi
archive_binary=$(find "$tmp/archive" -type f -path "*/bin/$binary" | head -1)
[[ -n "$archive_binary" ]]
chmod +x "$archive_binary"
check "$archive_binary"
record direct-archive pass

check "$dist/terminal-jarvis-$version-$platform${binary#terminal-jarvis}"
record direct-executable pass
printf 'delivery-matrix: ok (%s)\n' "$platform"
