# Architecture Decision Records

This directory keeps the meaning behind changes that merge into `develop`.

It is not a changelog. It records the *decision* that drove a merge and *why
that decision mattered* at the time, so the intent survives long after the
commit date scrolls past. Anyone landing on `main` without prior context --
humans or coding agents -- reads this record to reconstruct the reasoning edge
cases, tradeoffs, and intent of a change without re-deriving it from diffs.

## Rules

- A new record is written **only when merging to `develop`** (release branches
  and feature branches do not create records).
- Records accumulate in this `README.md`, newest first. One decision per hunk
  (create `docs/architecture-decision-records/<slug>-<date>.md` when a single
  merge carries more than one significant decision, and link it from here).
- Write the decision in present tense reasoning, not past-tense narration.
  State the constraints that were true then, the options considered, and why
  the chosen path stood up.
- Keep it short. A record is a signpost for understanding, not a spec.

## Format

```markdown
### <Decision title> (<branch> -> <date>)

**Decision:** One or two sentences. What changed on `develop` and why.

**Context:** The constraint or problem that forced the decision. What was true
at the time that would otherwise be invisible to a reader later.

**Considered:** The real alternatives (even rejected ones) and the tradeoff
that removed each.

**Consequence:** The durable effect on the codebase, the gates that now enforce
it, and anything future work must respect.
```

## Records

### Fix unsound quickcheck property and harden the release gate (release/0.1.13 -> 2026-08-04)

**Decision:** Replace the flaky `outcome_json_fields` quickcheck property with
an exact-envelope assertion, move the build script out of the repo root, and
add a `tj` short-form bin plus a manifest-derived harness risk report.

**Context:** The suite's pass/fail was sampling luck. The property asserted
`output.contains("\\n") == value.contains('\n')`, which conflates a literal
backslash-n in source text with a real newline; quickcheck only failed when the
random sample dragged in `\n`. fmt and clippy were also failing in the new
phase03 fixtures, so the release branch was not in a repeatable, trustworthy
state.

**Considered:** Patching the assertion to handle both `\n` and literal `\\n`
filters; rejected because escaping is not injective and the check still leaks
implementation detail. A structural, exact-envelope equality was chosen because
it is deterministic, stronger, and sound for all inputs.

**Consequence:** The gate is deterministic again. `tj` lowers the cost of daily
use. `scripts/python/catalog/index.py harness-risk` scores each harness from its own manifests
(dangerous/network surface only, so the report ranks rather than paints
everything the same color) and stays opt-in behind `TJ_HARNESS_RISK=1` until
it proves value. No root-level Rust scripts remain; the build script lives in
`build/build.rs` and the `build =` key in `Cargo.toml` points there.
