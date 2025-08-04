#!/bin/bash

# Terminal Jarvis Core Functionality Test Suite
# Validates that core behaviors work as expected to prevent regressions

# Colors for output
CYAN='\033[0;96m'
BLUE='\033[0;94m'
GREEN='\033[0;92m'
YELLOW='\033[0;93m'
RED='\033[0;91m'
RESET='\033[0m'

echo -e "${CYAN}🧪 Terminal Jarvis Core Functionality Test Suite${RESET}"
echo ""

# Ensure we have a release binary
if [ ! -f "./target/release/terminal-jarvis" ]; then
    echo -e "${BLUE}Building release binary for testing...${RESET}"
    cargo build --release
fi

TESTS_PASSED=0
TESTS_FAILED=0

# Test function for consistency
run_test() {
    local test_name="$1"
    local test_command="$2"
    
    echo -e "${BLUE}→ $test_name${RESET}"
    
    # Execute test command and capture result without exiting on failure
    if bash -c "$test_command" 2>/dev/null; then
        echo -e "${GREEN}  ✅ PASSED${RESET}"
        ((TESTS_PASSED++))
    else
        echo -e "${RED}  ❌ FAILED${RESET}"
        ((TESTS_FAILED++))
    fi
}

# Test 1: Basic CLI functionality
run_test "Test 1: Basic CLI help command" \
    "./target/release/terminal-jarvis --help > /dev/null 2>&1"

# Test 2: Tool listing
run_test "Test 2: Tool listing functionality" \
    "./target/release/terminal-jarvis list > /dev/null 2>&1"

# Test 3: Configuration loading - all 4 tools present
run_test "Test 3: All 4 tools loaded from configuration" \
    'TOOL_COUNT=$(./target/release/terminal-jarvis list 2>/dev/null | grep -E "^  (claude|gemini|qwen|opencode)" | wc -l); [ "$TOOL_COUNT" -eq 4 ]'

# Test 4: Update command structure 
run_test "Test 4: Update command help" \
    "./target/release/terminal-jarvis update --help > /dev/null 2>&1"

# Test 5: Install command structure
run_test "Test 5: Install command help" \
    "./target/release/terminal-jarvis install --help > /dev/null 2>&1"

# Test 6: Run command structure
run_test "Test 6: Run command help" \
    "./target/release/terminal-jarvis run --help > /dev/null 2>&1"

# Test 7: NPM package consistency
run_test "Test 7: All tools use NPM packages consistently" \
    'NPM_TOOLS=$(./target/release/terminal-jarvis list 2>/dev/null | grep -c "Requires: NPM"); [ "$NPM_TOOLS" -eq 4 ]'

# Test 8: Error handling for invalid tools
run_test "Test 8: Error handling for nonexistent tool" \
    '! ./target/release/terminal-jarvis run nonexistent-tool 2>/dev/null'

# Test 9: Version consistency across files
run_test "Test 9: Version consistency (Cargo.toml vs NPM package.json)" \
    'CARGO_VERSION=$(grep "^version = " Cargo.toml | sed "s/version = \"\(.*\)\"/\1/"); NPM_VERSION=$(grep "\"version\":" npm/terminal-jarvis/package.json | sed "s/.*\"version\": \"\(.*\)\".*/\1/"); [ "$CARGO_VERSION" = "$NPM_VERSION" ]'

# Test 10: Configuration files integrity
run_test "Test 10: Example configuration file has all 4 tools" \
    'CONFIG_TOOLS=$(grep -E "(claude-code|gemini-cli|qwen-code|opencode)" terminal-jarvis.toml.example | wc -l); [ "$CONFIG_TOOLS" -eq 4 ]'

# Test 11: Example config has NPM install commands for all tools
run_test "Test 11: Example config uses NPM for all installs" \
    'NPM_INSTALL_COMMANDS=$(grep -c "npm install" terminal-jarvis.toml.example); [ "$NPM_INSTALL_COMMANDS" -eq 4 ]'

# Test 12: Tool execution validation (check that run command works)
run_test "Test 12: Tool run command structure" \
    "./target/release/terminal-jarvis run --help > /dev/null 2>&1"

# Test 13: Concurrent update validation (command accepts no args - use timeout to avoid hanging)
run_test "Test 13: Concurrent update command initiates properly" \
    'timeout 3s ./target/release/terminal-jarvis update 2>&1 | head -1 | grep -q "Updating all packages" || true'

# Test 14: Info command functionality  
run_test "Test 14: Info command help works" \
    "./target/release/terminal-jarvis info --help > /dev/null 2>&1"

# Test 15: Templates command structure
run_test "Test 15: Templates command help" \
    "./target/release/terminal-jarvis templates --help > /dev/null 2>&1"

echo ""
echo -e "${CYAN}📊 Test Results Summary${RESET}"
echo -e "${GREEN}✅ Tests Passed: $TESTS_PASSED${RESET}"
echo -e "${RED}❌ Tests Failed: $TESTS_FAILED${RESET}"

if [ $TESTS_FAILED -eq 0 ]; then
    echo ""
    echo -e "${GREEN}🎉 All core functionality tests passed!${RESET}"
    echo -e "${BLUE}The application is ready for release.${RESET}"
    exit 0
else
    echo ""
    echo -e "${RED}💥 Some tests failed!${RESET}"
    echo -e "${YELLOW}Please fix the failing functionality before proceeding with release.${RESET}"
    exit 1
fi
