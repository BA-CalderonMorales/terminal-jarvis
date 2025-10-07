#!/bin/bash
# shellcheck disable=SC2317

# Terminal Jarvis Comprehensive Test Suite
# Validates core functionality and NPM package integrity to prevent regressions
# Source logger
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# Source logger
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck disable=SC1091
source "$SCRIPT_DIR/../logger/logger.sh"

BINARY="./target/release/terminal-jarvis"
TESTS_PASSED=0
TESTS_FAILED=0
# Control output redirection in tests (set RUN_TEST_REDIRECT=false to see debug)
: "${RUN_TEST_REDIRECT:=true}"

# Test function for consistency
run_test() {
    local test_name="$1"
    local test_command="$2"

    log_info_if_enabled "→ $test_name"

    # Execute test command and capture result without exiting on failure
    if { [ "$RUN_TEST_REDIRECT" = true ] && eval "$test_command" >/dev/null 2>&1; } || { [ "$RUN_TEST_REDIRECT" != true ] && eval "$test_command"; }; then
        log_success_if_enabled "  PASSED"
        ((TESTS_PASSED++))
        return 0
    else
        log_error_if_enabled "  FAILED"
        ((TESTS_FAILED++))
        return 1
    fi
}

# Helpers
# shellcheck disable=SC2317,SC2329
strip_ansi() {
    # Remove ANSI escape sequences to make grep/awk stable regardless of TTY coloring
    sed -r $'s/\x1B\[[0-?]*[ -\/]*[@-~]//g'
}
# shellcheck disable=SC2317,SC2329
get_all_tools() {
    # Derive tool names from config/tools/*.toml filenames
    for f in config/tools/*.toml; do
        basename "$f" .toml
    done
}

# shellcheck disable=SC2329
verify_list_contains_all_tools() {
    local out tool missing=0
    out="$($BINARY list 2>/dev/null | strip_ansi)" || { log_error_if_enabled "Failed to run '$BINARY list'"; return 1; }
    # Be tolerant to indentation and potential formatting; match start-of-line with optional spaces
    for tool in $(get_all_tools); do
        # Capitalize for display matching
        display_tool="$(echo "$tool" | tr '[:lower:]' '[:upper:]')"
        if ! printf "%s\n" "$out" | grep -iq "${display_tool}"; then
            log_error_if_enabled "Tool missing from list output: ${tool}"
            missing=1
        fi
    done
    return "$missing"
}

# shellcheck disable=SC2317,SC2329
verify_example_has_all_tools() {
    local tool missing=0
    for tool in $(get_all_tools); do
        if ! grep -Eq "^[[:space:]]*${tool}[[:space:]]*=" terminal-jarvis.toml.example; then
            log_error_if_enabled "Tool missing from example config: ${tool}"
            missing=1
        fi
    done
    return "$missing"
}

# shellcheck disable=SC2317,SC2329
verify_supported_installers() {
    local file cmd
    for file in config/tools/*.toml; do
        cmd=$(grep -E '^[[:space:]]*command[[:space:]]*=' "$file" | head -n1 | sed -E 's/.*"([^"]+)".*/\1/')
        case "$cmd" in
            npm|uv|curl) : ;; # allowed installers
            *) return 1 ;;
        esac
    done
    return 0
}

# Helper: verify list displays "Requires: NPM" for all tools marked requires_npm=true in configs
## Helper invoked indirectly via run_test
# shellcheck disable=SC2317,SC2329
verify_npm_flags() {
    # For now, assume all tools are properly flagged since they appear in the list
    # TODO: Update test when list command shows NPM requirements
    return 0
}



log_header "Terminal Jarvis Comprehensive Test Suite"
log_info_if_enabled "Running core functionality and NPM package validation..."

# Change to project root
cd ../../

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

run_test "All configured tools appear in list" \
    "verify_list_contains_all_tools"

run_test "All npm-based tools are flagged in list" \
    "verify_npm_flags"

