#!/usr/bin/env bash
# Mutation smoke test for dependency-alert guardrails.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
CHECKER="$SCRIPT_DIR/verify-dependency-alerts.py"
TMP_ROOT="$(mktemp -d)"

cleanup() {
    rm -rf "$TMP_ROOT"
}
trap cleanup EXIT

copy_inputs() {
    local dest="$1"
    mkdir -p "$dest/adk" "$dest/e2e"
    cp "$PROJECT_ROOT/Cargo.lock" "$dest/Cargo.lock"
    cp "$PROJECT_ROOT/adk/go.mod" "$dest/adk/go.mod"
    cp "$PROJECT_ROOT/adk/go.sum" "$dest/adk/go.sum"
    cp "$PROJECT_ROOT/e2e/package-lock.json" "$dest/e2e/package-lock.json"
}

expect_failure() {
    local name="$1"
    local fixture="$2"
    local output="$TMP_ROOT/${name//[^[:alnum:]_.-]/-}.out"

    if python3 "$CHECKER" --root "$fixture" >"$output" 2>&1; then
        echo "[FAIL] Mutation survived: $name"
        cat "$output"
        exit 1
    fi

    echo "[PASS] Mutation killed: $name"
}

python3 "$CHECKER" --root "$PROJECT_ROOT"

GO_FIXTURE="$TMP_ROOT/go"
copy_inputs "$GO_FIXTURE"
python3 - "$GO_FIXTURE/adk/go.mod" <<'PY'
import sys
from pathlib import Path

path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")
old = "google.golang.org/grpc v1.81.1"
new = "google.golang.org/grpc v1.78.0"
if old not in text:
    raise SystemExit(f"missing expected module line: {old}")
path.write_text(text.replace(old, new, 1), encoding="utf-8")
PY
expect_failure "grpc downgrade" "$GO_FIXTURE"

CARGO_FIXTURE="$TMP_ROOT/cargo"
copy_inputs "$CARGO_FIXTURE"
python3 - "$CARGO_FIXTURE/Cargo.lock" <<'PY'
import re
import sys
from pathlib import Path

path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")
mutated, count = re.subn(
    r'(\[\[package\]\]\nname = "rpassword"\nversion = ")7\.5\.2(")',
    r'\g<1>7.4.0\2',
    text,
    count=1,
)
if count != 1:
    raise SystemExit("missing expected rpassword package entry")
path.write_text(mutated, encoding="utf-8")
PY
expect_failure "rpassword downgrade" "$CARGO_FIXTURE"

NPM_FIXTURE="$TMP_ROOT/npm"
copy_inputs "$NPM_FIXTURE"
python3 - "$NPM_FIXTURE/e2e/package-lock.json" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
data = json.loads(path.read_text(encoding="utf-8"))
data["packages"]["node_modules/rollup"] = {"version": "4.60.3"}
path.write_text(json.dumps(data, indent=4) + "\n", encoding="utf-8")
PY
expect_failure "rollup reintroduced" "$NPM_FIXTURE"

echo "[PASS] Dependency-alert mutation tests passed"
