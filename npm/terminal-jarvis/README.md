<div align="center">

# Terminal Jarvis

<!-- NPM Package -->

[![NPM Version](https://img.shields.io/npm/v/terminal-jarvis.svg?logo=npm&label=NPM%20Version)](https://www.npmjs.com/package/terminal-jarvis)
[![NPM Downloads](https://img.shields.io/npm/dm/terminal-jarvis.svg?logo=npm&label=NPM%20Downloads)](https://www.npmjs.com/package/terminal-jarvis)
[![NPM Stable](https://img.shields.io/npm/v/terminal-jarvis/stable.svg?label=NPM%20Stable&color=green&logo=npm)](https://www.npmjs.com/package/terminal-jarvis)
[![NPM Beta](https://img.shields.io/npm/v/terminal-jarvis/beta.svg?label=NPM%20Beta&color=orange&logo=npm)](https://www.npmjs.com/package/terminal-jarvis)

<!-- Rust Crate -->

[![Crates.io Version](https://img.shields.io/crates/v/terminal-jarvis.svg?logo=rust&label=Crates.io%20Version)](https://crates.io/crates/terminal-jarvis)
[![Crates.io Downloads](https://img.shields.io/crates/d/terminal-jarvis.svg?logo=rust&label=Crates.io%20Downloads)](https://crates.io/crates/terminal-jarvis)

<!-- Homebrew -->

[![Homebrew](https://img.shields.io/badge/Homebrew-Available-blue.svg?logo=homebrew)](https://github.com/BA-CalderonMorales/homebrew-terminal-jarvis)

<!-- General -->

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Mentioned in Awesome Gemini CLI](https://awesome.re/mentioned-badge.svg)](https://github.com/Piebald-AI/awesome-gemini-cli)
[![Buy Me a Coffee](https://img.shields.io/badge/☕-Buy%20Me%20a%20Coffee-orange.svg)](https://www.buymeacoffee.com/brandoncalderonmorales)

</div>

A unified command center for AI coding tools. Manage and run claude-code, gemini-cli, qwen-code, opencode, llxprt, codex, and crush from one beautiful terminal interface.

<img src="screenshots/promo_image_for_readme.png" alt="Terminal Jarvis Interface" width="100%">

## Insights

**[View all 7 supported AI tools →](docs/SOURCES.md)**

**Try it instantly:** `npx terminal-jarvis`

**Known Issues:** [View current limitations and workarounds](docs/LIMITATIONS.md)

**Security:** [Comprehensive security audit report](docs/SECURITY_REPORT.md) - Zero exploitable vulnerabilities identified
- *Security review conducted with the aid of claude-code /security-review*

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

**Prerequisites:**

- Node.js and NPM
- **macOS users**: [Rust toolchain required](docs/INSTALLATION.md#macos-prerequisites)

**Full installation guide:** [docs/INSTALLATION.md](docs/INSTALLATION.md)

## What Terminal Jarvis Does

Terminal Jarvis is your AI coding assistant command center:

- **Interactive T.JARVIS Interface**: Beautiful ASCII art terminal UI with responsive design
- **One-Click Tool Management**: Install, update, and run AI coding tools seamlessly
- **Supported Tools**:
  - `claude` - Anthropic's Claude for code assistance
  - `gemini` - Google's Gemini CLI tool
  - `qwen` - Qwen coding assistant
  - `opencode` - Terminal-based AI coding agent (Testing)
  - `llxprt` - Multi-provider AI coding assistant (Testing)
  - `codex` - OpenAI Codex CLI for local AI coding (Testing)
- **Smart Status Detection**: See which tools are installed and ready to use
- **Interactive Workflows**: Guided prompts for running tools with the right arguments

**Testing Phase**: Looking for testers! opencode, llxprt, and codex are new additions.

## How to Use Terminal Jarvis

### Interactive Mode (Recommended)

```bash
# Launch the full T.JARVIS experience
terminal-jarvis
```

Get the complete interface with:

- Beautiful ASCII art welcome screen
- Real-time tool status dashboard
- Quick tool selection and launching
- Built-in management options
- Smart guidance and tips

### Direct Commands

```bash
# Install and manage tools
terminal-jarvis install claude
terminal-jarvis update               # Update all tools
terminal-jarvis list                # Show tool status
terminal-jarvis info claude         # Tool details

# Run tools directly
terminal-jarvis run claude --prompt "Refactor this function"
terminal-jarvis run gemini --file src/main.rs
terminal-jarvis run qwen --analyze
terminal-jarvis run opencode --generate
terminal-jarvis run llxprt --help
```

### Template Management

```bash
# Template workflow (requires gh CLI)
terminal-jarvis templates init       # Setup templates repo
terminal-jarvis templates create my-template
terminal-jarvis templates list
terminal-jarvis templates apply my-template
```

## Supported AI Tools

| Tool       | Description                               | Status     | Installation Command                         |
| ---------- | ----------------------------------------- | ---------- | -------------------------------------------- |
| `claude`   | Anthropic's Claude for code assistance    | Stable     | `npm install -g @anthropic-ai/claude-code`   |
| `gemini`   | Google's Gemini CLI tool                  | Stable     | `npm install -g @google/gemini-cli`          |
| `qwen`     | Qwen coding assistant                     | Stable     | `npm install -g @qwen-code/qwen-code@latest` |
| `opencode` | Terminal-based AI coding agent            | Testing    | `npm install -g opencode-ai@latest`          |
| `llxprt`   | Multi-provider AI coding assistant        | Testing    | `npm install -g @vybestack/llxprt-code-core` |
| `codex`    | OpenAI Codex CLI for local AI coding      | Testing    | `npm install -g @openai/codex`               |
| `crush`    | Charm's multi-model AI assistant with LSP | New        | `npm install -g @charmland/crush`            |

\*See [limitations](docs/LIMITATIONS.md) for known issues and workarounds

## Configuration (Optional)

Terminal Jarvis works out-of-the-box, but you can customize behavior with configuration files:

**Locations** (in priority order):

- `./terminal-jarvis.toml` (project-specific)
- `~/.config/terminal-jarvis/config.toml` (user-wide)

**Example configuration:**

```toml
[tools]
claude = { enabled = true, auto_update = true }
gemini = { enabled = true, auto_update = false }
qwen = { enabled = true, auto_update = true }
opencode = { enabled = false, auto_update = false }
llxprt = { enabled = true, auto_update = true }
codex = { enabled = true, auto_update = true }
crush = { enabled = true, auto_update = true }

[templates]
repository = "your-username/jarvis-templates"
auto_sync = true
```

## Documentation

- **[Installation Guide](docs/INSTALLATION.md)** - Platform-specific setup instructions
- **[Security Report](docs/SECURITY_REPORT.md)** - Comprehensive security audit with zero vulnerabilities
- **[Known Limitations](docs/LIMITATIONS.md)** - Current issues and workarounds
- **[Architecture Guide](docs/ARCHITECTURE.md)** - Technical details and development info
- **[Testing Guide](docs/TESTING.md)** - How to test and contribute

## Contributing

We welcome contributions! Please join our [Discord community](https://discord.gg/zNuyC5uG) first to discuss your ideas.

**Quick Start:**

1. Join the [Terminal Jarvis Discord](https://discord.gg/zNuyC5uG)
2. Discuss your contribution in `#features` or `#bugfix` channels
3. Fork the repository
4. Create a feature branch (`git checkout -b feature/amazing-feature`)
5. Follow our [contribution guidelines](docs/CONTRIBUTIONS.md)
6. Ensure tests pass (`cargo test`)
7. Use our PR template for submitting changes

**Full Guide:** [docs/CONTRIBUTIONS.md](docs/CONTRIBUTIONS.md) - Complete contributor guide with coding standards, testing requirements, and development workflow.

**Technical Details:** [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) - Architecture overview and development setup.

## Support the Project

If Terminal Jarvis has been helpful for your AI coding workflow, consider supporting development:

[![Buy Me a Coffee](https://img.shields.io/badge/☕-Buy%20Me%20a%20Coffee-orange.svg?style=for-the-badge)](https://www.buymeacoffee.com/brandoncalderonmorales)

Your support helps maintain and improve Terminal Jarvis for the entire community!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [x] Interactive T.JARVIS Interface with ASCII art
- [x] Smart tool detection and status reporting
- [x] One-click installation with NPM validation
- [x] Responsive terminal design
- [x] Enhanced authentication flows (addressing current login issues)
- [ ] Improved wrapper layer stability
- [ ] Plugin system for custom tools
- [ ] Shell completion scripts
- [ ] Web dashboard for tool management
