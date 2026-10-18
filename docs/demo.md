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

`docs/demo-tui.gif` is a real capture of the tui on an Omarchy desktop: the
focused monitor, the Omarchy bar, the wallpaper, and a floating `foot` window
running the release binary. `scripts/demo/record-omarchy.sh` drives it, and it
mirrors `scripts/demo/tui.tape` step for step so the two paths stay comparable:

1. boot the command center (`terminal-jarvis tui`)
2. `list` -- the numbered fleet picker
3. `status` -- the readiness dashboard
4. `show codex` -- one harness in detail
5. `codex` -- the switch, so the header leaves `ACTIVE none`
6. `plan codex headless` -- preview a command without running it
7. `run codex -- --version` -- refused with `dangerous_opt_in_required`: that
   is the guard working, not a failure
8. `/theme solarized` -- a named theme
9. `help`, then `Esc` and `j`/`k` -- NORMAL mode scrolls the table, `i` returns
10. `home` -- back to the welcome frame, where the recording stops

Nothing needs credentials, and nothing installs or runs an agent: the recording
machine is a daily driver, so the lifecycle verbs are previewed and refused
rather than exercised. `docs/development.md` and the tui's own `help` cover what
they do.

### Recording the GIF

On an Omarchy desktop with the release binary built:

```bash
cargo build --release
scripts/demo/record-omarchy.sh DP-14 9     # prints the crop and the trim
```

Feed the crop and trim it prints to the encoder:

```bash
ffmpeg -ss "$SS" -t "$T" -i ~/Videos/screenrecording-*.mp4 \
  -vf "crop=1500:940:210:0,fps=10,scale=1000:-1:flags=lanczos,split[a][b];[a]palettegen=max_colors=192:stats_mode=diff[p];[b][p]paletteuse=dither=bayer:bayer_scale=3:diff_mode=rectangle" \
  -loop 0 docs/demo-tui.gif
```

Two things the script exists to get right. Input comes from `drive.py` over a
pty the window owns, because synthetic key events stopped reaching the window
partway through an earlier take, and a recording that silently stops receiving
input is worse than no recording. And the tui is started only once the window
is placed, because one that boots while its window is still tiled measures the
tiled geometry and composes frames the placed window then clips.

### Alternative: a headless render

`scripts/demo/tui.tape` drives the same flow under
[VHS](https://github.com/charmbracelet/vhs), which needs no desktop and no
screen-recording permission -- useful on a headless box or in CI:

```bash
vhs scripts/demo/tui.tape      # -> docs/demo-tui.gif
```

It renders 1000x560 with no window chrome and no wallpaper, so the README uses
the real capture. The tape is kept in step with the recording so either path can
be re-rendered from the same script.

## Making new demos

The flow lives in two files, and they are kept in step:

- `scripts/demo/tui.tape` -- the VHS tape (`Type` / `Sleep` / `Escape`).
- `scripts/demo/drive.py` -- the same steps as `(settle, keys)` pairs for the
  desktop recording, plus the `PER_CHAR` typing delay.

Change both when the flow changes. The `Sleep` and the settle before each step
are the beat a screen holds, and they are the same number in both files: a
viewer reads a command while it is up, so a shorter beat makes the demo harder
to follow even though it is shorter.

The recording window is `foot` at `size=12` in 1000x620 -- about 110 columns,
which fits the plan table without truncating. It is deliberately the same grid
the tape renders, so the two candidates differ only in what surrounds the
terminal. `record-omarchy.sh` takes the monitor name, and a `TJ_DEMO_OUT`
directory for its trim output; it restores the desktop it borrowed, and refuses
to run a window dispatcher unless the demo window holds focus.
