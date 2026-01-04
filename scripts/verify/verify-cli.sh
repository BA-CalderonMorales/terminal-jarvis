#!/usr/bin/env bash
# Verification: CLI smoke tests
# Purpose: Verify CLI runs correctly and shows expected output
# Usage: ./scripts/verify/verify-cli.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$PROJECT_ROOT"

echo "[VERIFY] CLI Smoke Tests"
echo "========================"

# Ensure binary is built
echo "[0/5] Building release binary..."
cargo build --release 2>&1

BINARY="$PROJECT_ROOT/target/release/terminal-jarvis"

if [ ! -f "$BINARY" ]; then
    echo "[FAIL] Binary not found at $BINARY"
    exit 1
fi

FAILED=0

echo "[1/5] Testing --version..."
VERSION_OUTPUT=$("$BINARY" --version 2>&1 || true)
if echo "$VERSION_OUTPUT" | grep -q "terminal-jarvis"; then
    echo "[PASS] --version works"
else
    echo "[FAIL] --version output unexpected: $VERSION_OUTPUT"
    FAILED=1
fi

echo "[2/5] Testing --help..."
HELP_OUTPUT=$("$BINARY" --help 2>&1 || true)
if echo "$HELP_OUTPUT" | grep -qi "usage\|options\|commands"; then
    echo "[PASS] --help works"
else
    echo "[FAIL] --help output unexpected"
    FAILED=1
fi

echo "[3/5] Testing 'list' command..."
LIST_OUTPUT=$("$BINARY" list 2>&1 || true)
if echo "$LIST_OUTPUT" | grep -qi "claude\|gemini\|tools\|available"; then
    echo "[PASS] list command works"
else
    echo "[FAIL] list command output unexpected: $LIST_OUTPUT"
    FAILED=1
fi

echo "[4/5] Testing 'info' command..."
INFO_OUTPUT=$("$BINARY" info claude 2>&1 || true)
if echo "$INFO_OUTPUT" | grep -qi "claude\|anthropic\|tool\|info"; then
    echo "[PASS] info command works"
else
    echo "[WARN] info command output may be unexpected (non-blocking)"
fi

echo "[5/5] Testing invalid command handling..."
INVALID_OUTPUT=$("$BINARY" definitely-not-a-command 2>&1 || true)
# Should show error or help, not crash
if [ -n "$INVALID_OUTPUT" ]; then
    echo "[PASS] Invalid command handled gracefully"
else
    echo "[WARN] Invalid command produced no output (may be acceptable)"
fi

if [ $FAILED -eq 1 ]; then
    echo ""
    echo "[VERIFY] CLI Smoke Tests: FAILED"
    exit 1
fi

echo ""
echo "[VERIFY] CLI Smoke Tests: PASSED"
