#!/usr/bin/env bash
# Test script for terminal resize functionality
# This demonstrates that the ResponsiveSelectMenu properly handles terminal resize events

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

echo "=========================================="
echo "Terminal Resize Test for Terminal Jarvis"
echo "=========================================="
echo ""
echo "This test demonstrates the new ResponsiveSelectMenu"
echo "which properly handles terminal resize events in real-time."
echo ""
echo "INSTRUCTIONS:"
echo "1. The application will start in interactive mode"
echo "2. While in any menu, resize your terminal window"
echo "3. The header and menu should adapt immediately"
echo "4. Test with different menu options (AI Tools, Settings, etc.)"
echo "5. Press Ctrl+C or select 'Exit' when done"
echo ""
echo "Expected behavior:"
echo "  ✓ Header re-renders to match new terminal width"
echo "  ✓ Menu options remain centered and readable"
echo "  ✓ No visual artifacts or text wrapping issues"
echo "  ✓ Works seamlessly during menu navigation"
echo ""
read -p "Press Enter to start the test..."
echo ""

cd "$PROJECT_ROOT"

# Run the application in interactive mode
cargo run --release -- interactive

echo ""
echo "=========================================="
echo "Test Complete"
echo "=========================================="
echo ""
echo "Did the terminal resize work correctly? (y/n)"
read -r response

if [[ "$response" =~ ^[Yy]$ ]]; then
    echo "✓ SUCCESS: Terminal resize functionality working as expected!"
    exit 0
else
    echo "✗ ISSUE: Please report any problems you encountered"
    exit 1
fi
