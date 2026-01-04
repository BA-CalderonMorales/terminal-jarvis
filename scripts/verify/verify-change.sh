#!/usr/bin/env bash
# Verification: Complete change verification
# Purpose: Run all verification steps before committing
# Usage: ./scripts/verify/verify-change.sh [--quick]
#
# This is the main entry point for the verification feedback loop.
# AI agents should run this after making changes to ensure quality.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$PROJECT_ROOT"

QUICK_MODE=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --quick)
            QUICK_MODE=true
            shift
            ;;
        *)
            shift
            ;;
    esac
done

echo "=============================================="
echo "  VERIFICATION FEEDBACK LOOP"
echo "  Terminal Jarvis Change Verification"
echo "=============================================="
echo ""

START_TIME=$(date +%s)
FAILED=0

# Step 1: Build
echo "[STEP 1/4] Build Verification"
echo "------------------------------"
if "$SCRIPT_DIR/verify-build.sh"; then
    echo ""
else
    echo "[ABORT] Build failed - fix compilation errors first"
    exit 1
fi

# Step 2: Quality
echo "[STEP 2/4] Quality Verification"
echo "--------------------------------"
if "$SCRIPT_DIR/verify-quality.sh"; then
    echo ""
else
    echo "[ABORT] Quality check failed - fix linting/formatting issues"
    exit 1
fi

# Step 3: Tests (skip in quick mode)
if [ "$QUICK_MODE" = true ]; then
    echo "[STEP 3/4] Test Verification (SKIPPED - quick mode)"
    echo "----------------------------------------------------"
    echo "[SKIP] Use full mode for complete verification"
    echo ""
else
    echo "[STEP 3/4] Test Verification"
    echo "-----------------------------"
    if "$SCRIPT_DIR/verify-tests.sh" --unit-only; then
        echo ""
    else
        echo "[WARN] Some tests failed - review before committing"
        FAILED=1
    fi
fi

# Step 4: CLI smoke tests
echo "[STEP 4/4] CLI Verification"
echo "----------------------------"
if "$SCRIPT_DIR/verify-cli.sh"; then
    echo ""
else
    echo "[WARN] CLI smoke tests failed - review before committing"
    FAILED=1
fi

END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

echo "=============================================="
if [ $FAILED -eq 0 ]; then
    echo "  VERIFICATION: ALL CHECKS PASSED"
    echo "  Duration: ${DURATION}s"
    echo "=============================================="
    echo ""
    echo "Ready to commit. Suggested workflow:"
    echo "  1. git add -A"
    echo "  2. git commit -m 'type(scope): description'"
    echo "  3. git push"
    echo ""
    exit 0
else
    echo "  VERIFICATION: SOME CHECKS FAILED"
    echo "  Duration: ${DURATION}s"
    echo "=============================================="
    echo ""
    echo "Review the failures above before committing."
    echo ""
    exit 1
fi
