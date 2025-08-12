#!/bin/bash

# OpenCode Input Focus Fix Manual Test Script
# This script allows you to manually test the opencode input focus bug fix

set -e

# Colors for output
CYAN='\033[0;96m'
BLUE='\033[0;94m'
GREEN='\033[0;92m'
YELLOW='\033[0;93m'
RED='\033[0;91m'
RESET='\033[0m'

echo -e "${CYAN}🧪 OpenCode Input Focus Fix - Manual Test${RESET}"
echo -e "${BLUE}This script tests the fix for opencode input focus on fresh installs${RESET}"
echo ""

# Check if opencode is installed
if ! command -v opencode &> /dev/null; then
    echo -e "${YELLOW}⚠️  OpenCode is not installed. Installing now...${RESET}"
    echo -e "${BLUE}This will test the 'fresh install' scenario exactly${RESET}"
    
    # Build terminal-jarvis first
    echo -e "${BLUE}Building terminal-jarvis...${RESET}"
    cargo build --release
    
    # Install opencode using terminal-jarvis
    echo -e "${BLUE}Installing opencode via terminal-jarvis...${RESET}"
    ./target/release/terminal-jarvis install opencode
    
    if ! command -v opencode &> /dev/null; then
        echo -e "${RED}❌ Failed to install opencode. Please install manually first.${RESET}"
        echo -e "${BLUE}Try: npm install -g opencode-ai@latest${RESET}"
        exit 1
    fi
    
    echo -e "${GREEN}✅ OpenCode installed successfully!${RESET}"
    echo ""
fi

# Function to test opencode launch via terminal-jarvis
test_opencode_launch() {
    local test_name="$1"
    local launch_method="$2"
    
    echo -e "${CYAN}📋 Test: $test_name${RESET}"
    echo -e "${BLUE}Launch method: $launch_method${RESET}"
    echo ""
    
    echo -e "${YELLOW}Instructions:${RESET}"
    echo -e "${BLUE}1. OpenCode will launch in a moment${RESET}"
    echo -e "${BLUE}2. Try typing immediately when the interface appears${RESET}"
    echo -e "${BLUE}3. The input box should be focused and accept typing right away${RESET}"
    echo -e "${BLUE}4. If you can type immediately without clicking: ✅ FIX WORKS${RESET}"
    echo -e "${BLUE}5. If you need to click first to type: ❌ FIX FAILED${RESET}"
    echo -e "${BLUE}6. Press Ctrl+C or exit opencode when done testing${RESET}"
    echo ""
    
    read -p "Press Enter to launch opencode and test input focus..."
    echo ""
    
    # Launch using the specified method
    eval "$launch_method"
    
    echo ""
    echo -e "${YELLOW}Test completed for: $test_name${RESET}"
    echo ""
}

echo -e "${BLUE}We'll test the opencode input focus fix in multiple scenarios:${RESET}"
echo ""

# Test 1: Direct terminal-jarvis launch (no arguments)
test_opencode_launch \
    "Direct launch via terminal-jarvis (TUI mode)" \
    "./target/release/terminal-jarvis run opencode"

# Test 2: Interactive mode selection
echo -e "${CYAN}📋 Test: Interactive Mode Selection${RESET}"
echo -e "${BLUE}Launch method: Interactive terminal-jarvis interface${RESET}"
echo ""

echo -e "${YELLOW}Instructions:${RESET}"
echo -e "${BLUE}1. Terminal Jarvis interactive interface will open${RESET}"
echo -e "${BLUE}2. Select 'opencode' from the tool list${RESET}"
echo -e "${BLUE}3. Press Enter for default arguments${RESET}"
echo -e "${BLUE}4. Test input focus when opencode loads${RESET}"
echo -e "${BLUE}5. Exit both opencode and terminal-jarvis when done${RESET}"
echo ""

read -p "Press Enter to launch interactive terminal-jarvis..."
echo ""

./target/release/terminal-jarvis

echo ""
echo -e "${YELLOW}Interactive mode test completed${RESET}"
echo ""

# Test 3: Compare with direct opencode launch (control test)
echo -e "${CYAN}📋 Control Test: Direct OpenCode Launch${RESET}"
echo -e "${BLUE}This tests opencode directly (not through terminal-jarvis)${RESET}"
echo ""

echo -e "${YELLOW}Instructions:${RESET}"
echo -e "${BLUE}1. This launches opencode directly without terminal-jarvis${RESET}"
echo -e "${BLUE}2. Compare input focus behavior with previous tests${RESET}"
echo -e "${BLUE}3. Direct launch should also work fine${RESET}"
echo ""

read -p "Press Enter to launch opencode directly for comparison..."
echo ""

opencode .

echo ""
echo -e "${YELLOW}Direct launch control test completed${RESET}"
echo ""

# Summary
echo -e "${CYAN}🏁 Test Summary${RESET}"
echo ""
echo -e "${BLUE}You have tested the opencode input focus fix in three scenarios:${RESET}"
echo -e "${GREEN}1. ✅ Direct terminal-jarvis launch: ${BLUE}./target/release/terminal-jarvis run opencode${RESET}"
echo -e "${GREEN}2. ✅ Interactive terminal-jarvis mode: ${BLUE}./target/release/terminal-jarvis${RESET}"
echo -e "${GREEN}3. ✅ Direct opencode launch (control): ${BLUE}opencode .${RESET}"
echo ""
echo -e "${YELLOW}Expected Results After Fix:${RESET}"
echo -e "${GREEN}• Input box should be focused immediately in ALL scenarios${RESET}"
echo -e "${GREEN}• No need to click or manually focus the input box${RESET}"
echo -e "${GREEN}• Typing should work immediately when interface appears${RESET}"
echo ""

echo -e "${CYAN}💡 Technical Details of the Fix:${RESET}"
echo -e "${BLUE}• Added special terminal state preparation for opencode${RESET}"
echo -e "${BLUE}• Minimal terminal sequence: \\x1b[H\\x1b[2J (home + clear)${RESET}"
echo -e "${BLUE}• Removed problematic sequences that caused strange characters${RESET}"
echo -e "${BLUE}• Added 75ms initialization delay to prevent race conditions${RESET}"
echo -e "${BLUE}• Fixed terminal interference with opencode's input handling${RESET}"
echo ""

echo -e "${GREEN}✅ Manual testing completed!${RESET}"
echo -e "${BLUE}If the input focus worked immediately in the terminal-jarvis scenarios,${RESET}"
echo -e "${BLUE}then the fix is working correctly.${RESET}"
