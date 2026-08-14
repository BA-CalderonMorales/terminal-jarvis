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

### Land the 0.1.15 hardening stack and repair develop's latent gate breakage (fix stack -> 2026-08-14)

**Decision:** Land the five-fix 0.1.15 stack through develop in one atomic
squash merge: deadline-bounded gate scans (clamped env timeout, bounded reader
join), whitespace/inline-comment tolerance in gate.toml state selection, real
jules version capability data, mutation CI thread capping, and the Windows
delete-pending trivy walk fix (--skip-dirs and --skip-files for "~"-prefixed
entries). Carry two develop repairs in the same stack: a dropped unused
import and a 106-line test file split, because both broke the verify gate on
develop's own head.

**Context:** A hung scanner could block a headless gate scan indefinitely, and
on Windows the trivy filesystem walk crashed on NTFS delete-pending entries.
develop itself was silently red: the #188 and #189 merges left an unused
import and an over-length test file (both merged without CI on their final
heads), so verify.sh failed on develop before any of this stack merged. The
stack's own first PR initially dropped reader output by abandoning reader
threads on the non-timeout path; that race was caught by CI and fixed by
draining pipes fully unless the scan was killed. Copilot review found the
timeout clamp was documented but not implemented and that the state parser
stopped at the first malformed enabled line.

**Considered:** Rejecting values above the 86400s cap with a fallback --
rejected, the docs promise clamping and a surprise 300s default is worse than
a bounded ceiling. Killing the last surviving env-aux mutant (a `<` vs `<=`
grace-boundary comparison) -- rejected as provably equivalent: the 50ms sleep
steps can never land on the exact boundary, so no test can distinguish it;
the mutation job is a documented non-gating pre-existing failure. Rebasing
the stale stack branches by hand -- rejected; gh stack rebase linearized the
stack within the tool's own flow instead of force-pushing outside it.

**Consequence:** Gate scans are bounded and their output is never truncated;
gate.toml selection tolerates real-world TOML; develop's verify gate is green
again and stays enforced (length gate + clippy -D warnings). The stack merges
are the 0.1.15 payload; release/0.1.15 must be rebuilt from this develop so
the release branch carries the same commits as the trunk.


### Bucket Rust domains with index faces (release/0.1.13 -> 2026-08-05)

**Decision:** Extend the scripts/ bucketing discipline to Rust: every `src/`
domain and every integration test domain becomes a folder of the shape
`{index.rs, structs/, logic/, constants/, tests/}` with a single public index
face, per-domain `[[test]]` binaries for `tests/`, and property generators
owned once by `src/contracts/tests/`. The loose `platform.rs` and
`distribution.rs` root modules folded into `src/context/`, so every caller now
reads `crate::context::platform::*` and the owning domain is visible at each
import site.

**Context:** `src/` was ~150 flat snake-named files with no structural signal
about role or domain; `tests/` still carried 31 `phase02_*`/`phase03_*` names
and four phase-scattered fixture homes; `cargo test` was a single 60-file blob.
The scripts/ ledger already declared this shape wrong, but the rule applied
only to scripts. The clean-api layering (bucket with a base face, swappable
implementation, tapped types, consistent error) is the same separation of
concerns at the file level, and this decision applies it to the core trees.

**Considered:** Keeping flat snake_named files where the name carries the
meaning -- rejected: nothing tells a reader (or an agent) which domain a file
belongs to, how it is consumed, or where a change's blast radius ends.
Folding `platform`/`distribution` into a new synthetic domain -- rejected:
`context` already owns environment detection (`default_home`, `catalog_root`,
`gates_root`), so it is the natural home and keeps the file count down.

**Consequence:** A domain is consumed through its `index.rs` face and invoked
as `crate::<domain>::...`, so imports are self-documenting provenance. A
`tests/<domain>/` binary isolates failures to a domain (`cargo test --test
cli`), making blast radius witnessable before an agent touches `logic/`.
New files must conform to the bucketed shape and the 100-line limit; the
anti-patterns ledger and `AGENTS.md` enforce it.

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

### Gate scans: heartbeat progress and interactive skip consent (fix/gate-scan-hang -> 2026-08-12)

**Decision:** Catch `develop` up to the shipped 0.1.14 state (0.1.14 landed on
`main` directly) and land the gate-scan fix: a slow scan redraws a
fixed-width heartbeat every five seconds, and an interrupted scan asks the
interactive user for conscious skip consent instead of hard-cancelling the
command. Findings still block (nothing vulnerable is ever downloaded),
`--no-input` and piped runs still abort.

**Context:** On OneDrive-backed mounts the trivy workspace walk legitimately
takes 5-23 minutes (measured), while the clean view showed one static line
with no feedback and no escape: Ctrl+C killed the scanner but cancelled the
install. `develop` had stalled at the 0.1.13 release while 0.1.14 shipped
straight to `main`, so the two histories needed to rejoin.

**Considered:** A hard scan deadline that auto-kills and prompts -- rejected,
the user is the only judge of when a scan is too long. A session-wide skip
memo -- rejected, one-action consent keeps the gate honest for every command.

**Consequence:** install/update/run for every harness become bounded and
engaged: heartbeat at 5s, Ctrl+C then `y` proceeds in seconds, exit 5 on
refusal, and a property-tested verdict contract (passed / blocked-with-
findings / interrupted) witnessed by pty acceptance tests. The 0.1.15
roadmap (CHANGELOG) carries blocked-install drill-down, `uninstall|prune`,
and header/main/footer section rules. Operator still tags and publishes
after final review.

### Ship the TUI command center and its demo pipeline (release/0.1.14 -> 2026-08-12)

**Decision:** Land 0.1.14: the TUI as a context command center -- a clean
frame (one-row banner status, `[>_]::[tj:<ver>]::[harness:<name>]` indicator,
`/debug` toggle, in-terminal `help`), live in-place lifecycle beats ending in
honest verdict cards, a session scope-memoized scan gate, interactive consent
for unknown installers, and a session bracket around every launch. Demo
assets split into a primary animated gif (recorded from `scripts/demo/
tui.tape`) and a registry still png (rendered by `capture_frame.sh` +
`render_frame.go` in the stock Windows Terminal palette).

**Context:** The 0.1.13 switcher dumped raw tool output, hid verdicts behind
redraws, rescanned on every run, and failed closed on curl-pipe installers
with no path forward. The README demo was a recorded script tied to a stale
frame. All 0.1.14 work accumulated on `release/0.1.14` unshipped.

**Considered:** Shipping a structured TUI library (curses/ratatui) -- rejected,
the std-only 100-line-per-file discipline is the project's identity and the
line-based frame already carries the surface. A png overlay generated from a
mock layout -- rejected, the capture/render pipeline renders the real binary's
frame, so the artwork can never drift from the product.

**Consequence:** The terminal is always returned to an interactive prompt --
idle Ctrl+C redraws cleanly, a stuck gate scanner is hard-killed. Installs
reproduce one scan per session; unknown installers keep an explicit human
gate. `status` shows `1 of N ready`. The 0.1.15 roadmap (CHANGELOG) carries
blocked-install drill-down, `uninstall|prune`, and header/main/footer section
rules. Operator still tags and publishes after final review.
