# Usage

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

The demo recording, the full agent-handover script, and how to make new demos:
[Demo](demo.md).
