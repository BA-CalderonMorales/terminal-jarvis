#!/bin/bash

# Terminal Jarvis Local CI Script
# Runs all quality checks, tests, and builds without committing/tagging/pushing
# Use this to validate changes before deployment

set -e  # Exit on any error

# Colors for output
CYAN='\033[0;96m'
BLUE='\033[0;94m'
GREEN='\033[0;92m'
YELLOW='\033[0;93m'
RED='\033[0;91m'
RESET='\033[0m'

# Get current branch
CURRENT_BRANCH=$(git branch --show-current)

echo -e "${CYAN}🔍 Terminal Jarvis Local CI Pipeline${RESET}"
echo -e "${BLUE}Current branch: ${CURRENT_BRANCH}${RESET}"
echo -e "${YELLOW}Running validation and testing WITHOUT deployment${RESET}"
echo ""

# Step 0: CHANGELOG.md Check
echo -e "${CYAN}📝 Step 0: CHANGELOG.md Verification${RESET}"

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
echo -e "${BLUE}Current version in Cargo.toml: ${CURRENT_VERSION}${RESET}"

# Check if CHANGELOG.md has entry for current version
if ! grep -q "\[${CURRENT_VERSION}\]" CHANGELOG.md; then
    echo -e "${YELLOW}⚠️  CHANGELOG.md does not contain an entry for version ${CURRENT_VERSION}${RESET}"
    echo -e "${BLUE}Consider updating CHANGELOG.md before deployment${RESET}"
else
    echo -e "${GREEN}✅ CHANGELOG.md contains entry for version ${CURRENT_VERSION}${RESET}"
fi

echo ""

# Step 1: Run Quality Checks
echo -e "${CYAN}📋 Step 1: Running Quality Checks${RESET}"
echo -e "${BLUE}→ Running cargo fmt...${RESET}"
cargo fmt --all

echo -e "${BLUE}→ Running cargo clippy...${RESET}"
cargo clippy --all-targets --all-features -- -D warnings

echo -e "${BLUE}→ Running tests...${RESET}"
cargo test

echo -e "${GREEN}✅ All quality checks passed!${RESET}"
echo ""

# Step 2: Comprehensive Test Suite (Core Functionality + NPM Package Validation)
echo -e "${CYAN}🧪 Step 2: Comprehensive Test Suite${RESET}"
echo -e "${BLUE}Running core functionality and NPM package validation...${RESET}"
echo -e "${BLUE}This validates:${RESET}"
echo -e "${BLUE}  • Core CLI functionality and commands${RESET}"
echo -e "${BLUE}  • All 6 AI tools are properly configured${RESET}"
echo -e "${BLUE}  • NPM packages exist and are installable${RESET}"
echo -e "${BLUE}  • Configuration consistency across all files${RESET}"
echo -e "${BLUE}  • Binary name mappings are correct${RESET}"
echo ""

# Run our comprehensive smoke test which includes NPM package validation
if ! ./scripts/smoke-test.sh; then
    echo -e "${RED}❌ Comprehensive tests failed!${RESET}"
    echo -e "${YELLOW}This includes core functionality and NPM package validation.${RESET}"
    echo -e "${YELLOW}Please fix the issues before deploying.${RESET}"
    exit 1
fi

echo -e "${GREEN}🎉 All comprehensive tests passed!${RESET}"
echo -e "${BLUE}Core functionality works and all NPM packages are valid and installable.${RESET}"
echo ""

# Step 3: Build Release Binary
echo -e "${CYAN}📦 Step 3: Building Release Binary${RESET}"
cargo build --release
echo -e "${GREEN}✅ Release binary built successfully!${RESET}"
echo ""

# Step 4: Build NPM Package
echo -e "${CYAN}📦 Step 4: Building NPM Package${RESET}"
cd npm/terminal-jarvis
npm run build
cd ../..
echo -e "${GREEN}✅ NPM package built successfully!${RESET}"
echo ""

# Step 5: Validation Summary
echo -e "${CYAN}📊 Step 5: Validation Summary${RESET}"

# Check version consistency across files
echo -e "${BLUE}→ Checking version consistency...${RESET}"

NPM_VERSION=$(grep '"version":' npm/terminal-jarvis/package.json | sed 's/.*"version": "\(.*\)".*/\1/')
TS_VERSION=$(grep "console.log.*Terminal Jarvis v" npm/terminal-jarvis/src/index.ts | sed 's/.*Terminal Jarvis v\([0-9.]*\).*/\1/')

echo -e "${BLUE}  Cargo.toml: ${CURRENT_VERSION}${RESET}"
echo -e "${BLUE}  package.json: ${NPM_VERSION}${RESET}"
echo -e "${BLUE}  index.ts: ${TS_VERSION}${RESET}"

if [ "$CURRENT_VERSION" = "$NPM_VERSION" ] && [ "$CURRENT_VERSION" = "$TS_VERSION" ]; then
    echo -e "${GREEN}✅ All versions are synchronized${RESET}"
else
    echo -e "${RED}❌ Version mismatch detected!${RESET}"
    echo -e "${YELLOW}All versions must be synchronized before deployment${RESET}"
    exit 1
fi

echo ""

# Summary
echo -e "${GREEN}🎉 Local CI validation completed successfully!${RESET}"
echo -e "${BLUE}All checks passed for branch: ${CURRENT_BRANCH}${RESET}"
echo -e "${BLUE}Version: ${CURRENT_VERSION}${RESET}"
echo -e "${BLUE}Ready for deployment with local-cd.sh${RESET}"
echo ""
echo -e "${CYAN}📋 What was validated:${RESET}"
echo -e "${YELLOW}  ✓ Code formatting and linting${RESET}"
echo -e "${YELLOW}  ✓ All tests passing (including codex functionality)${RESET}"
echo -e "${YELLOW}  ✓ Core functionality working${RESET}"
echo -e "${YELLOW}  ✓ NPM package validation (all 6 tools)${RESET}"
echo -e "${YELLOW}  ✓ Release binary building${RESET}"
echo -e "${YELLOW}  ✓ NPM package building${RESET}"
echo -e "${YELLOW}  ✓ Version consistency${RESET}"
echo ""
echo -e "${BLUE}💡 Next step: Run ./scripts/local-cd.sh to deploy${RESET}"