# Clear the frame once per size change instead of erasing every paint

Branch `release/0.1.21` -> `develop`, 2026-10-19. Linked from `README.md`.

**Decision:** `resize::settle` erases the screen once when the painted size
changes, and only then. `screen::parked` calls it, so both paint paths (the
viewport and the raw prompt) inherit it without touching either call site.

**Context:** A frame pads its rows to `cols - 2` so that a full row can never
set the terminal's pending-wrap state. That leaves the last two columns
holding whatever the previous frame wrote there -- invisible while the terminal
keeps its size, and a stale tail the moment it shrinks. It stayed on screen
until content changed, so it survived into the README recording: a compositor
resize left `bo`, `to`, `ss`, `an` at the end of the rows.

**Considered:** Erasing every frame fixes it and flickers, which is why the
paint path was written to overwrite in place. Padding rows to `size.cols`
instead of `inner` would clear the tail by writing the last cell, and then a
full row could set pending-wrap -- the thing the two-column margin exists to
prevent. Clearing only on a size change keeps steady-state paints erase-free.

**Consequence:** `settle_at` takes the state the caller owns, so a test pins
the decision rather than a process-wide cache; `settle` wraps it with the
static. The erase is `\x1b[2J` alone, not `term::clear_screen()`: a frame opens
with its own cursor home, and a second one reads to the acceptance frame reader
as an extra, empty frame -- the regression the gate caught before this shipped.
