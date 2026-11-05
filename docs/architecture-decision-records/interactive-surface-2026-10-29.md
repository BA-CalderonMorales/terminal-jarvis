# Give the frame an object, a running status rule, and an honest key contract

**Decision:** The tui's fleet picker is a modal state machine that owns keys
while open; the frame carries a permanent status rule (active, readiness,
gate); and every key the surface honours is advertised (`?`, `q`, `Tab`, the
resize downgrade, one-`Enter` consent). Arrow-key decoding is fixed underneath,
because none of this is usable while CSI keys vanish.

**Context:** A structural review read the surface as static output: `list`
printed rows, selection was a number typed at the next prompt, `converse`
needed a second invocation to accept its notice, and the frame silently became
a line prompt when the terminal shrank. Two of those complaints were wrong
(Normal/Insert editing and the welcome tool table existed), but the picker and
the silence were real. The deeper defect was invisible from reading the table:
`escape::resolve_and_decode` replayed the consumed `Esc` inside the buffer it
handed the decoder, so every arrow decoded as `Ignored` -- no cursor, no arrow
scroll, no arrow history, ever.

**Considered:** Rendering a fake progress stepper for upstream installers was
rejected: npm and cargo own that output, and an invented bar lies. Mouse support
and more vim chords were rejected as breadth without a reader. Deleting chat
mode was rejected: it is the ssh and narrow-terminal path, and its command
grammar is identical. Reordering picker numbers to match a live filter was
rejected earlier for stable indices; the overlay keeps those indices and makes
the cursor the visible selection instead.

**Consequence:** `picker` decides keys with no terminal (`Step`), `picker_view`
owns the window, `picker_row` owns one row's columns, and `picker_run` owns the
paint and stdin. The status rule is derived from the same `home::collect` the
welcome card uses, so it cannot disagree with `status`. The frame's hint and the
help table are now the key contract; a key that is not in help is not shipped.
Future surface work must keep the chat fallback at parity -- `?` parses in both
-- because the same command line drives both.

Recorded because this is the release/0.1.24 merge into `develop`.
