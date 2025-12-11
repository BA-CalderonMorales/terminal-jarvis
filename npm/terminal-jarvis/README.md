<div align="center">

# Terminal Jarvis

</div>

<div align="center">

A unified command center for AI coding tools. Manage and run a suite of coding assistants from one beautiful terminal interface. See the full list in the [Supported AI Tools guide](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/quick_start/ai-tools/).

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

## Architecture Overview

The project follows a **domain-based modular architecture** designed for maintainability, extensibility, and clear separation of concerns.

<details>
<summary><strong>Click to expand full project structure</strong></summary>

```
src/
├── main.rs                    # Entry point - minimal code, delegates to CLI
├── lib.rs                     # Library entry point for module organization
├── cli.rs                     # Clean, expressive CLI interface definitions
├── installation_arguments.rs  # Installation commands and NPM validation
├── progress_utils.rs          # Theme-integrated messaging and progress indicators
│
├── cli_logic/                 # Business logic domain (9 focused modules)
│   ├── mod.rs                 # Module coordination and re-exports
│   ├── cli_logic_entry_point.rs        # Main CLI logic entry point
│   ├── cli_logic_interactive.rs        # Interactive T.JARVIS interface
│   ├── cli_logic_tool_execution.rs     # Tool execution workflows
│   ├── cli_logic_update_operations.rs  # Tool update management
│   ├── cli_logic_list_operations.rs    # Tool listing operations
│   ├── cli_logic_info_operations.rs    # Tool information display
│   ├── cli_logic_config_management.rs  # Configuration management
│   ├── cli_logic_template_operations.rs # Template system operations
│   └── cli_logic_utilities.rs          # Shared utility functions
│
├── auth_manager/              # Authentication domain (5 focused modules)
│   ├── mod.rs                 # Module coordination and re-exports
│   ├── auth_entry_point.rs            # Authentication system entry point
│   ├── auth_environment_detection.rs  # Environment detection logic
│   ├── auth_environment_setup.rs      # Environment configuration
│   ├── auth_api_key_management.rs     # API key handling
│   └── auth_warning_system.rs         # Browser prevention warnings
│
├── config/                    # Configuration domain (5 focused modules)
│   ├── mod.rs                 # Module coordination and re-exports
│   ├── config_entry_point.rs          # Configuration system entry point
│   ├── config_structures.rs           # TOML configuration structures
│   ├── config_file_operations.rs      # File I/O operations
│   ├── config_manager.rs              # Configuration management logic
│   └── config_version_cache.rs        # Version caching system
│
├── services/                  # External integrations domain (5 focused modules)
│   ├── mod.rs                 # Module coordination and re-exports
│   ├── services_entry_point.rs        # Service layer entry point
│   ├── services_package_operations.rs # Package management operations
│   ├── services_npm_operations.rs     # NPM-specific operations
│   ├── services_github_integration.rs # GitHub CLI integration
│   └── services_tool_configuration.rs # Tool configuration mapping
│
├── theme/                     # UI theming domain (7 focused modules)
│   ├── mod.rs                 # Module coordination and re-exports
│   ├── theme_entry_point.rs           # Theme system entry point
│   ├── theme_definitions.rs           # Theme color definitions
│   ├── theme_config.rs                # Theme configuration management
│   ├── theme_global_config.rs         # Global theme state
│   ├── theme_background_layout.rs     # Background and layout styling
│   ├── theme_text_formatting.rs       # Text formatting utilities
│   └── theme_utilities.rs             # Theme utility functions
│
├── tools/                     # Tool management domain (6 focused modules)
│   ├── mod.rs                 # Module coordination and re-exports
│   ├── tools_entry_point.rs           # Tool system entry point
│   ├── tools_detection.rs             # Tool detection and verification
│   ├── tools_command_mapping.rs       # Command-to-tool mapping
│   ├── tools_execution_engine.rs      # Tool execution engine
│   ├── tools_process_management.rs    # Process lifecycle management
│   └── tools_startup_guidance.rs      # Tool startup guidance system
│
└── api/                       # API framework domain (4 focused modules)
    ├── mod.rs                 # Module coordination and re-exports
    ├── api_base.rs            # Base API route configurations
    ├── api_client.rs          # HTTP client abstraction layer
    └── api_tool_operations.rs # API tool operation endpoints

tests/                         # Rust integration tests (cargo test)
e2e/                           # TypeScript E2E tests (cli-testing-library + Vitest)
├── *.test.ts                  # E2E test suites
├── helpers.ts                 # CLI test utilities
├── helpers/                   # Advanced test utilities
│   ├── ansi-utils.ts          # ANSI escape code parsing
│   ├── layout-validators.ts   # Terminal layout validation
│   ├── width-simulation.ts    # Responsive width testing
│   └── benchmark-helpers.ts   # Benchmark result validation
├── vitest.config.ts           # Test runner configuration
└── package.json               # E2E test dependencies

config/
├── tools/                     # Modular tool configurations
│   ├── claude.toml            # Anthropic Claude
│   ├── gemini.toml            # Google Gemini
│   ├── qwen.toml              # Qwen coding assistant
│   ├── opencode.toml, llxprt.toml, codex.toml
│   ├── crush.toml, goose.toml, amp.toml, aider.toml
└── config.toml                # Global settings

npm/terminal-jarvis/           # NPM distribution wrapper
homebrew/                      # Homebrew Formula + release archives
scripts/cicd/                  # Deployment automation
```

