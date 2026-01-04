#!/usr/bin/env bash
# Verification: Code quality check
# Purpose: Ensure code meets quality standards (clippy, formatting)
# Usage: ./scripts/verify/verify-quality.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$PROJECT_ROOT"

echo "[VERIFY] Quality Check"
echo "======================"

echo "[1/3] Running cargo fmt check..."
if cargo fmt --all -- --check 2>&1; then
    echo "[PASS] Formatting is correct"
else
    echo "[FAIL] Formatting issues found. Run: cargo fmt --all"
    exit 1
fi

echo "[2/3] Running cargo clippy..."
if cargo clippy -- -D warnings 2>&1; then
    echo "[PASS] No clippy warnings"
else
    echo "[FAIL] Clippy found issues"
    exit 1
fi

echo "[3/3] Checking TypeScript (NPM package)..."
if [ -d "$PROJECT_ROOT/npm/terminal-jarvis" ]; then
    cd "$PROJECT_ROOT/npm/terminal-jarvis"
    if [ -f "node_modules/.bin/biome" ]; then
        if npm run lint 2>&1; then
            echo "[PASS] TypeScript linting passed"
        else
            echo "[WARN] TypeScript linting issues (non-blocking)"
        fi
    else
        echo "[SKIP] Biome not installed, skipping TypeScript lint"
    fi
fi

cd "$PROJECT_ROOT"

echo ""
echo "[VERIFY] Quality Check: PASSED"
