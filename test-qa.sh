#!/bin/bash
set -e

echo "=== NVM Fix Test for Issue #37 ==="
echo ""

echo "[1/5] Check NVM environment..."
if command -v nvm &> /dev/null || [ -d "$HOME/.nvm" ]; then
  echo "  NVM detected: $(nvm --version 2>/dev/null || echo present)"
else
  echo "  NVM not detected (using standard node)"
fi
echo "  Node: $(node --version)"
echo "  NPM: $(npm --version)"
echo "  NPM prefix: $(npm config get prefix)"
echo ""

echo "[2/5] Install terminal-jarvis globally..."
npm install -g terminal-jarvis@0.0.73
echo ""

echo "[3/5] Verify installation..."
echo "  Version: $(terminal-jarvis --version)"
echo "  Binary: $(which terminal-jarvis)"
echo ""

echo "[4/5] List available tools..."
terminal-jarvis list
echo ""

echo "[5/5] Test tool installation (claude)..."
echo "  Attempting: terminal-jarvis install claude"
terminal-jarvis install claude 2>&1 || echo "  (Installation may require auth - check error output above)"
echo ""

echo "=== TEST COMPLETE ==="
echo "If you see 'sudo: npm: command not found' above, the fix is NOT working."
echo "If installation proceeds (even with auth prompt), the fix is working."
