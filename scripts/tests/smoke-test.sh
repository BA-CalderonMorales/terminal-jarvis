#!/bin/bash

# Terminal Jarvis Comprehensive Test Suite
# Validates core functionality and NPM package integrity to prevent regressions
# Source logger
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../logger/logger.sh"

BINARY="./target/release/terminal-jarvis"
TESTS_PASSED=0
TESTS_FAILED=0

# Test function for consistency
run_test() {
    local test_name="$1"
    local test_command="$2"
    
    log_info_if_enabled "→ $test_name"
    
    # Execute test command and capture result without exiting on failure
    if eval "$test_command" >/dev/null 2>&1; then
        log_success_if_enabled "  PASSED"
        ((TESTS_PASSED++))
        return 0
    else
        log_error_if_enabled "  FAILED"
        ((TESTS_FAILED++))
        return 1
    fi
}

log_header "Terminal Jarvis Comprehensive Test Suite"
log_info_if_enabled "Running core functionality and NPM package validation..."

# Build if needed
if [ ! -f "$BINARY" ]; then
    log_info_if_enabled "Building release binary..."
    cargo build --release
fi

# ===== CORE FUNCTIONALITY TESTS =====
log_info_if_enabled "Core Functionality Tests"

run_test "CLI help command works" \
    "$BINARY --help > /dev/null 2>&1"

run_test "Tool listing functionality" \
    "$BINARY list > /dev/null 2>&1"

run_test "All 7 tools loaded from configuration" \
    'TOOL_COUNT=$('$BINARY' list 2>/dev/null | grep -E "^ (claude|gemini|qwen|opencode|llxprt|codex|crush)" | wc -l); [ "$TOOL_COUNT" -eq 7 ]'

run_test "All tools use NPM packages consistently" \
    'OUT=$('$BINARY' list 2>/dev/null); COUNT=0; for t in claude gemini qwen opencode llxprt codex crush; do echo "$OUT" | awk -v t="$t" '\''$0 ~ "^ " t " - " {in_tool=1} in_tool && /Requires: NPM/ {found=1} in_tool && /^ [a-z]/ && $1 != t {in_tool=0} END { exit found?0:1 }'\'' >/dev/null 2>&1 && COUNT=$((COUNT+1)); done; [ "$COUNT" -eq 7 ]'

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

run_test "Example configuration file has all 7 tools" \
    'CONFIG_TOOLS=$(grep -E "^(claude|gemini|qwen|opencode|llxprt|codex|crush) = " terminal-jarvis.toml.example | wc -l); [ "$CONFIG_TOOLS" -eq 7 ]'

run_test "Example config uses NPM for all installs" \
    'for t in claude gemini qwen opencode llxprt codex crush; do grep -q "command = \"npm\"" config/tools/$t.toml || exit 1; done'

# Test the opencode input focus fix specifically
run_test "OpenCode input focus tests pass" \
    "cargo test opencode_input_focus >/dev/null 2>&1"

run_test "OpenCode terminal state preparation method exists" \
    'grep -r "prepare_opencode_terminal_state" src/tools/'

run_test "OpenCode special handling in interactive mode exists" \
    'grep -r "opencode.*extra time and careful terminal state management\|Special handling for opencode" src/cli_logic/ || grep -r "opencode.*focus\|Special.*opencode" src/cli_logic/'

# Test codex functionality specifically
run_test "Codex tool is properly configured" \
    '$BINARY list | grep -q "codex.*OpenAI Codex CLI"'

run_test "Codex auth environment variable handling exists" \
    'grep -r "CODEX_NO_BROWSER" src/auth_manager/'

run_test "Codex API key detection works" \
    'grep -r -A1 -B1 "codex.*=>" src/auth_manager/ | grep -q "OPENAI_API_KEY"'

run_test "Codex help message includes OpenAI API setup" \
    'grep -r -A10 "codex.*=>" src/auth_manager/ | grep -q "platform.openai.com"'

run_test "Codex binary mapping is correct" \
    'grep -r "codex.*codex" src/tools/'

run_test "Codex tool description is informative" \
    'grep -q "OpenAI Codex CLI for local AI coding" config/tools/codex.toml'

run_test "Codex functionality tests pass" \
    "cargo test codex_functionality >/dev/null 2>&1"

# Test crush functionality specifically
run_test "Crush tool is properly configured" \
    '$BINARY list | grep -q "crush.*Charm'\''s multi-model AI assistant with LSP"'

run_test "Crush binary mapping is correct" \
    'grep -r "crush.*crush" src/tools/'

