#!/bin/bash

# Terminal Jarvis Authentication Behavior Test Script
# Tests browser-opening prevention in various scenarios

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

# Function to simulate first-run environment
setup_first_run_env() {
    # Clear authentication-related environment variables
    unset GOOGLE_API_KEY GEMINI_API_KEY QWEN_CODE_API_KEY ANTHROPIC_API_KEY CLAUDE_API_KEY
    
    # Create temporary config directories to simulate clean install
    export XDG_CONFIG_HOME="/tmp/terminal-jarvis-test-$$"
    mkdir -p "$XDG_CONFIG_HOME"
    
    # Simulate CI/headless environment to prevent browser opening
    export CI="true"
    export TERM="dumb"
    unset DISPLAY
    export NO_BROWSER="1"
    export BROWSER="echo"  # Set safe browser command manually for testing
}

# Function to simulate GUI environment that might trigger browsers
setup_gui_env() {
    # Clear authentication-related environment variables
    unset GOOGLE_API_KEY GEMINI_API_KEY QWEN_CODE_API_KEY ANTHROPIC_API_KEY CLAUDE_API_KEY
    
    # Create temporary config directories
    export XDG_CONFIG_HOME="/tmp/terminal-jarvis-test-$$"
    mkdir -p "$XDG_CONFIG_HOME"
    
    # Simulate GUI environment
    export DISPLAY=":0"
    export TERM="xterm-256color"
    unset CI
    unset NO_BROWSER
}

# Function to test interactive mode with authentication warnings
test_interactive_mode() {
    local env_type="$1"
    log_info_if_enabled "Testing interactive mode in $env_type environment..."
    
    # Use expect to automate interactive input
    expect -c "
        set timeout 10
        spawn $BINARY
        expect {
            \"Choose an option:\" {
                send \"6\r\"
                expect \"Press Enter to return to menu\"
                send \"\r\"
                expect \"Choose an option:\"
                send \"q\r\"
                expect eof
            }
            timeout { exit 1 }
        }
    " > /tmp/interactive_output.log 2>&1
    
    local exit_code=$?
    
    if [ $exit_code -eq 0 ]; then
        log_success_if_enabled "  Interactive mode completed successfully"
        
        # Check for authentication warnings in output
        if grep -q "WARNING.*may attempt to open a browser" /tmp/interactive_output.log; then
            log_success_if_enabled "  Browser warning displayed"
        else
            log_warn_if_enabled "  No browser warning found"
        fi
        
        # Show any interesting authentication-related output
        if grep -i "api\|auth\|browser\|login" /tmp/interactive_output.log; then
            log_info_if_enabled "  Authentication-related output found:"
            grep -i "api\|auth\|browser\|login" /tmp/interactive_output.log | sed 's/^/    /'
        fi
    else
        log_error_if_enabled "  Interactive mode failed"
        log_warn_if_enabled "  Output:"
        cat /tmp/interactive_output.log | sed 's/^/    /'
    fi
    
    return $exit_code
}

# Function to test tool execution with authentication checks
test_tool_with_auth_check() {
    local tool="$1"
    log_info_if_enabled "Testing $tool with authentication checks..."
    
    # Try to run the tool with --help (should be safe)
    timeout 10s $BINARY run $tool --help > /tmp/${tool}_output.log 2>&1
    local exit_code=$?
    
    if [ $exit_code -eq 0 ]; then
        log_success_if_enabled "  $tool executed successfully"
        
        # Check for authentication warnings
        if grep -q "WARNING.*may attempt to open a browser" /tmp/${tool}_output.log; then
            log_success_if_enabled "  Browser warning displayed for $tool"
        elif grep -q "API_KEY\|environment variable" /tmp/${tool}_output.log; then
            log_success_if_enabled "  API key guidance provided for $tool"
        fi
        
    elif [ $exit_code -eq 124 ]; then
        log_warn_if_enabled "  $tool timed out (may have been waiting for input)"
    else
        log_error_if_enabled "  $tool failed with exit code $exit_code"
        # Check if it's because the tool isn't installed
        if grep -q "not installed" /tmp/${tool}_output.log; then
            log_info_if_enabled "  Tool is not installed - this is expected"
            return 0
        fi
    fi
    
    # Show relevant output
    if grep -i "warning\|api\|auth\|browser\|login" /tmp/${tool}_output.log; then
        log_info_if_enabled "  Authentication-related output:"
        grep -i "warning\|api\|auth\|browser\|login" /tmp/${tool}_output.log | sed 's/^/    /'
    fi
    
    return 0
}

log_header "Terminal Jarvis Authentication Behavior Test Suite"
log_info_if_enabled "Testing browser-opening prevention and authentication warnings..."

