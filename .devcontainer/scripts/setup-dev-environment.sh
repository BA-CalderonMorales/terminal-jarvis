#!/bin/bash
set -e

echo "Setting up Terminal Jarvis development environment..."

# Install Node.js version 20 using NodeSource repository
echo "Installing Node.js version 20..."
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs

# Verify installations
echo "Rust toolchain info:"
rustc --version
cargo --version

echo "Node.js info:"
node --version
npm --version

# Install Rust components if not already present
echo "Installing additional Rust components..."
rustup component add clippy rustfmt || echo "Clippy and rustfmt already installed"

# Set up shell environment
echo "Setting up shell environment..."
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc

# Add custom PS1 prompt
if ! grep -q "# Terminal Jarvis Custom PS1" ~/.bashrc; then
    echo "Adding custom PS1 prompt..."
    cat >> ~/.bashrc << 'EOF'

# Terminal Jarvis Custom PS1
# Color definitions
COL_USER='\[\e[96m\]'      # Cyan for [me]
COL_PATH='\[\e[93m\]'      # Yellow for path
COL_BRANCH='\[\e[92m\]'    # Green for branch
COL_BRACKETS='\[\e[90m\]'  # Dark gray for brackets and punctuation
COL_RESET='\[\e[0m\]'      # Reset color

parse_git_branch() {
    git branch 2> /dev/null | sed -e '/^[^*]/d' -e 's/* \(.*\)/\1/'
}

set_bash_prompt() {
    local branch=$(parse_git_branch)
    if [ -n "$branch" ]; then
        PS1="${COL_BRACKETS}[${COL_USER}me${COL_BRACKETS}]:${COL_PATH}\w ${COL_BRACKETS}(${COL_BRANCH}$branch${COL_BRACKETS}): ${COL_RESET}"
    else
        PS1="${COL_BRACKETS}[${COL_USER}me${COL_BRACKETS}]:${COL_PATH}\w${COL_BRACKETS}: ${COL_RESET}"
    fi
}

PROMPT_COMMAND=set_bash_prompt
EOF
else
    echo "Custom PS1 prompt already present in bashrc."
fi

# Add Terminal Jarvis development welcome message to bashrc
WELCOME_MARKER="# Terminal Jarvis Development Welcome"
if ! grep -q "$WELCOME_MARKER" ~/.bashrc; then
    echo "Adding Terminal Jarvis development prompt..."
    cat >> ~/.bashrc << 'EOF'
# Terminal Jarvis Development Welcome
if [ "$TERM" != "dumb" ] && [ -t 1 ]; then
    echo ""
    echo "Welcome to Terminal Jarvis development!"
    echo "Environment: Rust $(rustc --version 2>/dev/null | cut -d' ' -f2 || echo 'N/A') + Node.js $(node --version 2>/dev/null || echo 'N/A')"
    echo ""
    echo "Available commands:"
    echo "  cargo check             # Verify Rust compilation"
    echo "  cargo run -- --help     # Terminal Jarvis help"
    echo "  cargo run -- list       # List AI tools"
    echo "  cargo test              # Run all tests"
    echo "  ./scripts/cicd/local-ci.sh # CI checks"
    echo ""
fi
EOF
else
    echo "Terminal Jarvis welcome prompt already present in bashrc."
fi

# Install npm dependencies for the project
echo "Installing NPM dependencies..."
if [ -d "npm/terminal-jarvis" ]; then
    cd npm/terminal-jarvis
    npm install || echo "npm install failed (non-blocking)"
    cd ../..
fi