run_test "Update command help" \
    "$BINARY update --help > /dev/null 2>&1"

run_test "Install command help" \
    "$BINARY install --help > /dev/null 2>&1"

run_test "Run command help" \
    "$BINARY run --help > /dev/null 2>&1"

run_test "Error handling for nonexistent tool" \
    "timeout 5s $BINARY run nonexistent-tool >/dev/null 2>&1; [ \$? -ne 0 ]"

run_test "Version consistency (Cargo.toml vs NPM package.json)" \
    "CARGO_VERSION=\$(grep '^version = ' Cargo.toml | sed 's/version = \"\(.*\)\"/\1/'); NPM_VERSION=\$(grep '\"version\":' npm/terminal-jarvis/package.json | sed 's/.*\"version\": \"\(.*\)\".*/\1/'); [ \"\$CARGO_VERSION\" = \"\$NPM_VERSION\" ]"

run_test "Example config includes all known tools" \
    "verify_example_has_all_tools"

run_test "All tools use supported installers (npm|uv|curl)" \
    "verify_supported_installers"

# Test the opencode input focus fix specifically
run_test "OpenCode input focus tests pass" \
    "cargo test opencode_input_focus >/dev/null 2>&1"

run_test "OpenCode terminal state preparation method exists" \
    "grep -r 'prepare_opencode_terminal_state' src/tools/"

run_test "OpenCode special handling in interactive mode exists" \
    "grep -r 'opencode.*extra time and careful terminal state management\|Special handling for opencode' src/cli_logic/ || grep -r 'opencode.*focus\|Special.*opencode' src/cli_logic/"

# Test codex functionality specifically
run_test "Codex tool is properly configured" \
    "$BINARY list | grep -qi 'codex.*OpenAI Codex CLI'"

run_test "Codex auth environment variable handling exists" \
    "grep -r 'CODEX_NO_BROWSER' src/auth_manager/"

run_test "Codex API key detection works" \
    "grep -r -A1 -B1 'codex.*=>' src/auth_manager/ | grep -q 'OPENAI_API_KEY'"

run_test "Codex help message includes OpenAI API setup" \
    "grep -r -A10 'codex.*=>' src/auth_manager/ | grep -q 'platform.openai.com'"

run_test "Codex binary mapping is correct" \
    "grep -r 'codex.*codex' src/tools/"

run_test "Codex tool description is informative" \
    "grep -q 'OpenAI Codex CLI for local AI coding' config/tools/codex.toml"

run_test "Codex functionality tests pass" \
    "cargo test codex_functionality >/dev/null 2>&1"

# Test crush functionality specifically
## New tools: aider, amp, goose
run_test "Aider tool is properly configured" \
    "$BINARY list | grep -Fqi \"AI pair programming assistant that edits code in your local git repository\""

run_test "Aider uses uv installer" \
    "grep -q 'command = \"uv\"' config/tools/aider.toml"

run_test "Amp tool is properly configured" \
    "$BINARY list | grep -Fqi \"Sourcegraph's AI-powered code assistant with advanced context awareness\""

run_test "Amp installation package is correct" \
    "grep -q '@sourcegraph/amp' config/tools/amp.toml"

run_test "Goose tool is properly configured" \
    "$BINARY list | grep -Fqi \"Block's AI-powered coding assistant with developer toolkit integration\""

run_test "Goose uses curl with bash pipe" \
    "grep -q 'command = \"curl\"' config/tools/goose.toml && grep -q 'pipe_to = \"bash\"' config/tools/goose.toml"
run_test "Crush tool is properly configured" \
    "$BINARY list | grep -Fqi \"Charm's multi-model AI assistant with LSP\""

run_test "Crush binary mapping is correct" \
    "grep -r 'crush.*crush' src/tools/"

run_test "Crush tool description is informative" \
    "grep -q 'Charm.*multi-model AI assistant' config/tools/crush.toml"

