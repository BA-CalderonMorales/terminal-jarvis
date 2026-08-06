# Demo

The session shown in the README (`docs/demo-tui.gif`) was recorded with
[VHS](https://github.com/charmbracelet/vhs) from `scripts/demo/tui.tape`:

1. boot the switcher (`terminal-jarvis tui`)
2. `list` -- the numbered picker
3. `use codex` -- switch the active harness
4. `home` -- back to the welcome frame
5. `status` -- the readiness dashboard
6. `use pi` -- switch by name
7. `exit` -- leave the switcher

`scripts/demo/tui.tape` also carries the full story -- `run codex` and hand
over the terminal, `/exit` back, `run pi`, say `hi!`, wait for the answer,
`/exit`, then `exit` the switcher. Agent beats need that agent's credentials
and a working install.

## Recording

Install VHS (`go install github.com/charmbracelet/vhs@latest`) plus `ffmpeg`
and `ttyd` on PATH, then from the repository root:

```bash
vhs scripts/demo/tui.tape
```

and replace `docs/demo-tui.gif`.

## Making new demos

The tape is plain text; copy `scripts/demo/tui.tape` and adjust:

- `Set FontSize`, `Set Width`, `Set Height`, `Set Padding` -- match the
  window to the terminal content so frames never scroll.
- `Sleep` between commands -- the recorder runs in real time, so a beat is
  as long as its `Sleep` plus whatever the tool takes to respond.
- `Type "..."` then `Enter` types a command; `Ctrl+C` sends the interrupt.
- The session shells out to bash: `terminal-jarvis tui` (with the binary on
  `PATH`) must launch before anything else, or every line you type goes to
  bash instead of the switcher.
- Agent launches render only as far as this machine's credentials allow;
  re-record on a machine where the agent genuinely chats before shipping a
  new gif.
