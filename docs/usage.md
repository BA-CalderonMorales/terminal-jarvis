# Usage and demo

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

## Demo walkthrough

The demo recording in the README (`docs/demo-tui.gif`) was produced with
[VHS](https://github.com/charmbracelet/vhs) from `scripts/demo/tui.tape`:

1. boot the tui (`terminal-jarvis tui`)
2. `list` -- the numbered picker
3. `use codex` -- switch the active harness
4. `home` -- back to the welcome frame
5. `status` -- the readiness dashboard
6. `use pi` -- switch by name
7. `exit` -- leave the switcher

## Re-recording with agent handovers

`scripts/demo/tui.tape` also carries the full story -- `run codex` and hand
over the terminal, `/exit` back, `run pi`, say `hi!`, wait for the answer,
`/exit`, then `exit` the switcher. Agent beats need that agent's credentials
and a working install; with them loaded, re-record from the repository root,

```bash
vhs scripts/demo/tui.tape
```

and replace `docs/demo-tui.gif`.