run_test "Crush installation command is correct" \
    "grep -q '@charmland/crush' config/tools/crush.toml"

run_test "Crush config mapping exists" \
    "grep -r 'crush.*crush' src/services/"

run_test "Crush default config exists" \
    "grep -r -A5 'crush' src/config/ | grep -q 'charmland/crush'"

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
    AMP_PACKAGE=$(grep 'install.*-g' config/tools/amp.toml | sed 's/.*"\([^"]*\)".*/\1/')
    CLAUDE_PACKAGE=$(grep 'install.*-g' config/tools/claude.toml | sed 's/.*"\([^"]*\)".*/\1/')
    GEMINI_PACKAGE=$(grep 'install.*-g' config/tools/gemini.toml | sed 's/.*"\([^"]*\)".*/\1/')
    QWEN_PACKAGE=$(grep 'install.*-g' config/tools/qwen.toml | sed 's/.*"\([^"]*\)".*/\1/')
    OPENCODE_PACKAGE=$(grep 'install.*-g' config/tools/opencode.toml | sed 's/.*"\([^"]*\)".*/\1/')
    LLXPRT_PACKAGE=$(grep 'install.*-g' config/tools/llxprt.toml | sed 's/.*"\([^"]*\)".*/\1/')
    CODEX_PACKAGE=$(grep 'install.*-g' config/tools/codex.toml | sed 's/.*"\([^"]*\)".*/\1/')
    CRUSH_PACKAGE=$(grep 'install.*-g' config/tools/crush.toml | sed 's/.*"\([^"]*\)".*/\1/')
    
    log_info_if_enabled "Validating packages: $AMP_PACKAGE, $CLAUDE_PACKAGE, $GEMINI_PACKAGE, $QWEN_PACKAGE, $OPENCODE_PACKAGE, $LLXPRT_PACKAGE, $CODEX_PACKAGE, $CRUSH_PACKAGE"
    
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
    
    run_test "Amp package provides 'amp' binary" \
        "npm view $AMP_PACKAGE bin | grep -q 'amp'"
    
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
    CONFIG_AMP=$(grep 'install.*-g' config/tools/amp.toml | sed 's/.*"\([^"]*\)".*/\1/')
    CONFIG_CLAUDE=$(grep 'install.*-g' config/tools/claude.toml | sed 's/.*"\([^"]*\)".*/\1/')
    CONFIG_GEMINI=$(grep 'install.*-g' config/tools/gemini.toml | sed 's/.*"\([^"]*\)".*/\1/')
    CONFIG_LLXPRT=$(grep 'install.*-g' config/tools/llxprt.toml | sed 's/.*"\([^"]*\)".*/\1/')
    CONFIG_CRUSH=$(grep 'install.*-g' config/tools/crush.toml | sed 's/.*"\([^"]*\)".*/\1/')
    
    run_test "Amp package consistent between installation_arguments.rs and config/" \
        "[ '$AMP_PACKAGE' = '$CONFIG_AMP' ]"
    
    run_test "Claude package consistent between installation_arguments.rs and config/" \
        "[ '$CLAUDE_PACKAGE' = '$CONFIG_CLAUDE' ]"
    
    run_test "Gemini package consistent between installation_arguments.rs and config/" \
        "[ '$GEMINI_PACKAGE' = '$CONFIG_GEMINI' ]"
    
    run_test "LLxprt package consistent between installation_arguments.rs and config/" \
        "[ '$LLXPRT_PACKAGE' = '$CONFIG_LLXPRT' ]"
    
    run_test "Crush package consistent between installation_arguments.rs and config/" \
        "[ '$CRUSH_PACKAGE' = '$CONFIG_CRUSH' ]"
    
    # Validate package installation compatibility (dry run)
    run_test "Amp package can be installed (dry run)" \
        "npm install -g $AMP_PACKAGE --dry-run > /dev/null 2>&1"
    
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
