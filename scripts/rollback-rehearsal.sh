#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
output=
current_ref=$(git -C "$root" rev-parse HEAD)
baseline_ref=$(git -C "$root" rev-parse v0.1.12^{commit})
while (($#)); do
  case "$1" in
    --output) output=${2:?missing output}; shift 2 ;;
    --current-ref) current_ref=${2:?missing ref}; shift 2 ;;
    *) printf 'rollback-rehearsal: unknown option %s\n' "$1" >&2; exit 2 ;;
  esac
done
[[ -n "$output" && -d $(dirname "$output") ]] || {
  printf 'rollback-rehearsal: --output parent must exist\n' >&2
  exit 2
}
[[ "$current_ref" == "$(git -C "$root" rev-parse HEAD)" ]] || {
  printf 'rollback-rehearsal: current ref must be checked out\n' >&2
  exit 4
}

tmp=$(mktemp -d)
trap 'rm -rf -- "$tmp"' EXIT
mkdir -p "$tmp/baseline-src" "$tmp/install/0.1.12" "$tmp/install/0.1.13"
git -C "$root" archive "$baseline_ref" | tar -x -C "$tmp/baseline-src"
(cd "$tmp/baseline-src" && CARGO_NET_OFFLINE=true CARGO_TARGET_DIR="$tmp/baseline-target" \
  TERMINAL_JARVIS_GIT_SHA="$baseline_ref" cargo build --release --locked --quiet)
CARGO_NET_OFFLINE=true CARGO_TARGET_DIR="$tmp/current-target" \
  TERMINAL_JARVIS_GIT_SHA="$current_ref" cargo build --manifest-path "$root/Cargo.toml" \
  --release --locked --quiet

cp "$tmp/baseline-target/release/terminal-jarvis" "$tmp/install/0.1.12/terminal-jarvis"
cp "$tmp/current-target/release/terminal-jarvis" "$tmp/install/0.1.13/terminal-jarvis"
cp -R "$tmp/baseline-src/harnesses" "$tmp/install/0.1.12/harnesses"
cp -R "$root/harnesses" "$tmp/install/0.1.13/harnesses"
chmod +x "$tmp/install/"*/terminal-jarvis

sha() { sha256sum "$1" | cut -d ' ' -f 1; }
version() { "$1/terminal-jarvis" --version | awk '{print $2}'; }
rows() { TERMINAL_JARVIS_CATALOG="$1/harnesses" "$1/terminal-jarvis" --plain list | wc -l; }
[[ $(version "$tmp/install/0.1.12") == 0.1.12 ]]
[[ $(version "$tmp/install/0.1.13") == 0.1.13 ]]
[[ $(rows "$tmp/install/0.1.12") -eq 25 && $(rows "$tmp/install/0.1.13") -eq 25 ]]

ln -s "$tmp/install/0.1.13" "$tmp/active"
[[ $(version "$tmp/active") == 0.1.13 ]]
ln -s "$tmp/install/0.1.12" "$tmp/rollback-next"
mv -Tf "$tmp/rollback-next" "$tmp/active"
[[ $(version "$tmp/active") == 0.1.12 ]]

cp "$tmp/install/0.1.13/terminal-jarvis" "$tmp/interrupted"
printf 'interrupted\n' >"$tmp/interrupted"
[[ $(sha "$tmp/interrupted") != $(sha "$tmp/install/0.1.13/terminal-jarvis") ]]
[[ $(version "$tmp/active") == 0.1.12 ]]
ln -s "$tmp/install/0.1.13" "$tmp/forward-next"
mv -Tf "$tmp/forward-next" "$tmp/active"
[[ $(version "$tmp/active") == 0.1.13 ]]

(cd "$root" && node --test npm/terminal-jarvis/test-wrapper.js >/dev/null)
cat >"$output" <<EOF
schema_version	current_ref	baseline_ref	current_sha256	baseline_sha256	rows	result
1	$current_ref	$baseline_ref	$(sha "$tmp/install/0.1.13/terminal-jarvis")	$(sha "$tmp/install/0.1.12/terminal-jarvis")	25	pass
EOF
printf 'rollback-rehearsal: ok (%s -> %s -> %s)\n' "$current_ref" "$baseline_ref" "$current_ref"
