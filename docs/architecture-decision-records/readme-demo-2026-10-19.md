# Record the README demo on a real desktop, and drive it over a pty

Branch `release/0.1.21` -> `develop`, 2026-10-19. Linked from `README.md`.

**Decision:** `docs/demo-tui.gif` becomes a real capture of the release binary
on an Omarchy desktop -- wallpaper, bar, floating window -- produced by
`scripts/demo/record-omarchy.sh` through `scripts/demo/drive.py`. The VHS tape
stays as the headless alternative and is kept in step with it.

**Context:** The old gif was a VHS render: a terminal on a flat background, and
a flow that showed the fleet and the themes but never a switch, so the header
said `ACTIVE none` throughout and the product's central action was invisible.
Reviewing the first two captures also produced three findings worth keeping:
synthetic key events stopped reaching the window partway through a take, so the
last twenty seconds of one recording were a still frame; a tui that boots while
its window is still tiled measures the tiled geometry and composes frames the
placed window then clips; and a capture is only as clean as the desktop behind
it, which is how another recording ended up in frame.

**Considered:** Keeping VHS alone is reproducible everywhere but cannot show the
desktop, and the desktop is the part worth showing. Compositing a wallpaper
behind a render would be a picture of something that never ran. Driving with
`wtype` is the obvious automation and is what failed silently. Booting the tui
first and resizing after is what produced the clipped frames.

**Consequence:** The demo answers what a README has to answer -- what is
installed here, and what picking one does -- and it is honest about the limit:
the recording machine is a daily driver, so the lifecycle verbs are previewed
and refused rather than exercised, and nothing is installed. The window runs the
same ~110-column grid the tape renders, at the same pacing, so the two differ
only in what surrounds the terminal. `docs/demo.md` carries the crop and trim
commands.
