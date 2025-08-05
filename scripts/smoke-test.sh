#!/bin/bash

# Terminal Jarvis Comprehensive Test Suite
# Validates core functionality and NPM package integrity to prevent regressions

# Colors for output
CYAN='\033[0;96m'
BLUE='\033[0;94m'
GREEN='\033[0;92m'
YELLOW='\033[0;93m'
RED='\033[0;91m'
RESET='\033[0m'

BINARY="./target/release/terminal-jarvis"
TESTS_PASSED=0
TESTS_FAILED=0

# Test function for consistency
run_test() {
    local test_name="$1"
    local test_command="$2"
    
    echo -e "${BLUE}→ $test_name${RESET}"
    
    # Execute test command and capture result without exiting on failure
    if eval "$test_command" >/dev/null 2>&1; then
        echo -e "${GREEN}  ✅ PASSED${RESET}"
        ((TESTS_PASSED++))
        return 0
    else
        echo -e "${RED}  ❌ FAILED${RESET}"
        ((TESTS_FAILED++))
        return 1
    fi
}

echo -e "${CYAN}🧪 Terminal Jarvis Comprehensive Test Suite${RESET}"
echo -e "${BLUE}Running core functionality and NPM package validation...${RESET}"
echo ""

# Build if needed
if [ ! -f "$BINARY" ]; then
    echo -e "${BLUE}Building release binary...${RESET}"
    cargo build --release
fi

# ===== CORE FUNCTIONALITY TESTS =====
echo -e "${CYAN}� Core Functionality Tests${RESET}"

run_test "CLI help command works" \
    "$BINARY --help > /dev/null 2>&1"

run_test "Tool listing functionality" \
    "$BINARY list > /dev/null 2>&1"

run_test "All 4 tools loaded from configuration" \
    'TOOL_COUNT=$('$BINARY' list 2>/dev/null | grep -E "^  (claude|gemini|qwen|opencode)" | wc -l); [ "$TOOL_COUNT" -eq 4 ]'

run_test "All tools use NPM packages consistently" \
    'NPM_TOOLS=$('$BINARY' list 2>/dev/null | grep -c "Requires: NPM"); [ "$NPM_TOOLS" -eq 4 ]'

run_test "Update command help" \
    "$BINARY update --help > /dev/null 2>&1"

run_test "Install command help" \
    "$BINARY install --help > /dev/null 2>&1"

run_test "Run command help" \
    "$BINARY run --help > /dev/null 2>&1"

run_test "Error handling for nonexistent tool" \
    'timeout 5s '$BINARY' run nonexistent-tool >/dev/null 2>&1; [ $? -ne 0 ]'

run_test "Version consistency (Cargo.toml vs NPM package.json)" \
    'CARGO_VERSION=$(grep "^version = " Cargo.toml | sed "s/version = \"\(.*\)\"/\1/"); NPM_VERSION=$(grep "\"version\":" npm/terminal-jarvis/package.json | sed "s/.*\"version\": \"\(.*\)\".*/\1/"); [ "$CARGO_VERSION" = "$NPM_VERSION" ]'

run_test "Example configuration file has all 4 tools" \
    'CONFIG_TOOLS=$(grep -E "(claude-code|gemini-cli|qwen-code|opencode)" terminal-jarvis.toml.example | wc -l); [ "$CONFIG_TOOLS" -eq 4 ]'

run_test "Example config uses NPM for all installs" \
    'NPM_INSTALL_COMMANDS=$(grep -c "npm install" terminal-jarvis.toml.example); [ "$NPM_INSTALL_COMMANDS" -eq 4 ]'

echo ""

# ===== NPM PACKAGE VALIDATION TESTS =====
echo -e "${CYAN}📦 NPM Package Validation Tests${RESET}"

# Check if NPM is available
if ! command -v npm &> /dev/null; then
    echo -e "${YELLOW}⚠️  NPM not available - skipping NPM package validation${RESET}"
    echo -e "${BLUE}Install Node.js and NPM to run complete validation${RESET}"
