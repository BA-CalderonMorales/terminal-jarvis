# Harness Capability Contract

Every coding agent harness exposes the same 9 capabilities. Adding a new agent
means adding 9 `index.toml` files under `harnesses/<agent>/` -- no Rust code
changes.

## Capabilities

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

8 safe capabilities, 1 dangerous. Every harness implements all 9.

## Per-Harness Metadata

Each harness `index.toml` declares:

| Field | Purpose |
|---|---|
| `name` | Key used in CLI commands |
| `display` | Human-readable name |
| `description` | One-line summary |
| `binary` | Expected executable name |
| `env_mode` | `none`, `any`, or `all` |
| `env` | List of required environment variables |

Auth guidance stays at the harness level. Terminal Jarvis never retains
credentials -- it tells you what each harness needs and lets you manage
your own provider keys.

## Headless Invocation Guidelines

Every harness MUST define `headless/index.toml`. The headless capability
determines how `terminal-jarvis run <harness> <prompt>` behaves when
prompt words do not match a reserved capability name.

Three headless patterns are recognized:

| Pattern | Example | Behavior |
|---------|---------|----------|
| Direct exec | `opencode` → `opencode run`, `codex` → `codex exec` | Appends extra args (the prompt) to the command line and invokes the harness in non-interactive mode |
| `--help` stub | `claude` → `claude --help`, `aider` → `aider --help` | Harness has no documented non-interactive mode; shows guidance |
| Interactive-only | `forge`, `droid` | Falls back to `--help`; headless mode is explicitly a stub |

### Rules

1. A harness with a documented `--exec` / `--run` / `--pipeline` / non-interactive flag MUST use it in `headless/index.toml`.
2. A harness that ONLY supports interactive TUI MUST set `args = ["--help"]` so the user sees guidance.
3. Extra args from `run <harness> <prompt>` are APPENDED to the headless command line.
4. Harnesses MUST NOT execute destructive operations from headless mode without explicit user confirmation.
5. When a harness lacks any non-interactive mode, the `summary` MUST describe the capability as a stub.

## Adding a New Agent

```bash
mkdir -p harnesses/<agent>/{download,update,headless,version,stats,models,security,yolo,ui}
```

Write `index.toml` for the harness root and each capability. Each
capability `index.toml` contains a `summary`, `command`, and `args`.

Run `scripts/verify.sh` to validate the contract is met.
