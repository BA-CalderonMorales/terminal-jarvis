#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
ref=${TJ_PHASE03_REF:-$(git -C "$root" rev-parse HEAD)}
dev=${1:?usage: phase03-parity.sh DEV-BINARY STAGED-ROOT OUTPUT-DIR}
staged=${2:?usage: phase03-parity.sh DEV-BINARY STAGED-ROOT OUTPUT-DIR}
output=${3:?usage: phase03-parity.sh DEV-BINARY STAGED-ROOT OUTPUT-DIR}

sha_file() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | cut -d ' ' -f 1
  else
    shasum -a 256 "$1" | cut -d ' ' -f 1
  fi
}

tree_digest() {
  local base=$1 file
  while IFS= read -r file; do
    printf '%s\t%s\n' "${file#"$base"/}" "$(sha_file "$file")"
  done < <(find "$base" -type f | LC_ALL=C sort)
}

[[ -x "$dev" ]] || { printf 'dev binary is not executable: %s\n' "$dev" >&2; exit 4; }
[[ -x "$staged/bin/terminal-jarvis" ]] || { printf 'staged binary is missing\n' >&2; exit 4; }
[[ -d "$staged/harnesses" && -d "$staged/gates" ]] || { printf 'staged catalogs are missing\n' >&2; exit 4; }
mkdir -p "$output"

"$root/scripts/phase03-catalog-report.sh" --output "$output/dev.tsv" \
  --binary "$dev" --catalog "$root/harnesses" --tested-ref "$ref" >/dev/null
"$root/scripts/phase03-catalog-report.sh" --output "$output/staged.tsv" \
  --binary "$staged/bin/terminal-jarvis" --catalog "$staged/harnesses" \
  --tested-ref "$ref" >/dev/null
cmp "$output/dev.tsv" "$output/staged.tsv"

"$dev" --plain version --verbose >"$output/dev.version"
"$staged/bin/terminal-jarvis" --plain version --verbose >"$output/staged.version"
grep -F "git commit: $ref" "$output/dev.version" >/dev/null
grep -F "git commit: $ref" "$output/staged.version" >/dev/null
dev_version=$(sed -n '1p' "$output/dev.version")
staged_version=$(sed -n '1p' "$output/staged.version")
[[ "$dev_version" == "$staged_version" ]] || { printf 'version mismatch\n' >&2; exit 4; }

tree_digest "$root/harnesses" >"$output/dev.catalog.sha256"
tree_digest "$staged/harnesses" >"$output/staged.catalog.sha256"
tree_digest "$root/gates" >"$output/dev.gates.sha256"
tree_digest "$staged/gates" >"$output/staged.gates.sha256"
cmp "$output/dev.catalog.sha256" "$output/staged.catalog.sha256"
cmp "$output/dev.gates.sha256" "$output/staged.gates.sha256"

printf 'schema_version\tref\tversion\treport_sha256\tcatalog_sha256\tgates_sha256\tresult\n' >"$output/summary.tsv"
printf '1\t%s\t%s\t%s\t%s\t%s\tpass\n' "$ref" "${dev_version##* }" \
  "$(sha_file "$output/dev.tsv")" "$(sha_file "$output/dev.catalog.sha256")" \
  "$(sha_file "$output/dev.gates.sha256")" >>"$output/summary.tsv"
printf 'phase03 parity: ok (%s)\n' "$ref"
