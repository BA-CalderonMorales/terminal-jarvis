# Usage

## Two ways to run

| Mode | Invocation | Best for |
|---|---|---|
| **Headless** | `terminal-jarvis <command>` / `tj <command>` | Scripts, CI, one-shot tasks |
| **Interactive** | `tj tui` (bare `tj` on a terminal) | Switching between coding agents live |

Headless runs one command per invocation, prints stable line-oriented output
with `--plain`. Noninteractive state-changing runs require their exact
`--no-input --confirm=<capability>:<agent>` token. The interactive tui is a chat-style
switcher: `list` shows the numbered picker, a number or an agent name switches
instantly, `status` shows readiness, `home` resets the frame, and `exit`
leaves. A slashed name (`/codex`) launches an agent behind the same guarded
plan-plus-confirm as headless. Ctrl+C aimed at a running agent never kills the
switcher.

The demo recording, the full agent-handover script, and how to make new demos:
[Demo](demo.md).

## Catalog ownership

Default discovery uses catalogs installed beside the executable or in its
installation prefix, then bundled data. A repository's `harnesses/` and
`gates/` directories never override these defaults automatically.

For an intentional development or custom catalog, set
`TERMINAL_JARVIS_CATALOG=/absolute/path/to/harnesses` or
`TERMINAL_JARVIS_GATES=/absolute/path/to/gates`. Relative explicit paths are
also supported. Invalid explicit overrides fail without silently falling back.
`version --verbose` and `check` identify the selected catalog, including
`embedded:terminal-jarvis` for bundled data.

## Experimental conversations

In the viewport, `converse <turns> <a> <b> <topic...>` starts a conversation
limited to 1–12 turns. Read the first-use notice and repeat the command to
accept it. Every turn passes the ordinary support, freshness, platform,
intent, and workspace-gate checks. State-changing turns ask for consent.
For agents whose headless operation is dangerous, explicitly use
`converse --allow-dangerous <turns> <a> <b> <topic...>` and review each turn's
confirmation. The first-use notice does not authorize future dangerous runs.

Ctrl+C or Esc stops the current turn and ends the conversation. The prompt
remains usable, including cancellation of a subsequent install. Each reply is
limited to 256 KiB; the requested 80-word response length and “No tool use”
remain model instructions, not enforced upstream restrictions. A turn limit
bounds invocation count, not token cost or runtime.

Piped invocations own ordinary descendant processes. Unix uses a separate
session/process group; Windows uses a Job Object assigned before the suspended
child starts. Deliberately detached Unix descendants can escape a process
group. This lifecycle management is not an execution sandbox.