else
    echo -e "${BLUE}NPM version: $(npm --version)${RESET}"
    echo ""
    
    # Extract NPM package names from installation configuration
    CLAUDE_PACKAGE=$(grep -A5 'claude",' src/installation_arguments.rs | grep 'args: vec!' | sed 's/.*"\([^"]*\)".*/\1/' | tail -1)
    GEMINI_PACKAGE=$(grep -A5 'gemini",' src/installation_arguments.rs | grep 'args: vec!' | sed 's/.*"\([^"]*\)".*/\1/' | tail -1)
    QWEN_PACKAGE=$(grep -A5 'qwen",' src/installation_arguments.rs | grep 'args: vec!' | sed 's/.*"\([^"]*\)".*/\1/' | tail -1)
    OPENCODE_PACKAGE=$(grep -A5 'opencode",' src/installation_arguments.rs | grep 'args: vec!' | sed 's/.*"\([^"]*\)".*/\1/' | tail -1)
    
    echo -e "${BLUE}Validating packages: $CLAUDE_PACKAGE, $GEMINI_PACKAGE, $QWEN_PACKAGE, $OPENCODE_PACKAGE${RESET}"
    echo ""
    
    run_test "Claude package exists in NPM registry" \
        "npm view $CLAUDE_PACKAGE version > /dev/null 2>&1"
    
    run_test "Gemini package exists in NPM registry" \
        "npm view $GEMINI_PACKAGE version > /dev/null 2>&1"
    
    run_test "Qwen package exists in NPM registry" \
        "npm view $QWEN_PACKAGE version > /dev/null 2>&1"
    
    run_test "OpenCode package exists in NPM registry" \
        "npm view $OPENCODE_PACKAGE version > /dev/null 2>&1"
    
    run_test "Claude package provides 'claude' binary" \
        "npm view $CLAUDE_PACKAGE bin | grep -q 'claude'"
    
    run_test "Gemini package provides 'gemini' binary" \
        "npm view $GEMINI_PACKAGE bin | grep -q 'gemini'"
    
    # Validate configuration consistency across files
    CONFIG_CLAUDE=$(grep -A2 'claude-code' src/config.rs | grep 'install_command' | sed 's/.*npm install -g \([^ "]*\).*/\1/')
    CONFIG_GEMINI=$(grep -A2 'gemini-cli' src/config.rs | grep 'install_command' | sed 's/.*npm install -g \([^ "]*\).*/\1/')
    
    run_test "Claude package consistent between installation_arguments.rs and config.rs" \
        "[ '$CLAUDE_PACKAGE' = '$CONFIG_CLAUDE' ]"
    
    run_test "Gemini package consistent between installation_arguments.rs and config.rs" \
        "[ '$GEMINI_PACKAGE' = '$CONFIG_GEMINI' ]"
    
    # Validate package installation compatibility (dry run)
    run_test "Claude package can be installed (dry run)" \
        "npm install -g $CLAUDE_PACKAGE --dry-run > /dev/null 2>&1"
    
    run_test "Gemini package can be installed (dry run)" \
        "npm install -g $GEMINI_PACKAGE --dry-run > /dev/null 2>&1"
    
    run_test "Qwen package can be installed (dry run)" \
        "npm install -g $QWEN_PACKAGE --dry-run > /dev/null 2>&1"
    
    run_test "OpenCode package can be installed (dry run)" \
        "npm install -g $OPENCODE_PACKAGE --dry-run > /dev/null 2>&1"
    
    # Validate services.rs update logic has correct package names
    SERVICES_CLAUDE_PRIMARY=$(grep -A10 'claude-code.*=>' src/services.rs | grep 'update_npm_package' | head -1 | sed 's/.*update_npm_package("\([^"]*\)").*/\1/')
    SERVICES_GEMINI_PRIMARY=$(grep -A10 'gemini-cli.*=>' src/services.rs | grep 'update_npm_package' | head -1 | sed 's/.*update_npm_package("\([^"]*\)").*/\1/')
    
    run_test "Claude update logic uses correct primary package" \
        "[ '$CLAUDE_PACKAGE' = '$SERVICES_CLAUDE_PRIMARY' ]"
    
    run_test "Gemini update logic uses correct primary package" \
        "[ '$GEMINI_PACKAGE' = '$SERVICES_GEMINI_PRIMARY' ]"
    
    # Validate documentation consistency
    run_test "TESTING.md uses correct Claude package name" \
        "grep -q '$CLAUDE_PACKAGE' docs/TESTING.md"
    
    run_test "TESTING.md uses correct Gemini package name" \
        "grep -q '$GEMINI_PACKAGE' docs/TESTING.md"
fi

echo ""
echo -e "${CYAN}📊 Test Results Summary${RESET}"
echo -e "${GREEN}✅ Tests Passed: $TESTS_PASSED${RESET}"
echo -e "${RED}❌ Tests Failed: $TESTS_FAILED${RESET}"

if [ $TESTS_FAILED -eq 0 ]; then
    echo ""
    echo -e "${GREEN}🎉 All tests passed!${RESET}"
    echo -e "${BLUE}Core functionality works and NPM packages are valid.${RESET}"
    echo -e "${BLUE}The application is ready for release.${RESET}"
    exit 0
else
    echo ""
    echo -e "${RED}💥 Some tests failed!${RESET}"
    echo -e "${YELLOW}Please fix the failing functionality before proceeding with release.${RESET}"
    echo ""
    echo -e "${CYAN}💡 Common fixes for NPM package issues:${RESET}"
    echo -e "${BLUE}• Verify package names exist in NPM registry${RESET}"
    echo -e "${BLUE}• Update installation_arguments.rs with correct package names${RESET}"
    echo -e "${BLUE}• Ensure config.rs, terminal-jarvis.toml.example, and services.rs use same packages${RESET}"
    echo -e "${BLUE}• Update documentation with correct package references${RESET}"
    exit 1
fi
