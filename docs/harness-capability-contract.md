# Harness Capability Contract

Every coding agent harness exposes the same 9 capabilities. Adding a new agent
means adding 9 `index.toml` files under `harnesses/<agent>/` -- no Rust code
changes.

## Capabilities

| Capability | Effect policy | Behavior |
|---|---|---|
| `download` | state-changing | Install without sudo |
| `update` | state-changing | Upgrade without interactive auth |
| `headless` | descriptor-specific | Run noninteractively; some agents are classified dangerous |
| `version` | descriptor-specific | Inspect the installed version |
| `stats` | descriptor-specific | Inspect local statistics |
| `models` | descriptor-specific | Inspect available models |
| `security` | descriptor-specific | Inspect sandbox and approval settings |
| `ui` | state-changing | Open an interactive terminal UI |
| `yolo` | dangerous | Bypass upstream safeguards and approvals |

All nine descriptors are required by the schema. Their presence does not mean
all nine operations work: `stub`, `disabled`, `manual`, `unsupported`, and
`unknown` rows retain their own guards. Use `show` and `plan` to inspect the
support state and effect of the exact operation. The `effect` field controls
consent; capability names alone do not establish safety.

### Evidence lifetime

Freshness uses the current UTC date at runtime. `verified` evidence lasts 30
days; `expected` and `manual` evidence last 90 days, with a one-day allowance
for future dates. Stale descriptors still load for inspection. Execution and
readiness checks reject stale executable claims. Invalid calendar dates are
schema errors. There is no build-time freshness override.

Before claiming fresh evidence, repeat the documented verification in a
disposable environment and retain its provenance. A package version bump or a
fake executable test does not renew upstream evidence. Release 0.1.23 retains
the August 5 evidence dates and downgrades the seven expired `verified` rows to
`expected`; it does not claim new real-agent verification.

## Per-Harness Metadata

Each harness `index.toml` declares:

| Field | Purpose |
|---|---|
| `name` | Key used in CLI commands |
| `display` | Human-readable name |
| `description` | One-line summary |
| `binary` | Expected executable name |
| `env_mode` | `none`, `optional`, `any`, or `all`; `optional` lists credentials a login-based harness can use but does not require, so readiness never gates on them |
| `env` | List of environment variables (`optional` mode: advisory only) |
| `sunset_at` | Optional `YYYY-MM-DD`: the day the vendor retired the tool |
| `sunset_note` | Required with `sunset_at`: what happened, in the vendor's terms |
| `sunset_replaced_by` | Optional with `sunset_at`: the tool the vendor points to |

### Retirement

A retired tool stays in the catalog: its binary, install channel, and removal
path are still true statements, and a user who already installed it needs the
ledger more than a deletion. `sunset_at` and `sunset_note` appear together; a
successor alone, or a date without a note, is a schema error, and the date must
be a real calendar day. `terminal-jarvis graveyard` renders the record (rich,
`--plain` key=value, `--json`), and the tui names retired tools in the picker
with a `[sunset]` chip.

A retired harness never counts as part of the maintained fleet: readiness
reports its `readiness` record as `retired`, the dashboard counts it apart
(`RETIRED n`) instead of inside `READY n/m`, and `list` names it `(sunset)`.
Saying "ready" for a tool nobody maintains is the claim this contract refuses.

### Removal

A `download/index.toml` may declare the tool's own remover as an optional
pair, preferred over any package-manager guess:

| Field | Purpose |
|---|---|
| `uninstall_command` | Executable to run, e.g. `opencode` |
| `uninstall_args` | Its argv, e.g. `["uninstall", "--force", "--keep-config"]` |

Both keys appear together and only on `download`; anything else is a schema
error. Without the pair, removal inverts the install channel -- `npm uninstall
-g`, `uv tool uninstall`, `cargo uninstall` -- and never guesses for a script
installer. After a successful removal Terminal Jarvis re-checks the binary: a
survivor owned by another channel is reported as incomplete, not as success.

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

Run `scripts/bash/verify/index.sh verify` to validate the contract is met.

## Optional Security Gate

Harness commands can be preceded by an opt-in Trivy scan. The gate is separate
from harness metadata so scanner policy can evolve without turning a scanner
into a required harness dependency. See [Security Gates](security-gates.md).
