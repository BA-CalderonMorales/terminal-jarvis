# Keep retired tools in the catalog behind a graveyard

**Decision:** A harness whose upstream vendor retires it keeps its catalog row,
its capability rows, and its install/removal truth. It gains an optional dated
retirement record on the facade (`sunset_at`, `sunset_note`,
`sunset_replaced_by`), rendered by `terminal-jarvis graveyard`, marked `[sunset]`
in the picker, and counted apart from the maintained fleet instead of inside
`READY n/m`.

**Context:** Google retired Gemini CLI for consumers on 2026-06-18 and points
users to Antigravity; enterprise access continues through Gemini Code Assist and
paid API keys. Terminal Jarvis listed it as an ordinary, ready-capable harness,
so the tool was silent about the one fact that mattered. The same will happen to
other CLIs: vendor sunset is a normal lifecycle event, not an exception.

**Considered:** Deleting the harness was rejected. Deletion hides "what
happened?" from exactly the users who need it, and it discards install/removal
truth for a binary that may still be on their machine. A free-floating
`docs/graveyard.md` was rejected: it drifts from the catalog and cannot reach
the picker or readiness. Reporting a retired tool as `ready` with a note was
rejected: "ready" is a recommendation, and recommending an unmaintained CLI is
the claim this feature exists to stop making.

**Consequence:** `docs/harness-capability-contract.md` owns the schema
(`sunset_at` + `sunset_note` must travel together, the date must be a calendar
day). Readiness reports a retired harness as `retired` (warning, unsupported)
even when its binary resolves; the dashboard adds a `RETIRED n` clause only when
the catalog has retirements; `list` names it `(sunset)`. Adding a retirement is
a data change in `harnesses/<agent>/index.toml`, not a Rust branch. A new
sunset entry must cite the vendor's announcement in `sunset_note`; the sources
audit (`docs/harness-sources.md`) keeps the install channel evidence.
