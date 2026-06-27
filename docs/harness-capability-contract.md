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

## Adding a New Agent

```bash
mkdir -p harnesses/<agent>/{download,update,headless,version,stats,models,security,yolo,ui}
```

Write `index.toml` for the harness root and each capability. Each
capability `index.toml` contains a `summary`, `command`, and `args`.

Run `scripts/verify.sh` to validate the contract is met.
