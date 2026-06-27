# Terminal Jarvis

A data-driven harness switcher for AI coding agents. Terminal Jarvis maps
**25 coding-agent CLIs** through a shared **9-capability contract** --
one interface to download, run, update, and inspect any agent tool.

## The Pattern

Every harness exposes the same capability set. The tool commands are uniform
across all agents. Differences shrink to the `index.toml` files.

### Tool Commands (what Terminal Jarvis does)

| Command | Purpose |
|---|---|
| `list` | Show all available coding agents |
| `show <harness>` | Inspect a harness's full capability set |
| `use <harness>` / `current` | Select / show active harness |
| `plan [harness] <capability>` | Preview the shell command for a capability |
| `run [harness] [capability] [args...]` | Execute a capability through the harness |
| `check` | Report binary and environment readiness |
| `security [status\|audit\|harness]` | Per-harness security posture |
| `version [--verbose]` / `--version` / `-v` / `--info` | Version and provenance info |
| `config show` | Active configuration state |
| `auth help <harness>` | Credential setup guidance |
| `[harness] [args...]` | Direct pass-through to the harness binary |

Legacy aliases: `tools` &rarr; `list`, `status` &rarr; `check`, `info` &rarr; `show`,
`install` &rarr; `run <h> download`, `update` &rarr; `run <h> update`.

### Harness Capability Contract (every agent, every time)

| Capability | Safety | Behavior |
|---|---|---|
| `download` | safe | Install the agent without sudo |
| `update` | safe | Upgrade without interactive auth |
| `headless` | safe | Run in non-interactive / command mode |
| `version` | safe | Print the installed agent version |
| `stats` | safe | Show local agent statistics |
| `models` | safe | List available models |
| `security` | safe | Review sandbox and approval settings |
| `ui` | safe | Open the interactive terminal UI |
| `yolo` | **dangerous** | Bypass all safeguards and approvals |

8 safe capabilities, 1 dangerous. Every one of the **25 harnesses** implements
all 9. Adding a new agent means adding a directory under `harnesses/` with
9 `index.toml` files -- no Rust code changes.

### Supported Agents

aider, amp, claude, code, codex, copilot, crush, cursor-agent, droid, eca,
forge, gemini, goose, hermes, jules, kilocode, letta, llxprt, nanocoder,
ollama, openclaw, opencode, pi, qwen, vibe

## Quick Start

```bash
# List every available coding agent
cargo run -- list

# Inspect a harness and its capabilities
cargo run -- show opencode

# Preview the command for a capability
cargo run -- plan codex headless

# Select an active harness
cargo run -- use opencode
cargo run -- current

# Check what's ready to run
cargo run -- check
```

### Verification

```bash
scripts/verify.sh              # fmt, lint, tests, shape, hardening, security
scripts/local-ci.sh            # stronger gate with repository hygiene
scripts/local-cd.sh --check-auth  # release asset shape without publishing
```

## How It Works

```text
terminal-jarvis/
├── harnesses/      # 25 agents, 9 capabilities each = 225 index.toml files
│   └── <agent>/
│       ├── index.toml         # name, display, binary, env requirements
│       ├── download/index.toml
│       ├── update/index.toml
│       ├── headless/index.toml
│       ├── version/index.toml
│       ├── stats/index.toml
│       ├── models/index.toml
│       ├── security/index.toml
│       ├── yolo/index.toml
│       └── ui/index.toml
├── src/            # Rust CLI (kept under 100 lines per file)
├── tests/          # contract, CLI, and integration tests
└── docs/           # architecture, testing, release, migration
```

Auth setup lives at the harness level (`env_mode`, `env`). A harness can
require no key, one of several, or all listed. Setup guidance stays accurate
without forcing every provider key.

## Development

- **Branch strategy**: feature branches from `develop`, PRs against `develop`.
  `main` is for tagged releases only.
- **CI**: runs on every PR not limited to docs. Docs-only changes skip CI
  (trigger manually via `workflow_dispatch` if needed).
- **File discipline**: keep Rust source files at 100 lines or fewer. Prefer
  `index.toml` data over Rust conditionals. Zero dependencies until one proves
  its value.
- **Environment**: use a remote or disposable Linux workspace. Harnesses
  install binaries and run delegated commands; keep provider tokens scoped and
  test on short-lived environments.

## Release

Cargo is the primary distribution. npm and Homebrew surfaces are generated
from the same release build.

```bash
scripts/package-release.sh build dist/
scripts/local-cd.sh --check-auth
```

Tagged releases (`v*`) trigger `.github/workflows/cd-multiplatform.yml`:
crate publish, GitHub release assets, Homebrew tap update, and npm package.

See [docs/release-plan.md](docs/release-plan.md) for the checklist and auth
boundaries.
