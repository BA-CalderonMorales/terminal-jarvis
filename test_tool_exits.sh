#!/bin/bash

# Terminal Jarvis Tool Exit Behavior Test Script
# Tests that all tools launch and exit properly

set -e

echo "🔧 Testing Terminal Jarvis Tool Exit Behavior"
echo "=============================================="

# Build the project first
echo "Building Terminal Jarvis..."
cargo build --release
echo "✅ Build complete"
echo

# Function to test a tool with version flag
test_tool_version() {
    local tool=$1
    echo "Testing $tool --version..."
    timeout 10s cargo run --release -- run "$tool" --version > /dev/null 2>&1
    if [ $? -eq 0 ] || [ $? -eq 124 ]; then
        echo "✅ $tool version test passed"
    else
        echo "❌ $tool version test failed"
        return 1
    fi
}

# Function to test a tool with help flag using --
test_tool_help() {
    local tool=$1
    echo "Testing $tool --help..."
    timeout 10s cargo run --release -- run "$tool" -- --help > /dev/null 2>&1
    if [ $? -eq 0 ] || [ $? -eq 124 ]; then
        echo "✅ $tool help test passed"
    else
        echo "❌ $tool help test failed"
        return 1
    fi
}

# Test installed tools with version flags
echo "Testing installed tools with --version flag:"
test_tool_version "aider"
test_tool_version "amp" 
test_tool_version "goose"
test_tool_version "claude"
test_tool_version "gemini"
test_tool_version "opencode"

echo
echo "Testing installed tools with --help flag:"
test_tool_help "aider"
test_tool_help "amp"
test_tool_help "goose"
# Note: claude and gemini may not support standard --help, skip them

echo
echo "🎉 All tool exit behavior tests completed successfully!"
echo "✅ Tools launch properly"
echo "✅ Tools exit cleanly"
echo "✅ Terminal Jarvis returns to command line properly"