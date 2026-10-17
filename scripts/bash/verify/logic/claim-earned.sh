#!/usr/bin/env sh
# Prove the POSIX-shell platform claim earns its verdict, with the real binary.
#
# The stand-in reproduces the shape the catalog shipped before the catalog
# shape gate gained the rule: an executable probe that claims to run anywhere
# (`platforms = []`, which disables the platform guard) yet routes through
# `sh -c`, which a clean Windows PATH does not provide. The transform wraps
# every executing `*/version` row, so the stand-in is regenerated from the
# catalog and cannot drift from the class it stands for.
#
#   --from REF   also build the pre-fix binary from REF and show it accepts
#                the stand-in; without it only the new half runs.
set -eu

root=$(cd "$(dirname -- "$0")/../../../../" && pwd)
from=
while [ $# -gt 0 ]; do
  case "$1" in
    --from) from=${2:?--from needs a ref}; shift 2 ;;
    *) echo "claim-earned: unknown option $1" >&2; exit 2 ;;
  esac
done

fail() { echo "claim-earned: $*" >&2; exit 1; }

work=$(mktemp -d)
cleanup() {
  [ -d "$work/old" ] && git -C "$root" worktree remove --force "$work/old" >/dev/null 2>&1 || true
  rm -rf "$work"
}
trap cleanup EXIT
binary() { [ -x "$1" ] && echo "$1" || echo "$1.exe"; }

echo "claim-earned: building the binary under test"
cargo build --quiet --manifest-path "$root/Cargo.toml"
new=$(binary "$root/target/debug/terminal-jarvis")
[ -x "$new" ] || fail "built binary not found"

echo "claim-earned: materializing the stand-in catalog"
cp -R "$root/harnesses" "$work/harnesses"
python3 - "$work/harnesses" <<'PY'
import pathlib, re, sys
for path in sorted(pathlib.Path(sys.argv[1]).glob("*/version/index.toml")):
    text = path.read_text(encoding="utf-8")
    fields = dict(re.findall(r"^(\w+)\s*=\s*(.+)$", text, re.M))
    if fields.get("support", "").strip('"') not in ("verified", "expected"):
        continue
    if fields.get("executable", "").strip('"') == "sh":
        continue
    probe = " ".join([fields["command"].strip('"'),
                      *re.findall(r'"([^"]*)"', fields.get("args", "[]"))]).strip()
    text = re.sub(r"^command\s*=.*$", 'command = "sh"', text, flags=re.M)
    text = re.sub(r"^args\s*=.*$", f'args = ["-c", "{probe}"]', text, flags=re.M)
    text = re.sub(r"^executable\s*=.*$", 'executable = "sh"', text, flags=re.M)
    path.write_text(text, encoding="utf-8")
PY

echo "claim-earned: new binary vs the stand-in"
TERMINAL_JARVIS_CATALOG="$work/harnesses" "$new" --plain list >"$work/standin" 2>&1 &&
  fail "the new gate accepted the POSIX-shell stand-in"
grep -q "POSIX shell" "$work/standin" || { cat "$work/standin"; fail "refused for the wrong reason"; }
echo "  new: refused $(grep -o 'POSIX shell' "$work/standin" | wc -l | tr -d ' ') POSIX-shell claims"

echo "claim-earned: new binary vs the real catalog"
TERMINAL_JARVIS_CATALOG="$root/harnesses" "$new" --plain list >"$work/real" 2>&1 ||
  { cat "$work/real"; fail "the real catalog was refused"; }
echo "  new: accepted"

if [ -n "$from" ]; then
  echo "claim-earned: pre-fix binary from $from vs the stand-in"
  git -C "$root" worktree add --detach "$work/old" "$from" >/dev/null 2>&1 ||
    fail "cannot create a worktree at $from"
  cargo build --quiet --manifest-path "$work/old/Cargo.toml"
  old=$(binary "$work/old/target/debug/terminal-jarvis")
  [ -x "$old" ] || fail "pre-fix binary not built"
  if TERMINAL_JARVIS_CATALOG="$work/harnesses" "$old" --plain list >"$work/old.out" 2>&1; then
    echo "  old: accepted the stand-in"
  else
    echo "  old: refused it"
  fi
fi

echo "claim-earned: ok"
