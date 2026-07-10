<div align="center">

# Terminal Jarvis

**Unified command center for AI coding tools**

Manage Claude, Gemini, Qwen, and 22 more AI assistants from one terminal
interface.

[![NPM Version](https://img.shields.io/npm/v/terminal-jarvis.svg?logo=npm&style=flat-square)](https://www.npmjs.com/package/terminal-jarvis)
[![Crates.io](https://img.shields.io/crates/v/terminal-jarvis.svg?logo=rust&style=flat-square)](https://crates.io/crates/terminal-jarvis)
[![Homebrew](https://img.shields.io/badge/Homebrew-Available-blue.svg?logo=homebrew&style=flat-square)](https://github.com/BA-CalderonMorales/homebrew-terminal-jarvis)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](https://opensource.org/licenses/MIT)
[![Mentioned in Awesome](https://img.shields.io/badge/Mentioned%20in-Awesome-6f42c1?style=flat-square)](https://github.com/Piebald-AI/awesome-gemini-cli)
[![Docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://ba-calderonmorales.github.io/my-life-as-a-dev/latest/projects/active/terminal-jarvis/)
[![Coverage](https://img.shields.io/badge/coverage-report-green.svg?style=flat-square)](https://github.com/BA-CalderonMorales/terminal-jarvis/actions/workflows/ci.yml?query=branch%3Adevelop)

<img src="https://raw.githubusercontent.com/BA-CalderonMorales/terminal-jarvis/main/docs/promo-image.png" alt="Terminal Jarvis Interface" width="100%">

</div>

---

> **Safe Testing Recommended**: Terminal Jarvis is a harness for AI coding tools
> that can modify files and execute commands. For the safest experience, test in
> a remote development environment such as
> [GitHub Codespaces](https://github.com/codespaces),
> [Coder](https://coder.com/), [DevPod](https://devpod.sh/), or
> [Google Colab](https://colab.research.google.com/).

---

A data-driven harness switcher for AI coding agents. Maps **25 coding-agent
CLIs** through a shared **9-capability contract** -- one interface to
download, run, update, and inspect any agent tool.

## Install

```bash
# Cargo
cargo install terminal-jarvis

# npm
npm install -g terminal-jarvis

# Homebrew
brew install BA-CalderonMorales/homebrew-terminal-jarvis/terminal-jarvis
```

Cargo builds the Rust CLI from the crates.io source package. The npm package is
a Node launcher that downloads the matching Terminal Jarvis GitHub Release
asset, verifies its `.sha256` file, caches it, and then executes it. Homebrew
installs the matching platform release archive from the tap.

Supported prebuilt assets are `linux-x64-gnu`, `linux-arm64-gnu`,
`macos-x64`, `macos-arm64`, and `win32-x64`. Native Windows npm installs use
the `win32-x64` ZIP bundle and work from Command Prompt, PowerShell, or Git
Bash. Every release also includes a direct native executable for each platform;
downloaded Linux and macOS executables may need `chmod +x` before use.

An older Cargo or manual install can still win `PATH` resolution after a global
npm upgrade. The npm install now completes and prints the path-order fix rather
than blocking the upgrade; place the npm prefix before the stale location to run
the newly installed command.

## Quick Start

```bash
# List every coding agent
terminal-jarvis list

# Inspect a harness
terminal-jarvis show opencode

# Preview a capability command
terminal-jarvis plan codex headless

# Select and verify the active harness
terminal-jarvis use opencode
terminal-jarvis current
terminal-jarvis check

# Optional: block harness commands until Trivy clears this workspace
terminal-jarvis gate enable trivy
terminal-jarvis gate status
```

For development builds, replace `terminal-jarvis` with `cargo run --`.

### Layout

```text
harnesses/<agent>/
├── index.toml              # name, display, binary, env requirements
├── download/index.toml     # install without sudo
├── update/index.toml       # upgrade without interactive auth
├── headless/index.toml     # non-interactive command mode
├── version/index.toml      # print installed agent version
├── stats/index.toml        # local agent statistics
├── models/index.toml       # list available models
├── security/index.toml     # sandbox and approval settings
├── ui/index.toml           # interactive terminal UI
└── yolo/index.toml         # bypass safeguards (dangerous)
```

Auth stays with each harness -- terminal-jarvis never retains credentials.

## Commands

| Command | Purpose |
|---|---|
| `list` | Show all coding agents |
| `show <harness>` | Inspect a harness's capabilities |
| `use <harness>` / `current` | Select / show active harness |
| `plan [harness] <capability>` | Preview the shell command |
| `run [harness] [capability] [args...]` | Execute a capability |
| `check` | Report binary + env readiness |
| `security [status\|audit\|harness]` | Security posture |
| `gate [status\|list\|enable\|disable\|run]` | Optional local security gate |
| `version [--verbose]` / `--version` / `-v` / `--info` | Version info |
| `--update [--dry-run]` | Update Terminal Jarvis or print the update command |
| `config show` | Active config state |
| `auth help <harness>` | Credential setup guidance |
| `[harness] [args...]` | Pass-through to harness binary |

Legacy aliases remain available: `tools -> list`, `status -> check`,
`info <harness> -> show <harness>`, `install <harness> -> run <harness> download`, and `update <harness> -> run <harness> update`.

Human-facing commands use width-aware structured output and color only on an
interactive terminal. For scripts, put `--plain` before the command for stable
line-oriented output; `--no-color` keeps the structured layout without color.

The experimental dashboard is intentionally behind a feature wall and remains
noninteractive:

```bash
TERMINAL_JARVIS_EXPERIMENTAL_UI=1 terminal-jarvis experimental dashboard
```

## Docs

| Document | What |
|---|---|
| [Capability contract](docs/harness-capability-contract.md) | Full breakdown of the 9 capabilities |
| [Supported agents](docs/supported-agents.md) | All 25 coding agents |
| [Security gates](docs/security-gates.md) | Optional Trivy scan behavior and configuration |
| [Development](docs/development.md) | Architecture, verification, and release artifacts |
