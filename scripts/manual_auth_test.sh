#!/usr/bin/env bash

# Manual test script to demonstrate browser opening prevention
# This script simulates the problematic scenarios and shows how our fix works

echo "=== Terminal Jarvis Authentication Test ==="
echo "This script demonstrates the browser opening issue and our fix."
echo

# Function to test tool without our fix
test_tool_without_fix() {
    local tool=$1
    echo "--- Testing $tool without authentication prevention ---"
    
    # Clear API keys
    unset GOOGLE_API_KEY GEMINI_API_KEY QWEN_CODE_API_KEY DASHSCOPE_API_KEY
    
    # Try to run the tool in a way that might trigger browser opening
    echo "Running: $tool --help"
    timeout 10s $tool --help 2>&1 | head -20
    echo
}

# Function to test tool with our fix
test_tool_with_fix() {
    local tool=$1
    echo "--- Testing $tool WITH Terminal Jarvis authentication prevention ---"
    
    # Use Terminal Jarvis to run the tool (which includes our fix)
    echo "Running: terminal-jarvis run $tool --help"
    timeout 10s terminal-jarvis run $tool --help 2>&1 | head -20
    echo
}

# Function to show environment detection
show_environment_detection() {
    echo "--- Environment Detection ---"
    echo "Current environment variables:"
    echo "CI: ${CI:-not set}"
    echo "DISPLAY: ${DISPLAY:-not set}"
    echo "CODESPACES: ${CODESPACES:-not set}"
    echo "TERM: ${TERM:-not set}"
    echo "SSH_CONNECTION: ${SSH_CONNECTION:-not set}"
    echo

    # Check if we're in a browser-opening-problematic environment
    if [[ -n "$CI" ]] || [[ -z "$DISPLAY" ]] || [[ -n "$CODESPACES" ]] || [[ "$TERM" == "dumb" ]]; then
        echo "✅ Detected environment where browser opening should be prevented"
    else
        echo "⚠️  Environment might allow browser opening"
    fi
    echo
}

# Function to demonstrate the fix
demonstrate_fix() {
    echo "--- Demonstrating Terminal Jarvis Browser Prevention ---"
    
    # Clear API keys to simulate first-run scenario
    unset GOOGLE_API_KEY GEMINI_API_KEY QWEN_CODE_API_KEY DASHSCOPE_API_KEY
    
    echo "Setting up authentication-safe environment..."
    
    # Show what our AuthManager does
    export NO_BROWSER=1
    export BROWSER=echo
    export OAUTH_NO_BROWSER=1
    
    echo "Environment variables set:"
    echo "NO_BROWSER: $NO_BROWSER"
    echo "BROWSER: $BROWSER" 
    echo "OAUTH_NO_BROWSER: $OAUTH_NO_BROWSER"
    echo
    
    echo "This configuration should prevent tools from opening browsers automatically."
    echo
}

# Function to show API key setup instructions
show_api_key_setup() {
    echo "--- Recommended API Key Setup ---"
    echo "To avoid authentication issues entirely, set these environment variables:"
    echo
    echo "For Gemini CLI:"
    echo "  export GOOGLE_API_KEY=\"your-google-api-key\""
    echo "  # Get key from: https://makersuite.google.com/app/apikey"
    echo
    echo "For Qwen Code:"
    echo "  export QWEN_CODE_API_KEY=\"your-qwen-api-key\""
    echo "  # Get key from: https://dashscope.console.aliyun.com/"
    echo
    echo "Add these to your ~/.bashrc or ~/.zshrc for permanent setup."
    echo
}

# Main test execution
main() {
    show_environment_detection
    demonstrate_fix
    
    # Test if tools are available
    if command -v gemini >/dev/null 2>&1; then
        echo "📋 Gemini CLI is available for testing"
        test_tool_without_fix "gemini"
        if command -v terminal-jarvis >/dev/null 2>&1; then
            test_tool_with_fix "gemini"
        fi
    else
        echo "⚠️  Gemini CLI not found. Install with: npm install -g @google/gemini-cli"
    fi
    
    if command -v qwen >/dev/null 2>&1; then
        echo "📋 Qwen Code is available for testing"
        test_tool_without_fix "qwen"
        if command -v terminal-jarvis >/dev/null 2>&1; then
            test_tool_with_fix "qwen"
        fi
    else
        echo "⚠️  Qwen Code not found. Install with: npm install -g @qwen-code/qwen-code"
    fi
    
    show_api_key_setup
    
    echo "=== Test Complete ==="
    echo "Summary:"
    echo "- Terminal Jarvis detects browser-problematic environments"
    echo "- Sets NO_BROWSER and related environment variables"
    echo "- Provides helpful API key setup instructions"
    echo "- Prevents tools from opening browsers that would disrupt terminal sessions"
}

main "$@"
