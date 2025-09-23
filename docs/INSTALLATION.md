# Installation Guide

This document provides comprehensive installation instructions for Terminal Jarvis across different platforms and use cases.

## Recommended: Pre-configured Development Environment

**For the best Terminal Jarvis experience, especially for development and testing AI tools:**

### GitHub Codespaces (Cloud Development)
```bash
# Click "Code" → "Codespaces" → "Create codespace" on the repository
# Or use the direct link: https://github.com/codespaces/new?template_repository=BA-CalderonMorales/terminal-jarvis
```

**Advantages:**
- **Instant Setup**: Complete environment ready in 60 seconds
- **Zero Dependencies**: No local software installation required
- **Consistent Experience**: Same environment across all platforms
- **Pre-configured Tools**: Rust 1.87, Node.js 20, GitHub CLI, AI tools ready
- **Built-in Debugging**: Full debugging setup with LLDB and VS Code integration

### VS Code Dev Containers (Local Docker)
```bash
# Prerequisites: Docker Desktop + VS Code + Remote-Containers extension
# 1. Clone the repository
# 2. Open in VS Code
# 3. Click "Reopen in Container" when prompted
```

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

#### How NPM installation works

- The NPM package does not bundle large binaries.
- During installation, a postinstall script detects your OS/arch and downloads the correct binary from GitHub Releases.
- The binary is installed under the package’s `bin/` directory and launched via the `terminal-jarvis` shim.
- Tool configurations (`config/tools/*.toml`) are included and loaded at runtime by the binary.

#### NPM prerequisites

- `tar` must be available to extract the downloaded archive.
- Internet access is required during installation.

Install `tar` if needed:

```bash
# Debian/Ubuntu
sudo apt-get update && sudo apt-get install -y tar

# Fedora/RHEL/CentOS
sudo dnf install -y tar || sudo yum install -y tar

# Arch Linux
sudo pacman -S --noconfirm tar
```

macOS ships with `tar`. Windows users should prefer WSL2.

#### Re-running the installer

If the binary is missing (interrupted install), re-run postinstall:

```bash
# Global
npm rebuild -g terminal-jarvis || npm install -g terminal-jarvis@latest

# Local/project
npm rebuild terminal-jarvis || npm install terminal-jarvis@latest

# One-off run will also try to complete setup
npx terminal-jarvis --version
```

#### NPM Postinstall Behavior (Under the Hood)

To keep the NPM package lightweight and always deliver the correct binary for your platform, Terminal Jarvis uses a postinstall step:

- Version source: Uses the NPM package version to select the matching GitHub Release (tag `vX.Y.Z`).
- Download origin: Official release assets from this repository’s GitHub Releases.
- Asset naming:
	- macOS: `terminal-jarvis-mac.tar.gz`
	- Linux: `terminal-jarvis-linux.tar.gz`
- Extraction: Uses your system `tar` to extract the archive.
- Install location: Places the binary under the package’s `bin/` directory and launches it via the `terminal-jarvis` shim (exposed through NPM’s `bin`).
- Configs: Tool definitions (`config/tools/*.toml`) are included in the NPM package and loaded by the binary at runtime.
- Automatic retry: If you invoke `terminal-jarvis` and the binary is missing (e.g., interrupted install), the launcher will attempt to complete setup automatically.

Verification:

```bash
# Confirm version and tool discovery
npx terminal-jarvis --version
npx terminal-jarvis list
```

Corporate networks and proxies:

- Ensure your environment allows access to GitHub Releases.
- If behind a proxy, configure NPM accordingly (e.g., `npm config set https-proxy http://proxy:port`).

Security & provenance:

- Binaries are fetched directly from this repository’s official GitHub Releases under the exact version tag.
- No additional telemetry is collected by the installer.

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

### macOS

- macOS includes `tar` by default.
- Install via NPM, Homebrew, or Cargo.

```bash
npm install -g terminal-jarvis && terminal-jarvis --help
```

### Linux

```bash
# Most Linux distributions work out-of-the-box
sudo apt-get update && sudo apt-get install -y tar  # Debian/Ubuntu
npm install -g terminal-jarvis
```

### Windows

Native Windows installation via the NPM package is not yet supported. Use WSL2 for a seamless experience:

```bash
# In WSL2 (Ubuntu example)
sudo apt-get update && sudo apt-get install -y tar
npm install -g terminal-jarvis && terminal-jarvis --help
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

- **Node.js and NPM**: Required for NPM channel
- **tar**: Required for NPM installs to extract binaries
- **Internet connection**: For postinstall binary download

### Optional

- **Rust toolchain**: Only required for building from source (Cargo channel)
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

- **Size**: Very small tarball (no bundled Rust binary)
- **Contents**: POSIX launcher, postinstall script, tool configs (`config/tools/*.toml`)
- **Runtime Fetch**: Binary downloaded on install via postinstall
- **Dependencies**: Requires `tar` at install time; no runtime Node deps
- **Platforms**: Linux, macOS (Windows via WSL2)
- **Testing**: Integration tests and install-time verification
- **Current Version**: See the README badges or run `terminal-jarvis --version`
- **Known Issues**: See [LIMITATIONS.md](LIMITATIONS.md)

> Note: Offline installs via NPM are not supported due to install-time download. Use Cargo or prebuilt binaries for air-gapped environments.
