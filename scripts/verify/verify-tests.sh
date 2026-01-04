#!/usr/bin/env bash
# Verification: Test suite
# Purpose: Run unit tests and E2E tests
# Usage: ./scripts/verify/verify-tests.sh [--unit-only|--e2e-only]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$PROJECT_ROOT"

RUN_UNIT=true
RUN_E2E=true

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --unit-only)
            RUN_E2E=false
            shift
            ;;
        --e2e-only)
            RUN_UNIT=false
            shift
            ;;
        *)
            shift
            ;;
    esac
done

echo "[VERIFY] Test Suite"
echo "==================="

FAILED=0

if [ "$RUN_UNIT" = true ]; then
    echo "[1/2] Running Rust unit tests..."
    if cargo test 2>&1; then
        echo "[PASS] Unit tests passed"
    else
        echo "[FAIL] Unit tests failed"
        FAILED=1
    fi
else
    echo "[SKIP] Unit tests (--e2e-only specified)"
fi

if [ "$RUN_E2E" = true ]; then
    echo "[2/2] Running E2E tests..."
    if [ -d "$PROJECT_ROOT/e2e" ]; then
        cd "$PROJECT_ROOT/e2e"
        
        # Check if node_modules exists
        if [ ! -d "node_modules" ]; then
            echo "[INFO] Installing E2E dependencies..."
            npm install 2>&1
        fi
        
        # Ensure binary is built
        cd "$PROJECT_ROOT"
        cargo build --release 2>&1
        
        cd "$PROJECT_ROOT/e2e"
        if npm test 2>&1; then
            echo "[PASS] E2E tests passed"
        else
            echo "[FAIL] E2E tests failed"
            FAILED=1
        fi
    else
        echo "[SKIP] E2E directory not found"
    fi
else
    echo "[SKIP] E2E tests (--unit-only specified)"
fi

cd "$PROJECT_ROOT"

if [ $FAILED -eq 1 ]; then
    echo ""
    echo "[VERIFY] Test Suite: FAILED"
    exit 1
fi

echo ""
echo "[VERIFY] Test Suite: PASSED"
