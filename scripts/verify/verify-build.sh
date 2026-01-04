#!/usr/bin/env bash
# Verification: Build check
# Purpose: Ensure code compiles without errors
# Usage: ./scripts/verify/verify-build.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$PROJECT_ROOT"

echo "[VERIFY] Build Check"
echo "===================="

echo "[1/2] Running cargo check..."
if cargo check --all-targets 2>&1; then
    echo "[PASS] Compilation successful"
else
    echo "[FAIL] Compilation failed"
    exit 1
fi

echo "[2/2] Building release binary..."
if cargo build --release 2>&1; then
    echo "[PASS] Release build successful"
else
    echo "[FAIL] Release build failed"
    exit 1
fi

echo ""
echo "[VERIFY] Build Check: PASSED"
