#!/usr/bin/env bash
# Test script for Terminal Jarvis resize behavior
# This script provides instructions for manual testing of terminal resize functionality

set -e

echo "==================================="
echo "Terminal Jarvis Resize Test Script"
echo "==================================="
echo ""
echo "This script will build and run Terminal Jarvis for manual resize testing."
echo ""
echo "TESTING INSTRUCTIONS:"
echo "---------------------"
echo "1. After the menu appears, try resizing your terminal window"
echo "2. Make the terminal narrower (e.g., 50 columns)"
echo "3. Make the terminal wider (e.g., 150 columns)"
echo "4. Verify that the header (logo, borders) adjusts immediately"
echo "5. Navigate the menu with arrow keys while resizing"
echo "6. Try the filter (type letters) while resizing"
echo ""
echo "EXPECTED BEHAVIOR:"
echo "------------------"
echo "[SUCCESS] Header re-renders automatically when terminal resizes"
echo "[SUCCESS] No text wrapping or border misalignment"
echo "[SUCCESS] Menu remains functional during resize"
echo "[SUCCESS] Filter works correctly during resize"
echo ""
echo "Press Enter to build and start Terminal Jarvis..."
read -r

# Build the project
echo ""
echo "Building Terminal Jarvis..."
cargo build --release

# Run Terminal Jarvis in interactive mode
echo ""
echo "Starting Terminal Jarvis in interactive mode..."
echo "Use Ctrl+C or 'q' to exit"
echo ""
./target/release/terminal-jarvis interactive

echo ""
echo "==================================="
echo "Testing Complete"
echo "==================================="
