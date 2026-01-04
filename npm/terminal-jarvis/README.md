<div align="center">

# Terminal Jarvis

**Unified command center for AI coding tools**

Manage Claude, Gemini, Qwen, and 7 more AI assistants from one terminal interface.

[![NPM Version](https://img.shields.io/npm/v/terminal-jarvis.svg?logo=npm&style=flat-square)](https://www.npmjs.com/package/terminal-jarvis)
[![Crates.io](https://img.shields.io/crates/v/terminal-jarvis.svg?logo=rust&style=flat-square)](https://crates.io/crates/terminal-jarvis)
[![Homebrew](https://img.shields.io/badge/Homebrew-Available-blue.svg?logo=homebrew&style=flat-square)](https://github.com/BA-CalderonMorales/homebrew-terminal-jarvis)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](https://opensource.org/licenses/MIT)

<img src="https://raw.githubusercontent.com/BA-CalderonMorales/terminal-jarvis/docs/screenshots_and_demos/screenshots_and_demo/promo_image_for_readme.png" alt="Terminal Jarvis Interface" width="600">

</div>

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

- **Interactive Interface** - Beautiful terminal UI with ASCII art
- **10 AI Tools** - Claude, Gemini, Qwen, OpenCode, Codex, Aider, Goose, Amp, Crush, LLXPRT
- **One-Click Install** - Install any tool directly from the menu
- **Session Continuity** - Maintains state during browser auth flows

<p align="center">
<img src="https://raw.githubusercontent.com/BA-CalderonMorales/terminal-jarvis/docs/screenshots_and_demos/screenshots_and_demo/Terminal%20Jarvis%20Demo.gif" alt="Demo" width="700">
</p>

## Documentation

Full guides at **[Terminal Jarvis Docs](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/)**

| Guide | Description |
|-------|-------------|
| [Installation](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/quick_start/installation/) | Platform setup |
| [AI Tools](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/quick_start/ai-tools/) | Supported tools |
| [Configuration](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/quick_start/configuration/) | Customization |
| [Architecture](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/details/architecture/) | Technical details |

## Project Structure

<details>
<summary><strong>Click to expand</strong></summary>

```
terminal-jarvis/
├── src/                           # Rust application
│   ├── main.rs                    # Entry point
│   ├── cli.rs                     # CLI definitions
│   ├── cli_logic/                 # Business logic (9 modules)
│   ├── auth_manager/              # Authentication (5 modules)
│   ├── config/                    # Configuration (5 modules)
│   ├── services/                  # External integrations (5 modules)
│   ├── tools/                     # Tool management (6 modules)
│   ├── theme/                     # UI theming (7 modules)
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
│   └── skills/                    # AI agent skills (12 modules)
│       ├── verification/          # Quality verification
│       ├── deployment/            # Release workflows
│       ├── testing/               # TDD practices
│       └── ...                    # 9 more skills
│
├── tests/                         # Rust tests (cargo test)
├── e2e/                           # E2E tests (TypeScript/Vitest)
├── npm/terminal-jarvis/           # NPM wrapper
└── homebrew/                      # Homebrew Formula
```

</details>

## Development

### Prerequisites

**Recommended**: Use [GitHub Codespaces](https://github.com/codespaces/new?template_repository=BA-CalderonMorales/terminal-jarvis) for zero-setup development.

**Local**: Node.js 20+, Rust (for source builds)

### Verification

```bash
# Run before commits
./scripts/verify/verify-change.sh

# Individual checks
./scripts/verify/verify-build.sh      # Compilation
./scripts/verify/verify-quality.sh    # Clippy + fmt
./scripts/verify/verify-tests.sh      # Tests
./scripts/verify/verify-cli.sh        # CLI smoke tests
```

### Testing

```bash
cargo test              # Rust tests
cd e2e && npm test      # E2E tests
```

## Contributing

See [AGENTS.md](AGENTS.md) for AI-assisted development guidelines and the [Contribution Guide](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/details/contributions/).

## License

MIT - see [LICENSE](LICENSE)

<div align="center">

**[Documentation](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/)** |
**[Issues](https://github.com/BA-CalderonMorales/terminal-jarvis/issues)** |
**[Changelog](CHANGELOG.md)**

[![Buy Me a Coffee](https://img.shields.io/badge/Support-Buy%20Me%20a%20Coffee-orange.svg?style=flat-square)](https://www.buymeacoffee.com/brandoncalderonmorales)

</div>
