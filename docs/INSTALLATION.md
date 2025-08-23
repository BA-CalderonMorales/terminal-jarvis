# Installation Guide

This document provides comprehensive installation instructions for Terminal Jarvis across different platforms and use cases.

## Quick Installation

Terminal Jarvis is available through **three official distribution channels**:

### 1. NPM Installation (Recommended for Most Users)

```bash
# Try it instantly with npx (no installation required)
npx terminal-jarvis

# Install latest version globally
npm install -g terminal-jarvis

# Install stable version (recommended for production)
npm install -g terminal-jarvis@stable

# Install beta version (for testing new features)
npm install -g terminal-jarvis@beta
```

### 2. Rust Crate Installation (For Rust Developers)

```bash
# Install directly via Cargo
cargo install terminal-jarvis

# Verify installation
terminal-jarvis --help
```

### 3. Homebrew Installation (macOS/Linux Package Manager)

```bash
# Add the Terminal Jarvis tap
brew tap ba-calderonmorales/terminal-jarvis

# Install Terminal Jarvis
brew install terminal-jarvis

# Verify installation
terminal-jarvis --version
```

### Distribution Channel Comparison

| Method       | Best For                         | Pros                                        | Cons                    |
| ------------ | -------------------------------- | ------------------------------------------- | ----------------------- |
| **NPM**      | Node.js users, quick testing     | Instant with npx, multiple release channels | Requires Node.js        |
| **Cargo**    | Rust developers                  | Native Rust toolchain integration           | Requires Rust toolchain |
| **Homebrew** | macOS/Linux system package users | System package manager integration          | Limited to macOS/Linux  |

### NPM Distribution Channels

- **Latest** (`terminal-jarvis`): Most recently published version
- **Stable** (`terminal-jarvis@stable`): Production-ready, thoroughly tested releases
- **Beta** (`terminal-jarvis@beta`): Preview versions with experimental features

## Platform-Specific Instructions

### macOS Prerequisites

**Important**: macOS users must install Rust before using Terminal Jarvis.

```bash
# 1. Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Reload your shell environment
source ~/.cargo/env

# 3. Install Terminal Jarvis
npm install -g terminal-jarvis

# 4. Verify installation
terminal-jarvis --help
```

### Linux

```bash
# Most Linux distributions work out-of-the-box
npm install -g terminal-jarvis

# If you encounter issues, install Node.js first:
# Ubuntu/Debian:
sudo apt update && sudo apt install nodejs npm

# CentOS/RHEL/Fedora:
sudo dnf install nodejs npm
```

### Windows

```bash
# Install via NPM (works with PowerShell, Command Prompt, and WSL)
npm install -g terminal-jarvis

# For WSL users, follow Linux instructions
```

## Building from Source

### Prerequisites

- Rust 1.70 or later
- Node.js and NPM
- Git

### Steps

```bash
# 1. Clone the repository
git clone https://github.com/BA-CalderonMorales/terminal-jarvis.git
cd terminal-jarvis

# 2. Build the Rust application
cargo build --release

# 3. Install globally (optional)
cargo install --path .

# 4. Test the installation
terminal-jarvis --help
```

## Requirements and Dependencies

### Required

- **Node.js and NPM**: Required for most AI coding tools
- **Internet connection**: For package updates and installations

### Optional

- **Rust toolchain**: Only required for building from source or on macOS
- **`gh` CLI**: Optional, for template management features
- **Modern terminal**: For best visual experience (Unicode and color support)

## Troubleshooting Installation

### Common Issues

#### "command not found" after NPM install

```bash
# Check if NPM global bin is in your PATH
npm config get prefix

# Add to your shell profile (~/.bashrc, ~/.zshrc, etc.)
export PATH="$(npm config get prefix)/bin:$PATH"

# Reload your shell
source ~/.bashrc  # or ~/.zshrc
```

#### Permission errors on macOS/Linux

```bash
# Option 1: Use npx instead
npx terminal-jarvis

# Option 2: Configure NPM to use a different directory
mkdir ~/.npm-global
npm config set prefix '~/.npm-global'
echo 'export PATH=~/.npm-global/bin:$PATH' >> ~/.bashrc
source ~/.bashrc
```

#### Rust-related errors on macOS

```bash
# Ensure Rust is properly installed
rustc --version

# If not found, reinstall Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Verification Steps

After installation, verify everything works:

```bash
# Check version
terminal-jarvis --version

# List available tools
terminal-jarvis list

# Launch interactive mode
terminal-jarvis
```

## Uninstallation

### NPM Installation

```bash
npm uninstall -g terminal-jarvis
```

### Source Installation

```bash
cargo uninstall terminal-jarvis
```

## Package Information

**NPM Package Details:**

- **Size**: ~1.2MB compressed / ~2.9MB unpacked
- **Contents**: Pre-compiled binaries, TypeScript wrapper
- **Dependencies**: Zero runtime dependencies
- **Platforms**: Cross-platform support (Windows, macOS, Linux)
- **Testing**: All tools undergo comprehensive integration testing, but real-world usage patterns help identify edge cases
- **Current Version**: v0.0.61 with enhanced Homebrew support and comprehensive AI tool integration
- **Known Issues**: See [LIMITATIONS.md](LIMITATIONS.md) for detailed information on resolved and current limitations

> **Note**: The current NPM version includes full binary functionality with the complete T.JARVIS interface. No additional installation required!
