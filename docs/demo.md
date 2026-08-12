# Demo

## Registry still (PNG)

Registries like crates.io and npm don't animate GIFs, so the project also
ships a static frame: `docs/demo-tui.png`. It shows the boot banner, then a
live command with its output -- the `status` dashboard -- so new visitors
see what actually happens when they type, not an empty prompt. The animated
`docs/demo-tui.gif` stays on the GitHub README; the PNG backs the registry
pages.

Regenerate the PNG from the real binary:

```bash
cargo build --release
scripts/demo/capture_frame.sh demo      # -> docs/demo-demo.png
```

`scripts/demo/capture_frame.sh` boots the tui in a headless tmux pane (no
shell echo line, so the frame only ever contains TUI content), captures the
pane, and renders it with `scripts/demo/render_frame.go`, a std-only ANSI
replayer -- SGR colors, `\r` overwrites, and erase-to-EOL are applied the
way a real terminal does, then glyphs rasterize from a TTF onto the canvas.

The renderer defaults to the stock Windows Terminal scheme (Campbell: bg
`#0C0C0C`, fg `#CCCCCC`), which matches a default WSL profile; bold maps to
the bright palette slots exactly like Windows Terminal's intense-color
behavior. Point the fourth argument at any terminal's `colorScheme` JSON
(`{"foreground","background","ansiColor":[...16]}`) to match a custom
theme.

## Animated showcase (GIF)

The session on the GitHub README (`docs/demo-tui.gif`) is recorded with
[VHS](https://github.com/charmbracelet/vhs) from `scripts/demo/tui.tape`:

1. boot the command center (`terminal-jarvis tui`)
2. `list` -- the numbered picker
3. a bare number -- switch agents instantly
4. `status` -- the readiness dashboard
5. `plan pi headless` -- preview a command without running it
6. `help` -- the tui's own command table
7. `/debug on` then `/debug off` -- lift and drop the raw view
8. `home` -- back to the welcome frame
9. `exit` -- leave the switcher

The tape is self-contained: nothing needs credentials or external installs.
Lifecycle flows (install with the Trivy gate, update, run) appear exactly as
they do in the real tui: one-line plan preview, live `security scan`,
`package check`, and compact verdicts.

### Recording the GIF

Install VHS (`go install github.com/charmbracelet/vhs@latest`) plus `ffmpeg`
and `ttyd` on PATH, then from the repository root:

```bash
vhs scripts/demo/tui.tape
```

and replace `docs/demo-tui.gif`. The gif is deliberately not committed
until a recording machine re-renders it, so the artifact never goes stale
against the tape.

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
