#!/bin/bash

# Fix Codespace Permissions Script
# This script fixes common permission issues in GitHub Codespaces

echo "Terminal Jarvis - Codespace Permission Fix"
echo "=========================================="

# Set correct environment variables
export HOME=/home/vscode
export CARGO_HOME=/home/vscode/.cargo
export RUSTUP_HOME=/home/vscode/.rustup
export PATH="/home/vscode/.cargo/bin:$PATH"

echo "[OK] Environment variables set:"
echo "   HOME=$HOME"
echo "   CARGO_HOME=$CARGO_HOME"
echo "   RUSTUP_HOME=$RUSTUP_HOME"

# Fix workspace ownership
echo "[FIXING] Fixing workspace ownership..."
sudo chown -R vscode:vscode /workspaces/terminal-jarvis 2>/dev/null || {
    echo "[WARNING] Could not fix workspace ownership (may already be correct)"
}

# Fix script permissions
echo "[FIXING] Fixing script permissions..."
chmod -R 755 /workspaces/terminal-jarvis/scripts 2>/dev/null || {
    echo "[WARNING] Could not fix script permissions (may already be correct)"
}
chmod +x /workspaces/terminal-jarvis/scripts/*/*.sh 2>/dev/null || {
    echo "[WARNING] Could not make scripts executable (may already be correct)"
}

# Test Rust environment
echo "[TESTING] Testing Rust environment..."
if command -v cargo >/dev/null 2>&1; then
    echo "[OK] Cargo found: $(cargo --version)"
    echo "[OK] Rust found: $(rustc --version)"
else
    echo "[ERROR] Rust not found. Sourcing environment..."
    source /home/vscode/.cargo/env
    if command -v cargo >/dev/null 2>&1; then
        echo "[OK] Cargo now available: $(cargo --version)"
    else
        echo "[ERROR] Rust still not available. Manual setup may be required."
    fi
fi

# Add environment to bashrc if not already there
if ! grep -q "source /home/vscode/.cargo/env" /home/vscode/.bashrc 2>/dev/null; then
    echo "[FIXING] Adding Rust environment to bashrc..."
    echo "source /home/vscode/.cargo/env" >> /home/vscode/.bashrc
    echo "export HOME=/home/vscode" >> /home/vscode/.bashrc
    echo "[OK] Bashrc updated"
else
    echo "[OK] Bashrc already configured"
fi

echo ""
echo "[COMPLETE] Permission fix complete!"
echo "If you're still experiencing issues, try:"
echo "   source /home/vscode/.cargo/env"
echo "   cargo --version"