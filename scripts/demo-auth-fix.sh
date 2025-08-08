#!/bin/bash

# Terminal Jarvis Browser Prevention Demonstration Script
# Shows how the authentication fixes prevent browser opening

# Colors for output
CYAN='\033[0;96m'
BLUE='\033[0;94m'
GREEN='\033[0;92m'
YELLOW='\033[0;93m'
RED='\033[0;91m'
RESET='\033[0m'

BINARY="./target/release/terminal-jarvis"

echo -e "${CYAN}🔐 Terminal Jarvis Browser Prevention Demonstration${RESET}"
echo -e "${BLUE}This demonstrates how the authentication fixes prevent unwanted browser opening${RESET}"
echo ""

# Clear any existing API keys to simulate first-run scenario
unset GOOGLE_API_KEY GEMINI_API_KEY QWEN_CODE_API_KEY ANTHROPIC_API_KEY CLAUDE_API_KEY

echo -e "${CYAN}📋 Step 1: Show tool list and authentication warnings${RESET}"
$BINARY list
echo ""

echo -e "${CYAN}🖥️  Step 2: Demonstrate headless environment detection${RESET}"
export CI="true"
export TERM="dumb"  
unset DISPLAY
echo -e "${BLUE}Environment configured as headless (CI=true, no DISPLAY)${RESET}"
echo ""

echo -e "${CYAN}⚠️  Step 3: Try to run gemini without API key (should show warning)${RESET}"
echo -e "${YELLOW}Running: $BINARY run gemini --help${RESET}"
timeout 10s $BINARY run gemini --help 2>&1 | head -20
echo ""

echo -e "${CYAN}⚠️  Step 4: Try to run qwen without API key (should show warning)${RESET}"  
echo -e "${YELLOW}Running: $BINARY run qwen --help${RESET}"
timeout 10s $BINARY run qwen --help 2>&1 | head -20
echo ""

echo -e "${CYAN}✅ Step 5: Show that API keys prevent warnings${RESET}"
export GOOGLE_API_KEY="dummy-key-for-demo"
echo -e "${BLUE}Setting GOOGLE_API_KEY=dummy-key-for-demo${RESET}"
echo -e "${YELLOW}Running: $BINARY run gemini --help${RESET}"
timeout 10s $BINARY run gemini --help 2>&1 | head -10
echo ""

echo -e "${CYAN}🔧 Step 6: Show environment variables set by prevention mechanism${RESET}"
echo -e "${BLUE}Current authentication prevention environment:${RESET}"
echo "NO_BROWSER: ${NO_BROWSER:-<not set>}"
echo "CI: ${CI:-<not set>}"  
echo "DISPLAY: ${DISPLAY:-<not set>}"
echo "BROWSER: ${BROWSER:-<not set>}"
echo ""

echo -e "${CYAN}📚 Step 7: Show helpful API key setup messages${RESET}"
unset GOOGLE_API_KEY
echo -e "${BLUE}Without API key, users get helpful setup instructions:${RESET}"
timeout 10s $BINARY run gemini --help 2>&1 | grep -A 5 -B 5 "API.*key\|authentication\|export" || echo "No API key messages found"
echo ""

echo -e "${GREEN}✅ Demonstration complete!${RESET}"
echo -e "${BLUE}Key features demonstrated:${RESET}"
echo -e "${BLUE}• Browser opening prevented in headless/CI environments${RESET}"
echo -e "${BLUE}• Clear warnings shown when tools might open browsers${RESET}"
echo -e "${BLUE}• Helpful API key setup instructions provided${RESET}"
echo -e "${BLUE}• Environment variables set to prevent browser authentication${RESET}"
echo ""
echo -e "${CYAN}🛡️  Summary: Users will no longer be kicked out of terminal sessions${RESET}"
echo -e "${CYAN}due to unexpected browser authentication popups!${RESET}"
