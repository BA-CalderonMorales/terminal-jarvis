# Terminal Jarvis

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
| `version [--verbose]` / `--version` / `-v` / `--info` | Version info |
| `config show` | Active config state |
| `auth help <harness>` | Credential setup guidance |
| `[harness] [args...]` | Pass-through to harness binary |

> Legacy aliases from 0.0.x: see [docs/migration.md](docs/migration.md).

## Docs

| Document | What |
|---|---|
| [Capability contract](docs/harness-capability-contract.md) | Full breakdown of the 9 capabilities |
| [Supported agents](docs/supported-agents.md) | All 25 coding agents |
| [Maintainer guide](docs/maintainer-guide.md) | Development, verification, release |
| [Architecture](docs/architecture.md) | Module boundaries and contracts |
| [Release plan](docs/release-plan.md) | Checklist and auth boundaries |
| [Migration](docs/migration.md) | Breaking changes from 0.0.x |
| [Testing](docs/testing.md) | Test gates and integration hardening |
| [Distribution hardening](docs/distribution-hardening.md) | Package integrity and provenance |
