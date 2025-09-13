<div align="center">

# Terminal Jarvis

</div>

<table border="0">
<tr>
<td width="60%">

<img src="https://raw.githubusercontent.com/BA-CalderonMorales/terminal-jarvis/docs/screenshots_and_demos/screenshots_and_demo/promo_image_for_readme.png" alt="Terminal Jarvis Interface" width="100%">

</td>
<td width="40%" align="center">

<!-- NPM Package -->
[![NPM Stable](https://img.shields.io/npm/v/terminal-jarvis/stable.svg?label=NPM%20Stable&color=green&logo=npm)](https://www.npmjs.com/package/terminal-jarvis)
[![NPM Beta](https://img.shields.io/npm/v/terminal-jarvis/beta.svg?label=NPM%20Beta&color=orange&logo=npm)](https://www.npmjs.com/package/terminal-jarvis)
[![NPM Version](https://img.shields.io/npm/v/terminal-jarvis.svg?logo=npm&label=NPM%20Version)](https://www.npmjs.com/package/terminal-jarvis)
[![NPM Downloads](https://img.shields.io/npm/dm/terminal-jarvis.svg?logo=npm&label=NPM%20Downloads)](https://www.npmjs.com/package/terminal-jarvis)

<!-- Rust Crate -->

[![Crates.io Version](https://img.shields.io/crates/v/terminal-jarvis.svg?logo=rust&label=Crates.io%20Version)](https://crates.io/crates/terminal-jarvis)
[![Crates.io Downloads](https://img.shields.io/crates/d/terminal-jarvis.svg?logo=rust&label=Crates.io%20Downloads)](https://crates.io/crates/terminal-jarvis)

<!-- Homebrew -->

[![Homebrew](https://img.shields.io/badge/Homebrew-Available-blue.svg?logo=homebrew)](https://github.com/BA-CalderonMorales/homebrew-terminal-jarvis)

<!-- General -->

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Mentioned in Awesome Gemini CLI](https://awesome.re/mentioned-badge.svg)](https://github.com/Piebald-AI/awesome-gemini-cli)
[![Buy Me a Coffee](https://img.shields.io/badge/☕-Buy%20Me%20a%20Coffee-orange.svg)](https://www.buymeacoffee.com/brandoncalderonmorales)

</td>
</tr>
</table>

<div align="center">

A unified command center for AI coding tools. Manage and run claude-code, gemini-cli, qwen-code, opencode, llxprt, codex, and crush from one beautiful terminal interface.

</div>

<div align="center">

<img src="https://raw.githubusercontent.com/BA-CalderonMorales/terminal-jarvis/docs/screenshots_and_demos/screenshots_and_demo/Terminal Jarvis v0.0.67 Demo.gif" alt="Terminal Jarvis Demo" width="85%" style="border-radius: 6px;">

</div>

## Prerequisites

### **Recommended: Remote Development Environment (Zero Setup)**

The optimal way to use Terminal Jarvis is through a pre-configured remote development environment:

- **[Open in GitHub Codespaces](https://github.com/codespaces/new?template_repository=BA-CalderonMorales/terminal-jarvis)** - Instant, cloud-based development environment
- **[Use VS Code Dev Containers](https://code.visualstudio.com/docs/remote/containers)** - Local containerized environment

**Why this approach is ideal:**
- **Zero Setup Time**: Complete development environment ready in ~60 seconds
- **Consistent Environment**: Same setup across all contributors and platforms
- **Pre-installed Tools**: Rust 1.87, Node.js 20, GitHub CLI, Git LFS, LLDB debugger
- **VS Code Extensions**: GitHub Copilot, Rust debugging, TOML support pre-configured
- **Optimized Settings**: File watching excludes, format-on-save, search optimization
- **AI-Assisted Development**: GitHub Copilot pre-configured
- **All Dependencies Ready**: No manual installation of compilers or tools

### **Alternative: Local Installation**

If you prefer local development:

- **Node.js 20+** and NPM
- **macOS users**: [Rust toolchain required](docs/INSTALLATION.md#macos-prerequisites)
- **Linux users**: Build tools and development headers
- **Windows users**: Windows Subsystem for Linux (WSL2) recommended

> [!IMPORTANT]
> **Full installation guide:** [docs/INSTALLATION.md](docs/INSTALLATION.md)

## Quick Start

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

- **[Installation Guide](docs/INSTALLATION.md)** - Platform-specific setup instructions
- **[Usage Guide](docs/USAGE.md)** - How to use Terminal Jarvis effectively
- **[Configuration Guide](docs/CONFIGURATION.md)** - Customize Terminal Jarvis behavior
- **[Known Limitations](docs/LIMITATIONS.md)** - Current issues and workarounds
- **[Architecture Guide](docs/ARCHITECTURE.md)** - Technical details and development info
- **[Testing Guide](docs/TESTING.md)** - How to test and contribute
- **[Contribution Guide](docs/CONTRIBUTIONS.md)** - Complete contributor guidelines
- **[Roadmap](docs/ROADMAP.md)** - Future plans and development priorities
- **[Supported AI Tools](docs/SOURCES.md)** - Complete overview of all integrated AI coding tools

Terminal Jarvis provides a unified interface for multiple AI coding tools including Claude, Gemini, Qwen, and an expanding ecosystem of others. Each tool is carefully integrated with intelligent authentication flows and session management. For detailed tool information, capabilities, and current status, see our [comprehensive tool guide](docs/SOURCES.md).

> [!CAUTION]
> **Known Issues**: [View current limitations and workarounds](docs/LIMITATIONS.md)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support the Project

If Terminal Jarvis has been helpful for your AI coding workflow or you just thought the project is worth the maintainers going down this rabbit hole, consider supporting development:

[![Buy Me a Coffee](https://img.shields.io/badge/☕-Buy%20Me%20a%20Coffee-orange.svg?style=for-the-badge)](https://www.buymeacoffee.com/brandoncalderonmorales)

Your support helps maintain and improve Terminal Jarvis for the entire community!