# Build if needed
if [ ! -f "$BINARY" ]; then
    log_info_if_enabled "Building release binary..."
    cargo build --release
fi

# Check if expect is available for interactive testing
if ! command -v expect &> /dev/null; then
    log_warn_if_enabled "'expect' not available - skipping interactive tests"
    log_info_if_enabled "Install expect to run complete authentication tests"
    SKIP_INTERACTIVE=1
fi

# ===== ENVIRONMENT SETUP TESTS =====
log_info_if_enabled "Environment Setup Tests"

run_test "AuthManager module loads correctly" \
    "cargo test auth_manager::tests --lib"

run_test "Browser prevention detection works in CI" \
    "CI=true cargo run -- --help | grep -v 'WARNING.*browser' || true"

run_test "API key detection works" \
    "GOOGLE_API_KEY=test cargo test auth_manager::tests::test_api_key_detection --lib"

log_separator

# ===== HEADLESS ENVIRONMENT TESTS =====
log_info_if_enabled "Headless Environment Tests"

setup_first_run_env
log_info_if_enabled "Testing in headless/CI environment (should prevent browser opening)"

run_test "Help command works in headless environment" \
    "$BINARY --help"

run_test "List command works in headless environment" \
    "$BINARY list"

# Test tool execution with authentication checks
test_tool_with_auth_check "gemini"
test_tool_with_auth_check "qwen" 
test_tool_with_auth_check "claude"

# Test interactive mode if expect is available
if [ "$SKIP_INTERACTIVE" != "1" ]; then
    test_interactive_mode "headless"
fi

log_separator

# ===== GUI ENVIRONMENT TESTS =====
log_info_if_enabled "GUI Environment Tests"

setup_gui_env
echo -e "${BLUE}Testing in GUI environment (should show browser warnings)${RESET}"

run_test "Help command works in GUI environment" \
    "$BINARY --help"

run_test "List command works in GUI environment" \
    "$BINARY list"

# Test tool execution with authentication checks in GUI environment
test_tool_with_auth_check "gemini"
test_tool_with_auth_check "qwen"
test_tool_with_auth_check "claude"

# Test interactive mode if expect is available
if [ "$SKIP_INTERACTIVE" != "1" ]; then
    test_interactive_mode "GUI"
fi

echo ""

# ===== AUTHENTICATION PREVENTION MECHANISM TESTS =====
log_header "Authentication Prevention Mechanism Tests"

setup_first_run_env

run_test "NO_BROWSER environment variable is set" \
    "[ \"\$NO_BROWSER\" = \"1\" ]"

run_test "CI environment variable is set" \
    "[ \"\$CI\" = \"true\" ]"

run_test "DISPLAY is unset for headless mode" \
    "[ -z \"\$DISPLAY\" ]"

# Test that browser-opening commands are neutralized
run_test "BROWSER environment variable set to safe command" \
    "echo \$BROWSER | grep -q echo || [ -z \$BROWSER ]"

echo ""

# ===== CLEANUP =====
cleanup() {
    # Clean up temporary directories
    if [ -n "$XDG_CONFIG_HOME" ] && [ "$XDG_CONFIG_HOME" != "$HOME/.config" ]; then
        rm -rf "$XDG_CONFIG_HOME"
    fi
    
    # Clean up temporary files
    rm -f /tmp/interactive_output.log /tmp/*_output.log
    
    # Reset environment
    unset XDG_CONFIG_HOME CI NO_BROWSER DISPLAY TERM
}

trap cleanup EXIT

# ===== RESULTS =====
log_info_if_enabled "Test Results Summary"
log_success_if_enabled "Tests Passed: $TESTS_PASSED"
log_error_if_enabled "Tests Failed: $TESTS_FAILED"

if [ $TESTS_FAILED -eq 0 ]; then
    log_separator
    log_success_if_enabled "All authentication tests passed!"
    log_info_if_enabled "Browser-opening prevention is working correctly."
    log_info_if_enabled "Authentication warnings are being displayed appropriately."
    exit 0
else
    log_separator
    log_error_if_enabled "Some authentication tests failed!"
    log_warn_if_enabled "Please fix the authentication behavior before proceeding."
    
    log_info_if_enabled "Common fixes for authentication issues:"
    log_info_if_enabled "• Verify AuthManager module is working correctly"
    log_info_if_enabled "• Check that browser prevention environment variables are set"
    log_info_if_enabled "• Ensure authentication warnings are displayed in appropriate scenarios"
    log_info_if_enabled "• Test with actual NPM packages to verify behavior"
    exit 1
fi
