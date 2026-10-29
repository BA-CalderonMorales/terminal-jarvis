# Let the picker answer "what can I run" and the json answer "in what shape"

Prepared for `release/0.1.23` -> `develop`, 2026-10-29.

**Decision:** The tui picker renders every harness with the dashboard's
readiness verdict (`[ready]` / `[install]`) and accepts a text filter
(`list <text>`). A filtered picker keeps the original picker numbers, so a
number still selects the row it names. `--json` for `list`, `show`, and `plan`
now publishes structured catalog fields beside the rendered `text`, inside the
existing envelope and under the same `schema_version`.

**Context:** The picker answered "what exists", not "what can I do": its rows
carried a name and a description, so readiness required a second command and a
mental join across 25 tools. The only way to narrow the fleet was an exact
index or an exact name, and a typo ended in an error instead of a smaller list.
On the automation side, `check --json` already returned structured diagnostics,
but the catalog commands returned prose inside `data.text`, so an agent could
query health structurally and still had to re-parse
`support=expected=5,stub=3,disabled=1` to decide what to run. That asymmetry
was the difference between a scriptable tool and one that is solid for a
particular set of tasks.

**Considered:** Sorting ready tools first is the obvious "helpful" gesture, but
it breaks the index-to-tool contract that the number keys rely on, and it moves
rows under a user who learned them. A filter mode inside the raw prompt (typing
narrows live without the `list` verb) is the better long-term interaction, but
it entangles the key editor, the picker state machine, and the redraw path in
one release-branch change; the verb form gets the same outcome with a resolver
arm and a pure renderer. A separate catalog schema was rejected: the envelope
already carries `schema_version`, `text`, and `error`, so adding fields is
backward compatible and keeps one envelope for every command. Leaving agents to
parse the human text was rejected as the defect itself.

**Consequence:** Fleet truth is read in one place (`tui::shell::fleet`), so the
picker, the dashboard, and the header cannot disagree about readiness. The
fallback policy that decides between embedded and on-disk data is split from
the filesystem probe it depends on, so every root state is witnessed without
borrowing the process working directory. Mutation remains the gate: the picker's
chip, filter, and narrow-frame branches are pinned by unit tests, and the
Windows-only tree module is excluded by a file glob because cargo-mutants
27.1.0 does not apply `exclude_re` to `delete field ... from struct ...`
mutants. `--json` consumers may rely on the field names above; `text` stays the
human contract and is not a parsing surface.
