<div align="center">

# Terminal Jarvis

**Unified command center for AI coding tools**

Manage Claude, Gemini, Qwen, and 8 more AI assistants from one terminal interface.

[![NPM Version](https://img.shields.io/npm/v/terminal-jarvis.svg?logo=npm&style=flat-square)](https://www.npmjs.com/package/terminal-jarvis)
[![Crates.io](https://img.shields.io/crates/v/terminal-jarvis.svg?logo=rust&style=flat-square)](https://crates.io/crates/terminal-jarvis)
[![Homebrew](https://img.shields.io/badge/Homebrew-Available-blue.svg?logo=homebrew&style=flat-square)](https://github.com/BA-CalderonMorales/homebrew-terminal-jarvis)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](https://opensource.org/licenses/MIT)
[![Mentioned in Awesome](https://img.shields.io/badge/Mentioned%20in-Awesome-6f42c1?style=flat-square)](https://github.com/Piebald-AI/awesome-gemini-cli)

<img src="https://raw.githubusercontent.com/BA-CalderonMorales/terminal-jarvis/docs/screenshots_and_demos/screenshots_and_demo/promo_image_for_readme.png" alt="Terminal Jarvis Interface" width="100%">

</div>

---

> **Safe Testing Recommended**: Terminal Jarvis is a harness for AI coding tools that can modify files and execute commands. For the safest experience, we recommend testing in a remote development environment such as [GitHub Codespaces](https://github.com/codespaces), [Coder](https://coder.com/), [DevPod](https://devpod.sh/), or [Google Colab](https://colab.research.google.com/). These environments provide isolation from your local machine while offering full development capabilities.

---

## Quick Start

```bash
# Try instantly (no install)
npx terminal-jarvis

# Or install globally
npm install -g terminal-jarvis    # NPM
cargo install terminal-jarvis     # Cargo
brew tap ba-calderonmorales/terminal-jarvis && brew install terminal-jarvis  # Homebrew
```

## What It Does

| Feature | Description |
|:--------|:------------|
| **Interactive Interface** | Beautiful terminal UI with ASCII art, themed menus, and keyboard navigation for a polished command-line experience. |
| **11 AI Tools Supported** | Claude, Gemini, Qwen, OpenCode, Codex, Aider, Goose, Amp, Crush, LLXPRT, and GitHub Copilot CLI - all manageable from a single interface. |
| **Integrated Installation** | Install, update, or uninstall any supported AI tool directly from the menu without leaving the terminal. |
| **Session Continuity** | Preserves your terminal session state during browser-based authentication flows. Currently in development with expanding coverage. |
| **Comparative Evaluation** | Built-in framework for running evaluations across different AI tools. Currently in development with expanding coverage. |

<p align="center">
<img src="https://raw.githubusercontent.com/BA-CalderonMorales/terminal-jarvis/docs/screenshots_and_demos/screenshots_and_demo/Terminal%20Jarvis%20Demo.gif" alt="Demo" width="100%">
</p>

## Documentation

Full guides at **[Terminal Jarvis Docs](https://ba-calderonmorales.github.io/my-life-as-a-dev/latest/projects/active/terminal-jarvis/)**

| Guide | Description |
|:------|:------------|
| [Installation](https://ba-calderonmorales.github.io/my-life-as-a-dev/latest/projects/active/terminal-jarvis/quick_start/installation/) | Step-by-step platform setup for NPM, Cargo, and Homebrew with troubleshooting tips for common issues. |
| [AI Tools](https://ba-calderonmorales.github.io/my-life-as-a-dev/latest/projects/active/terminal-jarvis/quick_start/ai-tools/) | Detailed overview of all 11 supported AI coding assistants including authentication requirements and capabilities. |
| [Configuration](https://ba-calderonmorales.github.io/my-life-as-a-dev/latest/projects/active/terminal-jarvis/quick_start/configuration/) | Customize themes, keybindings, default tools, and environment variables to match your workflow. |
| [Architecture](https://ba-calderonmorales.github.io/my-life-as-a-dev/latest/projects/active/terminal-jarvis/details/architecture/) | Technical deep-dive into the Rust codebase, module organization, and design decisions. |

## Project Structure

<details open>
<summary><strong>Expand/Collapse</strong></summary>

```
terminal-jarvis/
├── src/                           # Rust application
│   ├── main.rs                    # Entry point
│   ├── cli.rs                     # CLI definitions
│   ├── cli_logic/                 # Business logic (22 modules)
│   ├── auth_manager/              # Authentication (8 modules)
│   ├── config/                    # Configuration (6 modules)
│   ├── services/                  # External integrations (6 modules)
│   ├── tools/                     # Tool management (14 modules)
│   ├── theme/                     # UI theming (9 modules)
│   └── api/                       # API framework (4 modules)
│
├── config/                        # Configuration files
│   ├── tools/                     # Per-tool configs (*.toml)
│   ├── evals/                     # Evaluation metrics
│   └── *.toml                     # Global settings
│
├── scripts/                       # Automation
│   ├── cicd/                      # CI/CD (local-ci.sh, local-cd.sh)
│   └── verify/                    # Verification feedback loop
│
├── .github/                       # GitHub integrations
│   └── skills/                    # AI agent skills (17 modules)
│       ├── verification/          # Quality verification
│       ├── release-checklist/     # Pre-release automation
│       ├── qa-testing/            # Minimal QA branch testing
│       ├── deployment/            # Release workflows
│       └── ...                    # 13 more skills
│
├── tests/                         # Rust tests (cargo test)
├── e2e/                           # E2E tests (TypeScript/Vitest)
├── npm/terminal-jarvis/           # NPM wrapper
└── homebrew/                      # Homebrew Formula
```

</details>

## Development

### Remote Development Environments (Recommended)

For the safest and most consistent development experience, use a cloud-based environment:

| Environment | Description |
|:------------|:------------|
| [GitHub Codespaces](https://github.com/codespaces/new?template_repository=BA-CalderonMorales/terminal-jarvis) | Zero-setup cloud development with VS Code integration. Pre-configured with all dependencies. |
| [Coder](https://coder.com/) | Self-hosted or cloud workspaces with full IDE support. Great for teams with custom infrastructure. |
| [DevPod](https://devpod.sh/) | Open-source, client-only solution that works with any cloud provider or local Docker. |
| [Google Colab](https://colab.research.google.com/) | Free cloud notebooks with terminal access. Useful for quick experimentation. |

### Local Development

**Prerequisites**: Node.js 20+, Rust toolchain (for source builds)

### Verification

```bash
# Run before commits - comprehensive quality check
./scripts/verify/verify-change.sh

# Individual checks for faster iteration
./scripts/verify/verify-build.sh      # Compilation only
./scripts/verify/verify-quality.sh    # Clippy + formatting
./scripts/verify/verify-tests.sh      # Unit + integration tests
./scripts/verify/verify-cli.sh        # CLI smoke tests
```

### Testing

```bash
cargo test              # Rust unit and integration tests
cd e2e && npm test      # End-to-end tests with Vitest
```

## Contributing

See [AGENTS.md](AGENTS.md) for AI-assisted development guidelines and the [Contribution Guide](https://ba-calderonmorales.github.io/my-life-as-a-dev/latest/projects/active/terminal-jarvis/details/contributions/).

## License

MIT - see [LICENSE](LICENSE)

---

<div align="center">

**[Documentation](https://ba-calderonmorales.github.io/my-life-as-a-dev/latest/projects/active/terminal-jarvis/)** |
**[Issues](https://github.com/BA-CalderonMorales/terminal-jarvis/issues)** |
**[Changelog](CHANGELOG.md)**

[![Buy Me a Coffee](https://img.shields.io/badge/Support-Buy%20Me%20a%20Coffee-orange.svg?style=flat-square)](https://www.buymeacoffee.com/brandoncalderonmorales)

</div>
