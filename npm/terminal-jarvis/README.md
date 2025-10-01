<div align="center">

# Terminal Jarvis

</div>

<div align="center">

A unified command center for AI coding tools. Manage and run a suite of coding assistants from one beautiful terminal interface. See the full list in the [Supported AI Tools guide](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/).

</div>

<div align="center">

<img src="https://raw.githubusercontent.com/BA-CalderonMorales/terminal-jarvis/docs/screenshots_and_demos/screenshots_and_demo/promo_image_for_readme.png" alt="Terminal Jarvis Interface">

<div align="left">

## Badges

Distribution and project status at a glance: NPM and Crates.io versions/downloads, Homebrew availability, license, acknowledgements, and support. Click any badge for details.

</div>

<p align="center">
    <a href="https://www.npmjs.com/package/terminal-jarvis"><img src="https://img.shields.io/npm/v/terminal-jarvis/stable.svg?label=NPM%20Stable&color=green&logo=npm&style=for-the-badge" alt="NPM Stable"></a>
    <a href="https://www.npmjs.com/package/terminal-jarvis"><img src="https://img.shields.io/npm/v/terminal-jarvis/beta.svg?label=NPM%20Beta&color=orange&logo=npm&style=for-the-badge" alt="NPM Beta"></a>
    <a href="https://www.npmjs.com/package/terminal-jarvis"><img src="https://img.shields.io/npm/v/terminal-jarvis.svg?logo=npm&label=NPM%20Version&style=for-the-badge" alt="NPM Version"></a>
    <a href="https://www.npmjs.com/package/terminal-jarvis"><img src="https://img.shields.io/npm/dm/terminal-jarvis.svg?logo=npm&label=NPM%20Downloads&style=for-the-badge" alt="NPM Downloads"></a>
</p>

<p align="center">
    <a href="https://crates.io/crates/terminal-jarvis"><img src="https://img.shields.io/crates/v/terminal-jarvis.svg?logo=rust&label=Crates.io%20Version&style=for-the-badge" alt="Crates.io Version"></a>
    <a href="https://crates.io/crates/terminal-jarvis"><img src="https://img.shields.io/crates/d/terminal-jarvis.svg?logo=rust&label=Crates.io%20Downloads&style=for-the-badge" alt="Crates.io Downloads"></a>
</p>

<p align="center">
    <a href="https://github.com/BA-CalderonMorales/homebrew-terminal-jarvis"><img src="https://img.shields.io/badge/Homebrew-Available-blue.svg?logo=homebrew&style=for-the-badge" alt="Homebrew"></a>
</p>

<p align="center">
    <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge" alt="License: MIT"></a>
    <a href="https://github.com/Piebald-AI/awesome-gemini-cli"><img src="https://img.shields.io/badge/Mentioned%20in-awesome-6f42c1?style=for-the-badge" alt="Mentioned in Awesome Gemini CLI"></a>
    <a href="https://www.buymeacoffee.com/brandoncalderonmorales"><img src="https://img.shields.io/badge/Buy%20Me%20a%20Coffee-orange.svg?style=for-the-badge" alt="Buy Me a Coffee"></a>
</p>

</div>

## Demo

This short clip shows the interactive T.JARVIS interface in action: list installed/available tools, perform one-command installs, and continue sessions seamlessly during authentication. Try it locally with `npx terminal-jarvis`.

<p align="center">
    <img src="https://raw.githubusercontent.com/BA-CalderonMorales/terminal-jarvis/docs/screenshots_and_demos/screenshots_and_demo/Terminal%20Jarvis%20Demo.gif" alt="Terminal Jarvis Demo" width="100%">
</p>

## Prerequisites

Pick your setup path: zero-setup cloud/dev container, or local tools—whichever gets you coding fastest.

### **1. Recommended: Remote Development Environment (Zero Setup)**

The optimal way to use Terminal Jarvis is through a pre-configured remote development environment:

- **[Open in GitHub Codespaces](https://github.com/codespaces/new?template_repository=BA-CalderonMorales/terminal-jarvis)** - Instant, cloud-based development environment
- **[Use VS Code Dev Containers](https://code.visualstudio.com/docs/remote/containers)** - Local containerized environment
- **[Coder](https://coder.com), [DevPod](https://devpod.sh), or [Ona (GitPod)](https://www.gitpod.io)** - Alternative remote dev environments with cloud or self-hosted options

**Why this approach is ideal:**
- **Zero Setup Time**: Complete development environment ready in ~60 seconds
- **Consistent Environment**: Same setup across all contributors and platforms
- **Pre-installed Tools**: Rust 1.87, Node.js 20, GitHub CLI, Git LFS, LLDB debugger
- **VS Code Extensions**: GitHub Copilot, Rust debugging, TOML support pre-configured
- **Optimized Settings**: File watching excludes, format-on-save, search optimization
- **AI-Assisted Development**: GitHub Copilot pre-configured
- **All Dependencies Ready**: No manual installation of compilers or tools

### **2. Alternative: Local Installation**

If you prefer local development:

- **Node.js 20+** and NPM
- **macOS users**: No Rust required for NPM/Homebrew; Rust only needed for Cargo/source builds
- **Linux users**: `tar` for NPM installs; standard build tools if building from source
- **Windows users**: Windows Subsystem for Linux (WSL2) recommended

> [!IMPORTANT]
> **Full documentation:** [Terminal Jarvis Docs](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/)

## Quick Start

Launch in seconds or install for daily use—choose the method that fits your workflow.

```bash
# Try instantly (no installation required)
npx terminal-jarvis

# Install globally for regular use
npm install -g terminal-jarvis

# Install stable version (recommended for production)
npm install -g terminal-jarvis@stable

# Install via Cargo (Rust users)
cargo install terminal-jarvis

# Install via Homebrew (macOS/Linux)
brew tap ba-calderonmorales/terminal-jarvis
brew install terminal-jarvis
```

## What Terminal Jarvis Does

Terminal Jarvis is your AI coding assistant command center:

- **Interactive T.JARVIS Interface**: Beautiful ASCII art terminal UI with responsive design
- **One-Click Tool Management**: Install, update, and run AI coding tools seamlessly

## Documentation

Comprehensive guides and references available at the external docs site:

**➤ [Terminal Jarvis Documentation](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/)**

Includes:
- Installation Guide - Platform-specific setup instructions
- Usage Guide - How to use Terminal Jarvis effectively
- Configuration Guide - Customize Terminal Jarvis behavior
- Known Limitations - Current issues and workarounds
- Architecture Guide - Technical details and development info
- Testing Guide - How to test and contribute
- Contribution Guide - Complete contributor guidelines
- Roadmap - Future plans and development priorities
- **[Supported AI Tools](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/quick_start/ai-tools/)** - Complete overview of all integrated AI coding tools

Terminal Jarvis provides a unified interface for multiple AI coding tools including Claude, Gemini, Qwen, and an expanding ecosystem of others. Each tool is carefully integrated with intelligent authentication flows and session management. For detailed tool information, capabilities, and current status, see our [comprehensive tool guide](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/quick_start/ai-tools/).

> [!CAUTION]
> **Known Issues**: [View current limitations and workarounds](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/details/limitations/)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support the Project

Help sustain development and unlock more tool integrations.

If Terminal Jarvis has been helpful for your AI coding workflow or you just thought the project is worth the maintainers going down this rabbit hole, consider supporting development:

[![Buy Me a Coffee](https://img.shields.io/badge/Buy%20Me%20a%20Coffee-orange.svg?style=for-the-badge)](https://www.buymeacoffee.com/brandoncalderonmorales)

Your support helps maintain and improve Terminal Jarvis for the entire community!


