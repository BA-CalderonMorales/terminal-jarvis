#!/usr/bin/env sh
set -eu

# Exclusions: every exclude_re entry must still match a real mutant.
#
# An exclusion that matches nothing is dead weight with a dangerous failure
# mode. The mutant it was written to hide is no longer hidden -- cargo-mutants
# now generates and tests it -- and the reasoning that justified the exclusion
# is lost with it, so the next reader believes a lie about the code.
#
# Cargo-mutants reports excluded mutants as absent, so it cannot notice this
# on its own: the exclusion is indistinguishable from a rule that was always
# respected. The gate asks cargo-mutants for the whole universe of mutants,
# asks it again with each pattern applied, and fails when a pattern changes
# nothing. The pattern list and the matcher are the same ones the mutation
# run uses, so the gate cannot disagree with the run about what is excluded.
#
# The ledger lives in mutants.toml because that is where cargo-mutants reads
# it; the mutation runner passes --config mutants.toml for the same reason.
# Without that flag the exclusions are silently ignored.

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../../../.." && pwd)
cd "$root"

config=${TJ_MUTANTS_CONFIG:-mutants.toml}
tool=${TJ_CARGO_MUTANTS:-cargo mutants}

fail() {
  echo "mutation exclusions: $*" >&2
  exit 1
}

usage() {
  echo "usage: scripts/bash/mutation/index.sh exclusions"
}

[ $# -eq 0 ] || {
  usage >&2
  exit 2
}

test -f "$config" || fail "no mutation config at $config"
command -v "${tool%% *}" >/dev/null 2>&1 ||
  fail "cargo-mutants is required; install it with 'cargo install cargo-mutants --locked'"

# The exclude_re entries, one per line, with the TOML string escapes the
# ledger actually uses resolved. An escape the ledger does not use is a
# hard failure rather than a guess: a pattern the gate reads differently
# from cargo-mutants is a gate that does not gate.
patterns=$(sh "$here/exclusion-patterns.sh" "$config") || fail "the exclude_re ledger could not be read"

test -n "$patterns" || fail "the exclude_re ledger is empty"

universe=$(mktemp)
kept=$(mktemp)
ledger=$(mktemp)
report=$(mktemp)
trap 'rm -f "$universe" "$kept" "$ledger" "$report"' EXIT

# The universe: every mutant the tool would generate, with no exclusion
# applied. --no-config keeps the repository's own ledger out of the count.
$tool --list --no-config >"$universe" 2>/dev/null ||
  fail "cargo-mutants could not list the mutants"
total=$(wc -l <"$universe" | tr -d ' ')
test "$total" -gt 0 || fail "cargo-mutants listed no mutants at all"

stale=0
printf '%s\n' "$patterns" >"$ledger"
printf 'mutation exclusions: %s patterns against %s mutants\n' \
  "$(grep -c . "$ledger")" "$total"

while IFS= read -r pattern; do
  [ -n "$pattern" ] || continue
  $tool --list --no-config --exclude-re "$pattern" >"$kept" 2>/dev/null ||
    fail "cargo-mutants could not list the mutants for $pattern"
  remaining=$(wc -l <"$kept" | tr -d ' ')
  excluded=$((total - remaining))
  if [ "$excluded" -eq 0 ]; then
    printf 'STALE  %s\n' "$pattern" >>"$report"
  else
    printf 'live   %s excluded  %s\n' "$excluded" "$pattern" >>"$report"
  fi
done <"$ledger"

cat "$report"

if grep -q '^STALE' "$report"; then
  stale=$(grep -c '^STALE' "$report")
  echo >&2
  echo "mutation exclusions: $stale pattern(s) match no mutant." >&2
  echo "An exclusion that matches nothing hides nothing: the mutant it was" >&2
  echo "written to suppress is generated and tested again, and the reason it" >&2
  echo "was excluded is forgotten. Drop the pattern, or re-anchor it to the" >&2
  echo "code it was written about." >&2
  exit 1
fi

echo "mutation exclusions: every pattern still matches a real mutant"
