#!/usr/bin/env bash

# Manual test script to demonstrate browser opening prevention
# This script simulates the problematic scenarios and shows how our fix works

# Source logger
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../logger/logger.sh
# shellcheck disable=SC1091
source "$SCRIPT_DIR/../logger/logger.sh"

log_header "Terminal Jarvis Authentication Test"
log_info_if_enabled "This script demonstrates the browser opening issue and our fix."

# Function to test tool without our fix
test_tool_without_fix() {
    local tool=$1
    log_section "Testing $tool without authentication prevention"
    
    # Clear API keys
    unset GOOGLE_API_KEY GEMINI_API_KEY QWEN_CODE_API_KEY DASHSCOPE_API_KEY
    
    # Try to run the tool in a way that might trigger browser opening
    log_info_if_enabled "Running: $tool --help"
    timeout 10s "$tool" --help 2>&1 | head -20
    echo
}

# Function to test tool with our fix
test_tool_with_fix() {
    local tool=$1
    log_section "Testing $tool WITH Terminal Jarvis authentication prevention"
    
    # Use Terminal Jarvis to run the tool (which includes our fix)
    log_info_if_enabled "Running: terminal-jarvis run $tool --help"
    timeout 10s terminal-jarvis run "$tool" --help 2>&1 | head -20
    echo
}

# Function to show environment detection
show_environment_detection() {
    log_section "Environment Detection"
    log_info_if_enabled "Current environment variables:"
    log_info_if_enabled "CI: ${CI:-not set}"
    log_info_if_enabled "DISPLAY: ${DISPLAY:-not set}"
    log_info_if_enabled "CODESPACES: ${CODESPACES:-not set}"
    log_info_if_enabled "TERM: ${TERM:-not set}"
    log_info_if_enabled "SSH_CONNECTION: ${SSH_CONNECTION:-not set}"
    echo

    # Check if we're in a browser-opening-problematic environment
    if [[ -n "$CI" ]] || [[ -z "$DISPLAY" ]] || [[ -n "$CODESPACES" ]] || [[ "$TERM" == "dumb" ]]; then
        log_success "Detected environment where browser opening should be prevented"
    else
        log_warn "Environment might allow browser opening"
    fi
    echo
}

# Function to demonstrate the fix
demonstrate_fix() {
    log_section "Demonstrating Terminal Jarvis Browser Prevention"
    
    # Clear API keys to simulate first-run scenario
    unset GOOGLE_API_KEY GEMINI_API_KEY QWEN_CODE_API_KEY DASHSCOPE_API_KEY
    
    log_info_if_enabled "Setting up authentication-safe environment..."
    
    # Show what our AuthManager does
    export NO_BROWSER=1
    export BROWSER=echo
    export OAUTH_NO_BROWSER=1
    
    log_info_if_enabled "Environment variables set:"
    log_info_if_enabled "NO_BROWSER: $NO_BROWSER"
    log_info_if_enabled "BROWSER: $BROWSER"
    log_info_if_enabled "OAUTH_NO_BROWSER: $OAUTH_NO_BROWSER"
    echo
    
    log_info_if_enabled "This configuration should prevent tools from opening browsers automatically."
    echo
}

# Function to show API key setup instructions
show_api_key_setup() {
    log_section "Recommended API Key Setup"
    log_info_if_enabled "To avoid authentication issues entirely, set these environment variables:"
    echo
    log_info_if_enabled "For Gemini CLI:"
    log_info_if_enabled "  export GOOGLE_API_KEY=\"your-google-api-key\""
    log_info_if_enabled "  # Get key from: https://makersuite.google.com/app/apikey"
    echo
    log_info_if_enabled "For Qwen Code:"
    log_info_if_enabled "  export QWEN_CODE_API_KEY=\"your-qwen-api-key\""
    log_info_if_enabled "  # Get key from: https://dashscope.console.aliyun.com/"
    echo
    log_info_if_enabled "Add these to your ~/.bashrc or ~/.zshrc for permanent setup."
    echo
}

# Main test execution
main() {
    show_environment_detection
    demonstrate_fix
    
    # Test if tools are available
    if command -v gemini >/dev/null 2>&1; then
        log_info_if_enabled "Gemini CLI is available for testing"
        test_tool_without_fix "gemini"
        if command -v terminal-jarvis >/dev/null 2>&1; then
            test_tool_with_fix "gemini"
        fi
    else
        log_warn "Gemini CLI not found. Install with: npm install -g @google/gemini-cli"
    fi
    
    if command -v qwen >/dev/null 2>&1; then
        log_info_if_enabled "Qwen Code is available for testing"
        test_tool_without_fix "qwen"
        if command -v terminal-jarvis >/dev/null 2>&1; then
            test_tool_with_fix "qwen"
        fi
    else
        log_warn "Qwen Code not found. Install with: npm install -g @qwen-code/qwen-code"
    fi
    
    show_api_key_setup
    
    log_header "Test Complete"
    log_info_if_enabled "Summary:"
    log_info_if_enabled "- Terminal Jarvis detects browser-problematic environments"
    log_info_if_enabled "- Sets NO_BROWSER and related environment variables"
    log_info_if_enabled "- Provides helpful API key setup instructions"
    log_info_if_enabled "- Prevents tools from opening browsers that would disrupt terminal sessions"
}

main "$@"
