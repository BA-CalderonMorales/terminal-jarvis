#!/bin/bash

# Terminal Jarvis Quick CI/CD Test Script
# Runs quality checks only (no publishing)

set -e  # Exit on any error

# Colors for output
CYAN='\033[0;96m'
BLUE='\033[0;94m'
GREEN='\033[0;92m'
RESET='\033[0m'

# Get current branch
CURRENT_BRANCH=$(git branch --show-current)

echo -e "${CYAN}🚀 Terminal Jarvis Quick CI Test${RESET}"
echo -e "${BLUE}Current branch: ${CURRENT_BRANCH}${RESET}"
echo ""

# Step 1: Run Quality Checks
echo -e "${CYAN}📋 Running Quality Checks${RESET}"
echo -e "${BLUE}→ cargo fmt...${RESET}"
cargo fmt --all

echo -e "${BLUE}→ cargo clippy...${RESET}"
cargo clippy --all-targets --all-features -- -D warnings

echo -e "${BLUE}→ cargo test...${RESET}"
cargo test

echo -e "${GREEN}✅ All quality checks passed!${RESET}"
echo ""

# Step 2: Build Release Binary
echo -e "${CYAN}📦 Building Release Binary${RESET}"
cargo build --release
echo -e "${GREEN}✅ Release binary built successfully!${RESET}"
echo ""

# Step 3: Test Binary
echo -e "${CYAN}🧪 Testing Binary${RESET}"
./target/release/terminal-jarvis --help > /dev/null
./target/release/terminal-jarvis list > /dev/null
echo -e "${GREEN}✅ Binary tests passed!${RESET}"
echo ""

# Step 4: Update NPM Package
echo -e "${CYAN}📦 Building NPM Package${RESET}"
cd npm/terminal-jarvis
npm run build > /dev/null 2>&1
cd ../..
echo -e "${GREEN}✅ NPM package built successfully!${RESET}"
echo ""

# Step 5: Test NPM Package
echo -e "${CYAN}🧪 Testing NPM Package${RESET}"
./npm/terminal-jarvis/bin/terminal-jarvis --help > /dev/null
echo -e "${GREEN}✅ NPM package tests passed!${RESET}"
echo ""

echo -e "${GREEN}🎉 All CI checks completed successfully!${RESET}"
echo -e "${BLUE}Ready for: ${CURRENT_BRANCH}${RESET}"
echo ""
echo -e "${CYAN}💡 To run full CI/CD with merge/publish options:${RESET}"
echo -e "${BLUE}   ./scripts/local-cicd.sh${RESET}"