</details>

### Design Principles

1. **Domain-Driven Design**: Each module represents a distinct business domain
2. **Single Responsibility**: Modules handle one aspect of functionality
3. **Clear Dependencies**: Explicit imports, minimal coupling
4. **Testability**: Pure functions, dependency injection patterns
5. **Maintainability**: Small, focused files (~200-300 lines each)

### Testing Strategy

| Location | Technology | Purpose |
|----------|------------|---------|
| `tests/` | Rust (`cargo test`) | Unit and integration tests for Rust code |
| `e2e/` | TypeScript (Vitest + cli-testing-library) | End-to-end CLI behavior tests |

**Run tests:**
```bash
cargo test                     # Rust tests
cd e2e && npm test             # E2E tests (requires npm install first)
```

### Key Goal: Session Continuation System

Terminal Jarvis aims to provide a **Session Continuation System** that prevents users from being kicked out during authentication workflows. When a tool requires browser-based auth, Terminal Jarvis will maintain your session state and resume seamlessly.

> [!NOTE]
> **Work in Progress**: This feature is under active development. Current behavior may require manual re-authentication in some scenarios. See [#27](https://github.com/BA-CalderonMorales/terminal-jarvis/issues/27) for status.

For complete technical documentation, see the [Architecture Guide](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/details/architecture/).

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

Comprehensive guides and references available at the external docs site: **[Terminal Jarvis Documentation](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/)**

Includes:
- **[Installation Guide](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/quick_start/installation/)** - Platform-specific setup instructions
- **[Usage Guide](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/quick_start/usage/)** - How to use Terminal Jarvis effectively
- **[Configuration Guide](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/quick_start/configuration/)** - Customize Terminal Jarvis behavior
- **[Supported AI Tools](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/quick_start/ai-tools/)** - Complete overview of all integrated AI coding tools
- **[Architecture Guide](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/details/architecture/)** - Technical details and development info
- **[Testing Guide](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/details/testing/)** - How to test and contribute
- **[Contribution Guide](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/details/contributions/)** - Complete contributor guidelines
- **[Roadmap](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/details/roadmap/)** - Future plans and development priorities
- **[Known Limitations](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/details/limitations/)** - Current issues and workarounds

Terminal Jarvis provides a unified interface for multiple AI coding tools including Claude, Gemini, Qwen, and an expanding ecosystem of others. Each tool is carefully integrated with intelligent authentication flows and session management. For detailed tool information, capabilities, and current status, see our [comprehensive tool guide](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/quick_start/ai-tools/).

> [!CAUTION]
> **Known Issues**: [View current limitations and workarounds](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/details/limitations/)

## Beta Experiments

Features under exploration that are **not enabled by default**. These require explicit feature flags and may have stability issues.

### Voice Control (Deferred)

Terminal Jarvis includes experimental voice recognition using OpenAI's Whisper model for 100% offline voice commands.

**Status**: Deferred until core CLI stability is achieved. The voice module exists in the codebase (`src/voice/`) but is behind a feature flag.

**What it aims to do**:
- Hands-free tool switching ("switch to claude", "run gemini")
- Voice-activated commands without leaving the terminal
- 100% offline processing - no API keys, no data leaves your device

**Why it's deferred**:
- Core CLI tool execution needs to be seamless first (see [#26](https://github.com/BA-CalderonMorales/terminal-jarvis/issues/26))
- Authentication flows need stabilization (see [#27](https://github.com/BA-CalderonMorales/terminal-jarvis/issues/27))
- Adding complexity before fundamentals are solid creates maintenance burden

**To experiment** (not recommended for production):
```bash
cargo build --release --features local-voice
```

Voice will be revisited once the core experience is polished and stable.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support the Project

Help sustain development and unlock more tool integrations.

If Terminal Jarvis has been helpful for your AI coding workflow or you just thought the project is worth the maintainers going down this rabbit hole, consider supporting development:

[![Buy Me a Coffee](https://img.shields.io/badge/Buy%20Me%20a%20Coffee-orange.svg?style=for-the-badge)](https://www.buymeacoffee.com/brandoncalderonmorales)

Your support helps maintain and improve Terminal Jarvis for the entire community!


