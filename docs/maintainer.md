# Maintainer Guide

## Command surface

**Headless** (`terminal-jarvis <command>` / `tj <command>`) -- one command per
invocation, for scripts, CI, and one-shot tasks:

| Command | Purpose |
|---|---|
| `list` | Show all coding agents |
| `check` | Report binary + env readiness |
| `show <harness>` | Inspect a harness's capabilities |
| `install <harness>` | Install a harness |
| `update [<harness>]` | Upgrade a harness |
| `plan [harness] <capability>` | Preview the shell command |
| `use <harness>` / `current` | Select / show active harness |
| `run [harness] [capability] [args...]` | Execute a capability |
| `security [status\|audit\|harness]` | Security posture |
| `gate [status\|list\|enable\|disable\|run]` | Optional local security gate |
| `version [--verbose]` / `--version` / `-v` / `--info` | Version info |
| `self-update [--dry-run]` / `--update` | Update Terminal Jarvis or print the update command |
| `config show` | Active config state |
| `auth help <harness>` | Credential setup guidance |
| `[harness] [args...]` | Pass-through to harness binary |

**Interactive** (`terminal-jarvis tui` / bare `tj` on a terminal) -- the
chat-style switcher with the numbered picker and readiness dashboard. Every
headless command above also works here, plus:

| Command | Purpose |
|---|---|
| `tui` | Open the switcher |
| `home` | Back to the welcome frame (works as `clear` too) |
| `exit` | Leave the switcher |

## Security model

Auth stays with each harness -- terminal-jarvis never retains credentials.
