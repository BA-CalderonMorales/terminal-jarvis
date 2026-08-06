<div align="center">

# Terminal Jarvis

**Catalog-driven command center for AI coding tools**

Inspect and safely gate catalog descriptors for Claude, Gemini, Qwen, and 22
other AI assistants from one terminal interface. Catalog presence does not mean
that a capability is supported; each capability reports its own evidence-backed
state.

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

A data-driven harness catalog for AI coding agents. It maps **25 coding-agent
CLIs** through a shared **9-capability contract** and fails closed unless a
capability's declared support, evidence, platform, and freshness permit it.

## Install

```bash
# Cargo
cargo install terminal-jarvis
# Cargo binary name is terminal-jarvis; add an alias if you want a short call:
#   echo 'alias tj=terminal-jarvis' >> ~/.bashrc

# npm
npm install -g terminal-jarvis
# The npm package also installs a tj shim, so you can call either:
#   terminal-jarvis list   # or  tj list

# Homebrew
brew install BA-CalderonMorales/homebrew-terminal-jarvis/terminal-jarvis
```

Cargo builds the Rust CLI from the crates.io source package. The npm package is
a Node launcher that downloads the matching Terminal Jarvis GitHub Release
asset, verifies its `.sha256` file, caches it, and then executes it. Homebrew
installs the matching platform release archive from the tap. The npm install
links both `terminal-jarvis` and the shorter `tj` entry point to the same
launcher, so either command name works; `tj` keeps daily use fast.

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
# Open the interactive switcher (bare tj on a terminal does the same)
terminal-jarvis tui

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

## Two ways to run

| Mode | Invocation | Best for |
|---|---|---|
| **Headless** | `terminal-jarvis <command>` / `tj <command>` | Scripts, CI, one-shot tasks |
| **Interactive** | `tj tui` (bare `tj` on a terminal) | Switching between coding agents live |

Headless runs one command per invocation, prints stable line-oriented output
with `--plain`, and never opens a prompt. The interactive tui is a chat-style
switcher: `list` shows the numbered picker, a number or an agent name switches
instantly, `status` shows readiness, `home` resets the frame, and `exit`
leaves. A slashed name (`/codex`) launches an agent behind the same guarded
plan-plus-confirm as headless. Ctrl+C aimed at a running agent never kills the
switcher.

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
| `self-update [--dry-run]` / `--update` | Update Terminal Jarvis or print the update command |
| `config show` | Active config state |
| `auth help <harness>` | Credential setup guidance |
| `[harness] [args...]` | Pass-through to harness binary |

Legacy aliases remain available: `tools -> list`, `status -> check`,
`info <harness> -> show <harness>`, `install <harness> -> run <harness> download`, and `update <harness> -> run <harness> update`.

Human-facing commands use width-aware structured output and color only on an
interactive terminal. For scripts, put `--plain` before the command for stable
line-oriented output; `--no-color` keeps the structured layout without color.

The interactive switcher (`terminal-jarvis tui`) doubles as the live dashboard
-- readiness, active harness, and the numbered picker -- and every slash
command runs the same guarded surface as headless automation.

## Interactive demo

The switcher, recorded with [VHS](https://github.com/charmbracelet/vhs) from
`scripts/demo/tui.tape`:

![The switcher in action](docs/demo-tui.gif)

The demo above walks the switcher's surface: boot the tui, browse the numbered
picker, switch by number, return to the welcome frame, and read the readiness
dashboard. `scripts/demo/tui.tape` carries the full story -- launch Codex and
hand over the terminal, exit Codex cleanly, launch Pi and chat with it, exit
Pi, then leave the switcher. Agent beats need your agent credentials; with
them loaded, re-record from the repository root,

```bash
vhs scripts/demo/tui.tape
```

and replace `docs/demo-tui.gif`.

## Docs

| Document | What |
|---|---|
| [Capability contract](docs/harness-capability-contract.md) | Full breakdown of the 9 capabilities |
| [Cataloged agents](docs/supported-agents.md) | All 25 descriptors and support caveat |
| [Support matrix](docs/support-matrix.md) | All 225 capability truth rows |
| [Security gates](docs/security-gates.md) | Optional Trivy scan behavior and configuration |
| [Development](docs/development.md) | Architecture, verification, and release artifacts |
