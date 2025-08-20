#!/bin/bash

# Terminal Jarvis Local CI Script
# Runs all quality checks, tests, and builds without committing/tagging/pushing
# Use this to validate changes before deployment

set -e  # Exit on any error

# Source logger
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../logger/logger.sh"

# Get current branch
CURRENT_BRANCH=$(git branch --show-current)

log_header "Terminal Jarvis Local CI Pipeline"
log_info_if_enabled "Current branch: ${CURRENT_BRANCH}"
log_warn_if_enabled "Running validation and testing WITHOUT deployment"

# Step 0: CHANGELOG.md Check
log_info_if_enabled "Step 0: CHANGELOG.md Verification"

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
log_info_if_enabled "Current version in Cargo.toml: ${CURRENT_VERSION}"

# Check if CHANGELOG.md has been updated for current version
if ! grep -q "## \[${CURRENT_VERSION}\]" CHANGELOG.md 2>/dev/null; then
    log_warn_if_enabled "CHANGELOG.md does not contain an entry for version ${CURRENT_VERSION}"
    log_warn_if_enabled "Please update CHANGELOG.md before deployment"
else
    log_success_if_enabled "CHANGELOG.md contains entry for version ${CURRENT_VERSION}"
fi

log_separator

# Step 1: Quality Checks
log_info_if_enabled "Step 1: Running Quality Checks"
echo -e "${BLUE}→ Running cargo fmt...${RESET}"
cargo fmt --all

echo -e "${BLUE}→ Running cargo clippy...${RESET}"
cargo clippy --all-targets --all-features -- -D warnings

echo -e "${BLUE}→ Running tests...${RESET}"
cargo test

# Run Clippy (strict mode - warnings as errors)
log_progress "Running Clippy checks"
if cargo clippy --all-targets --all-features -- -D warnings; then
    log_progress_done
else
    log_progress_failed
    exit 1
fi

# Format check
log_progress "Checking code formatting"
cargo fmt --all --check
log_progress_done

log_success_if_enabled "All quality checks passed!"

log_separator

# Step 2: Comprehensive Test Suite (Core Functionality + NPM Package Validation)
log_info_if_enabled "Step 2: Comprehensive Test Suite"
log_info_if_enabled "Running core functionality and NPM package validation..."
log_info_if_enabled "This validates:"
log_info_if_enabled "  • Core CLI functionality and commands"
log_info_if_enabled "  • All 7 AI tools are properly configured"
log_info_if_enabled "  • NPM packages exist and are installable"
log_info_if_enabled "  • Configuration consistency across all files"
log_info_if_enabled "  • Binary name mappings are correct"

# Run our comprehensive smoke test which includes NPM package validation
log_progress "Running comprehensive smoke tests"
if ./scripts/tests/smoke-test.sh; then
    log_progress_done
    log_success_if_enabled "All comprehensive tests passed!"
    log_info_if_enabled "Core functionality works and all NPM packages are valid and installable."
else
    log_progress_failed
    log_error_if_enabled "Comprehensive tests failed!"
    log_info_if_enabled "This includes core functionality and NPM package validation."
    log_warn_if_enabled "Please fix the issues before deploying."
    exit 1
fi

log_separator

# Step 3: Build Release Binary
log_info_if_enabled "Step 3: Building Release Binary"
log_progress "Building release binary"

# Check if multi-platform build should be tested
if [ "${MULTIPLATFORM_BUILD:-false}" = "true" ]; then
    log_info_if_enabled "Testing multi-platform build capabilities..."
    if ./scripts/utils/build-multiplatform.sh --current-only; then
        log_success_if_enabled "Multi-platform build system working"
    else
        log_warn_if_enabled "Multi-platform build failed, falling back to standard build"
        cargo build --release
    fi
else
    cargo build --release
fi

log_progress_done
log_success_if_enabled "Release binary built successfully!"

log_separator

# Step 4: Build NPM Package
log_info_if_enabled "Step 4: Building NPM Package"
log_progress "Building NPM package"
cd npm/terminal-jarvis && npm run build && cd ../..
log_progress_done
log_success_if_enabled "NPM package built successfully!"

log_separator

# Step 5: Validation Summary
log_info_if_enabled "Step 5: Validation Summary"

# Check version consistency across files
log_progress "Checking version consistency"

NPM_VERSION=$(grep '"version":' npm/terminal-jarvis/package.json | sed 's/.*"version": "\(.*\)".*/\1/')
TS_VERSION=$(grep "console.log.*Terminal Jarvis v" npm/terminal-jarvis/src/index.ts | sed 's/.*Terminal Jarvis v\([0-9.]*\).*/\1/')

log_info_if_enabled "  Cargo.toml: ${CURRENT_VERSION}"
log_info_if_enabled "  package.json: ${NPM_VERSION}"
log_info_if_enabled "  index.ts: ${TS_VERSION}"

if [ "$CURRENT_VERSION" = "$NPM_VERSION" ] && [ "$CURRENT_VERSION" = "$TS_VERSION" ]; then
    log_progress_done
    log_success_if_enabled "All versions are synchronized"
else
    log_progress_failed
    log_error_if_enabled "Version mismatch detected!"
    log_warn_if_enabled "All versions must be synchronized before deployment"
    exit 1
fi

log_separator

# Summary
log_success_if_enabled "Local CI validation completed successfully!"
log_info_if_enabled "All checks passed for branch: ${CURRENT_BRANCH}"
log_info_if_enabled "Version: ${CURRENT_VERSION}"
log_info_if_enabled "Ready for deployment with local-cd.sh"

log_info_if_enabled "What was validated:"
log_info_if_enabled "  ✓ Code formatting and linting"
log_info_if_enabled "  ✓ All tests passing (including core functionality)"
log_info_if_enabled "  ✓ Core functionality working"
log_info_if_enabled "  ✓ NPM package validation (all 7 tools)"
log_info_if_enabled "  ✓ Release binary building"
log_info_if_enabled "  ✓ NPM package building"
log_info_if_enabled "  ✓ Version consistency"

log_info_if_enabled "Next step: Run ./scripts/cicd/local-cd.sh to deploy"