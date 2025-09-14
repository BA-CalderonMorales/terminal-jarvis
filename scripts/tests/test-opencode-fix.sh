#!/bin/bash

# OpenCode Input Focus Fix Manual Test Script
# This script allows you to manually test the opencode input focus bug fix

set -e

# Source logger
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../logger/logger.sh"

log_header "OpenCode Input Focus Fix - Manual Test"
log_info_if_enabled "This script tests the fix for opencode input focus on fresh installs"
echo

# Check if opencode is installed
if ! command -v opencode &> /dev/null; then
    log_warn "OpenCode is not installed. Installing now..."
    log_info_if_enabled "This will test the 'fresh install' scenario exactly"
    
    # Build terminal-jarvis first
    log_info_if_enabled "Building terminal-jarvis..."
    cargo build --release
    
    # Install opencode using terminal-jarvis
    log_info_if_enabled "Installing opencode via terminal-jarvis..."
    ./target/release/terminal-jarvis install opencode
    
    if ! command -v opencode &> /dev/null; then
        log_error "Failed to install opencode. Please install manually first."
        log_info_if_enabled "Try: npm install -g opencode-ai@latest"
        exit 1
    fi
    
    log_success "OpenCode installed successfully!"
    echo
fi

# Function to test opencode launch via terminal-jarvis
test_opencode_launch() {
    local test_name="$1"
    local launch_method="$2"
    
    log_subheader "Test: $test_name"
    log_info_if_enabled "Launch method: $launch_method"
    echo
    
    log_warn "Instructions:"
    log_info_if_enabled "1. OpenCode will launch in a moment"
    log_info_if_enabled "2. Try typing immediately when the interface appears"
    log_info_if_enabled "3. The input box should be focused and accept typing right away"
    log_info_if_enabled "4. If you can type immediately without clicking: FIX WORKS"
    log_info_if_enabled "5. If you need to click first to type: FIX FAILED"
    log_info_if_enabled "6. Press Ctrl+C or exit opencode when done testing"
    echo
    
    read -p "Press Enter to launch opencode and test input focus..."
    echo
    
    # Launch using the specified method
    eval "$launch_method"
    
    echo
    log_warn "Test completed for: $test_name"
    echo
}

log_info_if_enabled "We'll test the opencode input focus fix in multiple scenarios:"
echo

# Test 1: Direct terminal-jarvis launch (no arguments)
test_opencode_launch \
    "Direct launch via terminal-jarvis (TUI mode)" \
    "./target/release/terminal-jarvis run opencode"

# Test 2: Interactive mode selection
log_subheader "Test: Interactive Mode Selection"
log_info_if_enabled "Launch method: Interactive terminal-jarvis interface"
echo

log_warn "Instructions:"
log_info_if_enabled "1. Terminal Jarvis interactive interface will open"
log_info_if_enabled "2. Select 'opencode' from the tool list"
log_info_if_enabled "3. Press Enter for default arguments"
log_info_if_enabled "4. Test input focus when opencode loads"
log_info_if_enabled "5. Exit both opencode and terminal-jarvis when done"
echo

read -p "Press Enter to launch interactive terminal-jarvis..."
echo

./target/release/terminal-jarvis

echo
log_warn "Interactive mode test completed"
echo

# Test 3: Compare with direct opencode launch (control test)
log_subheader "Control Test: Direct OpenCode Launch"
log_info_if_enabled "This tests opencode directly (not through terminal-jarvis)"
echo

log_warn "Instructions:"
log_info_if_enabled "1. This launches opencode directly without terminal-jarvis"
log_info_if_enabled "2. Compare input focus behavior with previous tests"
log_info_if_enabled "3. Direct launch should also work fine"
echo

read -p "Press Enter to launch opencode directly for comparison..."
echo

opencode .

echo
log_warn "Direct launch control test completed"
echo

# Summary
log_header "Test Summary"
echo
log_info_if_enabled "You have tested the opencode input focus fix in three scenarios:"
log_success "1. Direct terminal-jarvis launch: ./target/release/terminal-jarvis run opencode"
log_success "2. Interactive terminal-jarvis mode: ./target/release/terminal-jarvis"
log_success "3. Direct opencode launch (control): opencode ."
echo
log_warn "Expected Results After Fix:"
log_success "• Input box should be focused immediately in ALL scenarios"
log_success "• No need to click or manually focus the input box"
log_success "• Typing should work immediately when interface appears"
echo

log_subheader "Technical Details of the Fix:"
log_info_if_enabled "• Added special terminal state preparation for opencode"
log_info_if_enabled "• Minimal terminal sequence: \\x1b[H\\x1b[2J (home + clear)"
log_info_if_enabled "• Removed problematic sequences that caused strange characters"
log_info_if_enabled "• Added 75ms initialization delay to prevent race conditions"
log_info_if_enabled "• Fixed terminal interference with opencode's input handling"
echo

log_success "Manual testing completed!"
log_info_if_enabled "If the input focus worked immediately in the terminal-jarvis scenarios,"
log_info_if_enabled "then the fix is working correctly."
