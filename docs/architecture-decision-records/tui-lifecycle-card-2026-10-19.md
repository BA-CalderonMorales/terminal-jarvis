# Give a lifecycle run its own card and stop stamping the log

Branch `release/0.1.21` -> `develop`, 2026-10-19. Linked from `README.md`.

**Decision:** A streaming run clears the body and starts from a themed chapter,
so the primer and the previous command's output yield to the task under way.
The live log drops the wall-clock stamp the runner prefixed and keeps the level
a line declares; the frame renders each classified row as `info   message`,
holding raw mode for the whole run and clipping rows rather than letting the
terminal wrap them.

**Context:** The frame accumulated. The primer stayed above an install; every
child row was stamped `HH:MM:SS INFO` by `runtime::classify` -- our stamp, not
the child's -- and the level was guessed from substrings of the whole line, so
a message containing "error" read as an error. A key typed while the child
worked echoed into the body because raw mode was released with the prompt. A
blank line became an `info` row with nothing after it, which is the stray line
that appeared after an install.

**Considered:** Keeping the stamp tells the reader nothing a live view does
not, and widens every row. Classifying on the whole line treats a message as a
level. Wrapping long rows instead of clipping them is friendlier to a wide
message but lets the frame grow past the terminal, so the frame clips (bounded,
and the full text stays available headless) and a streamed child is given no
controlling terminal, so an installer that reads `/dev/tty` fails closed
instead of painting over the frame.

**Consequence:** The screen tells one story per task; a log row carries the
level its tool declared; nothing echoes into the frame while a child works; the
body cannot outgrow the terminal. The runtime stream no longer owns a clock, so
`stamp` is gone, and `Drop for Session` prints the alt-screen exit without a
trailing newline so the restored screen does not scroll by a row.
