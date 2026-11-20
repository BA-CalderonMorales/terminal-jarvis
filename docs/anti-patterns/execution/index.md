# Execution Anti-Patterns

### Provenance age used as a user-facing lock (UX audit, 0.1.25)

**Pattern:** `install`, `update` and every lifecycle command consulted the
capability's `verified_at` and refused when the evidence was older than the
refresh window -- 24 of 25 harnesses became uninstallable on a release build,
`--dry-run` included. Readiness did the same, so installed binaries read
`not-ready` because their catalog row was stale. A verification gate then
codified the bug: `integration-hardening.sh` asserted `exit 4` for a fleet with
nothing installed, so the one machine that had to pass failed.

**Why it is wrong:** Evidence age is a statement about the maintainer's work
queue, and the catalog ships inside the binary. The user cannot refresh it, so
the refusal is unactionable: it blocks a working install to report a fact only
the maintainer can change. "Nothing is installed yet" is a normal first-run
state, not a fault, so a diagnostic that exits non-zero on every fresh machine
teaches users to ignore exit codes.

**Fix:** Keep provenance in the report, never in the decision. Block on what
the user can act on -- platform incompatibility, an unclaimed capability
(`stub`, `manual`, `unsupported`, `disabled`), a required interactive terminal
-- and surface staleness as a footnote and a `check` row. Make `--dry-run` and
every preview unconditional past consent. Never assert a failure exit for the
empty-fleet state; assert `exit 0` and add a regression check that no
`executable missing` row is an error.

### Synthetic events that bypass the input protocol (v0.1.24 release review)

**Pattern:** Completion tests constructed a Tab event that the decoder never
emitted. Paging helper tests supplied a full tail that the real reader truncated.

**Why it is wrong:** The handler can pass every test while physical keys fail.

**Fix:** Drive actual bytes through the same decoder and terminal entry path.
Keep helper tests, but add terminal regressions for framing and consumption.

### Mixing bytes, characters and display cells (v0.1.24 release review)

**Pattern:** Completion advanced one byte after Unicode whitespace; picker
budgets omitted cursor/chip cells and placed ellipses beyond the stated limit.

**Why it is wrong:** Valid input can panic, and clipping hides meaningful state.

**Fix:** Advance string offsets at character boundaries. Budget rendered cells,
including every prefix, marker and ellipsis, before optional text. Exercise wide
characters and small supported geometries rather than only ASCII defaults.

### Reconstructing intent from the execution transport (PR #220)

**Pattern:** The streaming guard treated every request as explicit, although
active-agent shorthand and direct commands carry different consent semantics.
Resolving direct lifecycle capabilities also skipped their binary postconditions.

**Why it is wrong:** Choosing a display transport can grant permissions that
blocking execution refuses, or turn installer exit zero into apparent success.

**Fix:** Preserve the original request's intent through planning and authorization.
Apply capability postconditions on every entry path. Keep verifying installation
separate from adopting an active agent. Test refusal before any child or gate is
started, successful explicit consent, missing targets, and selection preservation.
