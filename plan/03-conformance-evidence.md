---
id: "03"
target: v0.1.13
title: Conformance Evidence
status: in-progress
owner: core-maintainer
starts_after: ["01"]
completion_requires: ["01"]
independent_review_required: false
---

# 03 - Conformance Evidence

## Objective

Build one reusable, secret-free test system that proves the CLI contract and
all catalog claims without running unreviewed agents on a daily-driver machine.
The same fixture must work against a development binary and staged release
artifacts so source and packaging tests cannot drift.

## Evidence Tiers

1. **Deterministic:** fake executables, controlled filesystems/environments,
   golden output, and failure injection cover every descriptor and CLI contract.
2. **Disposable real:** isolated CI/container/VM smoke verifies only approved,
   non-agent, non-secret lifecycle behavior for first-class harnesses.
3. **Manual:** a documented human procedure covers behavior that inherently
   requires safe interaction or user-owned credentials.
4. **Unsupported:** known-incompatible behavior is rejected deterministically.

Every capability row is assigned exactly one primary tier. Manual and
unsupported are honest evidence categories, not skipped deterministic tests.

## Deterministic Fixture

The fixture must:

- create fake harness executables from catalog data rather than hand-maintained cases;
- record argv, cwd, environment-name allowlists, stdout, stderr, exit status,
  signals, timeouts, and attempted side effects;
- inject missing binaries, empty/missing environment variables, malformed
  catalog/config/cache, path shadowing, permissions, corrupt checksums, wrong
  platform/architecture, non-UTF8-safe output, and interrupted execution;
- deny arbitrary shell input, network use, provider credentials, and writes
  outside its temporary root;
- reset deterministically and emit a machine-readable coverage report for all
  25 harnesses and 225 capability rows.

The fixture may also generate a self-contained transcript or recording that
demonstrates the candidate safely. It must visibly identify simulation, accept
no arbitrary execution, require no external service, and be produced from the
same tested binary and catalog. A separate marketing application is not required.

## CLI Contract Matrix

Generate tests for:

- every command, alias, help form, valid option ordering, invalid flag, and
  unexpected argument;
- rich, plain, JSON, no-color, TTY/non-TTY, redirected streams, `NO_COLOR`,
  `TERM=dumb`, CI mode, and widths 40/80/100/120 plus invalid or absent width;
- long names, paths, descriptions, Unicode, newlines, and unbroken words;
- stable stdout/stderr and exit classes for expected errors and child outcomes;
- diagnostics redaction and the guarantee that diagnostics execute nothing.

Golden updates must be intentional and reviewable. Terminal Jarvis-authored
layout obeys width constraints; byte-preserved child streams do not get rewritten.

## Disposable Real Smoke

- Freeze image/runner digests and tool versions used for evidence.
- Install only the first-class harness dependencies justified by Phase 01.
- Exercise package discovery, version/help, safe start/stop when proven
  non-agentic, and supported update preview. Do not submit prompts, mutate a
  repository, accept upstream terms, authenticate, or use provider credentials.
- Record upstream drift separately from Terminal Jarvis regressions.
- Keep Docker/VM definitions baseline environments that users may extend; do
  not turn them into installers for all harness dependencies.

## WIP Checkpoint - 2026-07-17

Implemented in the current branch checkpoint:

- a catalog-driven deterministic walk that records all 225 unique rows in a
  226-line TSV with support, evidence, guard, result, and tested-ref fields;
- fake-child proof for exact boundary argv, cwd, allowlisted environment name,
  matching streams, exit status, signal behavior, and zero pre-spawn effects;
- 24 passing Phase 03 tests covering canonical surfaces, compatibility aliases,
  help forms, option positions, child boundaries, invalid input, JSON errors,
  exit classes 2/3/4/5/126/127, TTY/color decisions, widths, long text, and Unicode.

Remaining before this phase can become evidence-ready:

1. Drive safe fake-executable execution and declared guard outcomes from every
   descriptor, with missing/empty/malformed state, permission, path shadow,
   checksum, architecture, timeout, and attempted-side-effect injection.
2. Prove seeded-secret and sensitive-path redaction across rich, plain, JSON,
   stderr, debug, child-failure, and generated support artifacts.
3. Record reproducible manual or unsupported treatment for every row that
   cannot receive safe automation, and make a data-derived promotion or
   non-promotion decision for all five first-class candidates.
4. Add only the bounded disposable-real smoke justified by those decisions;
   never submit prompts, authenticate, mutate a repository, or use credentials.
5. Run the same fixture against the development binary and a locally staged
   package, then commit the report and populate the evidence table on one ref.

## Work

- [ ] Build one catalog-driven fake-executable fixture and coverage report.
- [ ] Generate deterministic conformance for every capability row and support state.
- [ ] Generate the complete CLI/output/error/diagnostic contract matrix.
- [ ] Add adversarial redaction, path, permission, corruption, stream, signal,
  timeout, and side-effect tests.
- [ ] Add disposable real smoke for each approved first-class guarantee.
- [ ] Define manual procedures and deterministic unsupported cases for every row
  that cannot safely receive real automation.
- [ ] Run the fixture against a development binary and one locally staged package.
- [ ] Optionally generate the safe offline transcript from the same fixture.

## Acceptance Criteria

- [ ] `EVD-01` The coverage report contains every one of the 225 catalog rows
  exactly once with support state, evidence tier, result, and tested ref.
- [ ] `EVD-02` Fake-executable tests verify planned argv and guarded execution
  without network access, external writes, credentials, or real agent actions.
- [ ] `EVD-03` Every CLI command, alias, option, error class, output mode, width,
  stream, and exit category in Phase 01 has deterministic coverage.
- [ ] `EVD-04` Stubbed, disabled, unsupported, manual, dangerous, and unknown
  rows take their declared path and cannot accidentally spawn an operational command.
- [ ] `EVD-05` Seeded-secret and sensitive-path tests emit no protected value in
  rich, plain, JSON, stderr, debug, child-failure, or support artifacts.
- [ ] `EVD-06` Child success, failure, signal, timeout, stdout, stderr, and
  non-UTF8-safe cases preserve the Phase 01 contract.
- [ ] `EVD-07` Disposable real smoke covers every first-class guarantee without
  a provider secret, prompt submission, repository mutation, or maintainer-machine execution.
- [ ] `EVD-08` Manual and unsupported rows have reproducible procedures or
  negative tests and are never counted as automated passes.
- [ ] `EVD-09` A locally staged package passes the same fixture used for the
  development binary with matching version and catalog identity.
- [ ] `EVD-10` If generated, the offline transcript is pinned, self-contained,
  visibly simulated, noninteractive, text-accessible, secret-free, and
  reproducible from the fixture.

## Evidence

| Covers | Method | Artifact | Ref | UTC | Result | Verified by |
|---|---|---|---|---|---|---|
| pending | pending | pending | pending | pending | pending | pending |

## Exit

This phase is complete when the deterministic tier covers the full catalog and
CLI contract, while every real/manual/unsupported claim has honest bounded
evidence. It may complete in parallel with Phase 02.
