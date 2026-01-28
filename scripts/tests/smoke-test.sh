#!/usr/bin/env bash
# Smoke Test Wrapper
# Purpose: Run comprehensive smoke tests for CI/CD pipeline
# Usage: ./scripts/tests/smoke-test.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$PROJECT_ROOT"

echo "[SMOKE] Running comprehensive smoke tests..."
echo "==========================================="

# Run CLI smoke tests
./scripts/verify/verify-cli.sh

echo ""
echo "[SMOKE] All smoke tests passed!"
