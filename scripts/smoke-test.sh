#!/bin/bash

# Terminal Jarvis Smoke Test
# Quick validation that basic functionality works

set -e

BINARY="./target/release/terminal-jarvis"

# Build if needed
if [ ! -f "$BINARY" ]; then
    echo "Building release binary..."
    cargo build --release
fi

echo "🚀 Running smoke test..."

# Test 1: CLI works
echo -n "CLI help... "
$BINARY --help > /dev/null
echo "✅"

# Test 2: List command works  
echo -n "List tools... "
$BINARY list > /dev/null
echo "✅"

# Test 3: All 4 tools present
echo -n "Tool count... "
TOOL_COUNT=$($BINARY list 2>/dev/null | grep -E "^  (claude|gemini|qwen|opencode)" | wc -l)
if [ "$TOOL_COUNT" -ne 4 ]; then
    echo "❌ Expected 4, found $TOOL_COUNT"
    exit 1
fi
echo "✅"

# Test 4: NPM consistency
echo -n "NPM consistency... "
NPM_COUNT=$($BINARY list 2>/dev/null | grep -c "Requires: NPM")
if [ "$NPM_COUNT" -ne 4 ]; then
    echo "❌ Expected 4 NPM tools, found $NPM_COUNT"
    exit 1
fi
echo "✅"

echo "🎉 Smoke test passed! Core functionality is working."
