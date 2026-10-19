# Make the frame one themed surface with a role vocabulary

Branch `release/0.1.21` -> `develop`, 2026-10-19. Linked from `README.md`.

**Decision:** Replace the tui's two styling systems with one role vocabulary
(`screen::roles::Role`) resolved by the active theme, and route the tui-native
body renderers -- header, primer, `status`, `help`, the `list` heading, the
verdict cards -- through `theme::paint`. `Palette` becomes a table of one SGR
code per role, so each of the six themes is a full design rather than three
colours. Narrow viewports drop a table's secondary column instead of clipping
it.

**Context:** The chrome was theme-driven and always emitted SGR, while every
body screen used `cli::style`'s hardcoded colours behind an `is_terminal()`
gate. A theme swap therefore repainted the header and nothing else, and the
flatness was two systems disagreeing rather than a design choice. `body_block`
also wrapped each line in `theme::dim`, and an inner reset cancelled the outer
dim mid-line, so styled rows read as half-dimmed.

**Considered:** Leaving `cli::style` as the body system and adding theme codes
alongside it means two palettes to keep in sync, and the next body screen would
have picked whichever it saw first. Making `theme::paint` respect
`is_terminal()` so `cli` could delegate inverts the domain dependency and
breaks the theme tests, which assert exact SGR. Converting the shared `show` /
`plan` renderers in the same pass is tempting -- they are the headless surface
too -- but their colour gate has to be unified first, so it stays deferred and
recorded in the structure ledger.

**Consequence:** One styling entry point, so a theme swap is total; the
theme-completeness and `paint_wraps_once_and_never_nests` tests hold it. The
header carries the working directory again, and a narrow terminal loses a
secondary column by rule instead of mid-word. `show` and `plan` still paint
through `cli::style`, which is the visible next step: they render monochrome
inside a themed frame.