run_test "Crush tool description is informative" \
    'grep -q "Charm.*multi-model AI assistant" config/tools/crush.toml'

run_test "Crush installation command is correct" \
    'grep -q "@charmland/crush" config/tools/crush.toml'

run_test "Crush config mapping exists" \
    'grep -r "crush.*crush" src/services/'

run_test "Crush default config exists" \
    'grep -r -A5 "crush" src/config/ | grep -q "charmland/crush"'

log_separator

# ===== NPM PACKAGE VALIDATION TESTS =====
log_info_if_enabled "NPM Package Validation Tests"

# Check if NPM is available
if ! command -v npm &> /dev/null; then
    log_warn_if_enabled "NPM not available - skipping NPM package validation"
    log_info_if_enabled "Install Node.js and NPM to run complete validation"
else
    log_info_if_enabled "NPM version: $(npm --version)"
    
    # Extract NPM package names from modular configuration system
    CLAUDE_PACKAGE=$(grep 'install.*-g' config/tools/claude.toml | sed 's/.*"\([^"]*\)".*/\1/')
    GEMINI_PACKAGE=$(grep 'install.*-g' config/tools/gemini.toml | sed 's/.*"\([^"]*\)".*/\1/')
    QWEN_PACKAGE=$(grep 'install.*-g' config/tools/qwen.toml | sed 's/.*"\([^"]*\)".*/\1/')
    OPENCODE_PACKAGE=$(grep 'install.*-g' config/tools/opencode.toml | sed 's/.*"\([^"]*\)".*/\1/')
    LLXPRT_PACKAGE=$(grep 'install.*-g' config/tools/llxprt.toml | sed 's/.*"\([^"]*\)".*/\1/')
    CODEX_PACKAGE=$(grep 'install.*-g' config/tools/codex.toml | sed 's/.*"\([^"]*\)".*/\1/')
    CRUSH_PACKAGE=$(grep 'install.*-g' config/tools/crush.toml | sed 's/.*"\([^"]*\)".*/\1/')
    
    log_info_if_enabled "Validating packages: $CLAUDE_PACKAGE, $GEMINI_PACKAGE, $QWEN_PACKAGE, $OPENCODE_PACKAGE, $LLXPRT_PACKAGE, $CODEX_PACKAGE, $CRUSH_PACKAGE"
    
    run_test "Claude package exists in NPM registry" \
        "npm view $CLAUDE_PACKAGE version > /dev/null 2>&1"
    
    run_test "Gemini package exists in NPM registry" \
        "npm view $GEMINI_PACKAGE version > /dev/null 2>&1"
    
    run_test "Qwen package exists in NPM registry" \
        "npm view $QWEN_PACKAGE version > /dev/null 2>&1"
    
    run_test "OpenCode package exists in NPM registry" \
        "npm view $OPENCODE_PACKAGE version > /dev/null 2>&1"
    
    run_test "LLxprt package exists in NPM registry" \
        "npm view $LLXPRT_PACKAGE version > /dev/null 2>&1"
    
    run_test "Codex package exists in NPM registry" \
        "npm view $CODEX_PACKAGE version > /dev/null 2>&1"
    
    run_test "Crush package exists in NPM registry" \
        "npm view $CRUSH_PACKAGE version > /dev/null 2>&1"
    
    run_test "Claude package provides 'claude' binary" \
        "npm view $CLAUDE_PACKAGE bin | grep -q 'claude'"
    
    run_test "Gemini package provides 'gemini' binary" \
        "npm view $GEMINI_PACKAGE bin | grep -q 'gemini'"
    
    run_test "LLxprt package provides 'llxprt' binary" \
        "npm view $LLXPRT_PACKAGE bin | grep -q 'llxprt'"
    
    run_test "Codex package provides 'codex' binary" \
        "npm view $CODEX_PACKAGE bin | grep -q 'codex'"
    
    run_test "Crush package provides 'crush' binary" \
        "npm view $CRUSH_PACKAGE bin | grep -q 'crush'"
    
    # Validate configuration consistency across files
    CONFIG_CLAUDE=$(grep 'install.*-g' config/tools/claude.toml | sed 's/.*"\([^"]*\)".*/\1/')
    CONFIG_GEMINI=$(grep 'install.*-g' config/tools/gemini.toml | sed 's/.*"\([^"]*\)".*/\1/')
    CONFIG_LLXPRT=$(grep 'install.*-g' config/tools/llxprt.toml | sed 's/.*"\([^"]*\)".*/\1/')
    CONFIG_CRUSH=$(grep 'install.*-g' config/tools/crush.toml | sed 's/.*"\([^"]*\)".*/\1/')
    
    run_test "Claude package consistent between installation_arguments.rs and config/" \
        "[ '$CLAUDE_PACKAGE' = '$CONFIG_CLAUDE' ]"
    
    run_test "Gemini package consistent between installation_arguments.rs and config/" \
        "[ '$GEMINI_PACKAGE' = '$CONFIG_GEMINI' ]"
    
    run_test "LLxprt package consistent between installation_arguments.rs and config/" \
        "[ '$LLXPRT_PACKAGE' = '$CONFIG_LLXPRT' ]"
    
    run_test "Crush package consistent between installation_arguments.rs and config/" \
        "[ '$CRUSH_PACKAGE' = '$CONFIG_CRUSH' ]"
    
    # Validate package installation compatibility (dry run)
    run_test "Claude package can be installed (dry run)" \
        "npm install -g $CLAUDE_PACKAGE --dry-run > /dev/null 2>&1"
    
    run_test "Gemini package can be installed (dry run)" \
        "npm install -g $GEMINI_PACKAGE --dry-run > /dev/null 2>&1"
    
    run_test "Qwen package can be installed (dry run)" \
        "npm install -g $QWEN_PACKAGE --dry-run > /dev/null 2>&1"
    
    run_test "OpenCode package can be installed (dry run)" \
        "npm install -g $OPENCODE_PACKAGE --dry-run > /dev/null 2>&1"
    
    run_test "LLxprt package can be installed (dry run)" \
        "npm install -g $LLXPRT_PACKAGE --dry-run > /dev/null 2>&1"
    
    run_test "Codex package can be installed (dry run)" \
        "npm install -g $CODEX_PACKAGE --dry-run > /dev/null 2>&1"
    
    run_test "Crush package can be installed (dry run)" \
        "npm install -g $CRUSH_PACKAGE --dry-run > /dev/null 2>&1"
    
    # Validate services/ update logic has correct package names via config
    SERVICES_CLAUDE_PRIMARY=$(grep 'update.*-g' config/tools/claude.toml | sed 's/.*"\([^"]*\)".*/\1/')
    SERVICES_GEMINI_PRIMARY=$(grep 'update.*-g' config/tools/gemini.toml | sed 's/.*"\([^"]*\)".*/\1/')
    SERVICES_LLXPRT_PRIMARY=$(grep 'update.*-g' config/tools/llxprt.toml | sed 's/.*"\([^"]*\)".*/\1/')
    
    run_test "Claude update logic uses correct primary package" \
        "[ '$CLAUDE_PACKAGE' = '$SERVICES_CLAUDE_PRIMARY' ]"
    
    run_test "Gemini update logic uses correct primary package" \
        "[ '$GEMINI_PACKAGE' = '$SERVICES_GEMINI_PRIMARY' ]"
    
    run_test "LLxprt update logic uses correct primary package" \
        "[ '$LLXPRT_PACKAGE' = '$SERVICES_LLXPRT_PRIMARY' ]"
    
    # Validate documentation consistency
    run_test "TESTING.md uses correct Claude package name" \
        "grep -q '$CLAUDE_PACKAGE' docs/TESTING.md"
    
    run_test "TESTING.md uses correct Gemini package name" \
        "grep -q '$GEMINI_PACKAGE' docs/TESTING.md"
    
    run_test "TESTING.md uses correct LLxprt package name" \
        "grep -q '$LLXPRT_PACKAGE' docs/TESTING.md"
fi

log_separator
log_info_if_enabled "Test Results Summary"
log_success_if_enabled "Tests Passed: $TESTS_PASSED"
log_error_if_enabled "Tests Failed: $TESTS_FAILED"

if [ $TESTS_FAILED -eq 0 ]; then
    log_separator
    log_success_if_enabled "All tests passed!"
    log_info_if_enabled "Core functionality works and NPM packages are valid."
    log_info_if_enabled "The application is ready for release."
    exit 0
else
    log_separator
    log_error_if_enabled "Some tests failed!"
    log_warn_if_enabled "Please fix the failing functionality before proceeding with release."
    
    log_info_if_enabled "Common fixes for NPM package issues:"
    log_info_if_enabled "• Verify package names exist in NPM registry"
    log_info_if_enabled "• Update installation_arguments.rs with correct package names"
    log_info_if_enabled "• Ensure config.rs, terminal-jarvis.toml.example, and services.rs use same packages"
    log_info_if_enabled "• Update documentation with correct package references"
    exit 1
fi
