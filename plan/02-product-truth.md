---
id: "02"
target: v0.1.13
title: Product Truth
status: complete
owner: core-maintainer
starts_after: ["01"]
completion_requires: ["01"]
independent_review_required: false
---

# 02 - Product Truth

## Objective

Make the CLI tell the truth about support, environment readiness, side effects,
and distribution state before it plans or executes a third-party command.
Support metadata, diagnostics, execution guards, output, and documentation must
derive from the same contracts rather than independent Rust branches or prose.

## Workstreams

The workstreams below may run in parallel after Phase 01.

### Support model

- Extend the smallest backward-compatible catalog schema with support state,
  evidence mode, side-effect class, platforms, source, and verification freshness.
- Classify all 225 capability rows. Generate initial states from current facts;
  use `unknown` rather than researching or installing every upstream CLI.
- Validate state/command contradictions, incomplete platform claims, unsafe
  lifecycle commands, and ambiguous help fallbacks at catalog load time.
- Generate support reports and public tables from the catalog.

### Diagnostics

- Implement the Phase 01 diagnostic decision as one canonical rich/plain/JSON surface.
- Report Terminal Jarvis version, distribution, resolved executable, path
  shadowing, OS/architecture/libc/shell, TTY/color/width decisions, catalog and
  gate sources, active harness, support state, resolved harness binary/version,
  environment readiness, config/cache state, and update route.
- Use a strict field allowlist. Report credential-like variable names and
  presence only; empty values are not ready, and values are never emitted.
- Distinguish missing, empty, malformed, unsupported, conflicting, stale, and
  permission-denied states with stable codes and next actions.
- Keep diagnostics local and read-only: no network, install, update, provider,
  scanner, or harness execution.

### Safe execution and CLI consistency

- Apply the Phase 01 parser, option-position, help, error, stream, and exit contract.
- Reject extra arguments and ignored flags. Add versioned JSON and keep plain
  output stable and decoration-free.
- Guard `stub`, `unsupported`, `disabled`, and incompatible-platform rows before
  spawning a child process.
- Add contract-approved dry-run and explicit-intent behavior for install,
  harness update, and dangerous capabilities, including noninteractive use.
- Preserve child stdout/stderr and child exit status according to the contract;
  do not discard successful stderr or report failed stderr on stdout.

### Distribution truth

- Reuse existing Cargo, npm/npx, Homebrew, and direct-asset machinery.
- Make version, channel, resolved path, catalog location, cache behavior,
  checksum result, update route, and unsupported-platform errors consistent.
- Validate corrupt/missing caches, stale path entries, wrong architecture,
  unsupported libc/platform, read-only homes, missing catalog, and channel conflicts.
- Keep packaging and update operations nonpublishing throughout this phase.

## WIP Checkpoint - 2026-07-17

Implemented in the current branch checkpoint:

- strict metadata and contradiction validation for all 225 capability rows,
  currently classified as 99 `stub`, 23 `disabled`, and 103 `unknown`, with no
  promoted executable-support claim;
- catalog-derived list/show/plan/run truth, freshness and platform guards, and
  pre-spawn rejection evidence for manual, stub, unsupported, disabled,
  unknown, stale, incompatible-platform, dangerous, and non-TTY interactive paths;
- canonical local diagnostics with rich/plain/schema-v1 JSON, redaction,
  readiness, path/platform/presentation/catalog/gate/config/cache/update/checksum
  records, and read-only permission inspection;
- strict option parsing, stable handled-error envelopes, lifecycle preview and
  bound intent, byte-preserved child streams, exact and signal exits, and
  display-cell-aware width handling;
- shared distribution-channel and update-route normalization plus initial npm
  platform, checksum, extraction, and recovery hardening.

Remaining before this phase can become evidence-ready:

1. Persist and validate npm cache identity and integrity metadata for target,
   architecture, archive, binary, catalog, and gates; preserve valid read-only
   reuse and safe staged recovery; export the verified checksum state.
2. Add the canonical self-update command/help forms required by Phase 01,
   remove the unreachable alternate `check` dispatch, and stop update summaries
   from exposing commands for guarded rows.
3. Generate all 225 public support rows with evidence, freshness, source, and
   platform fields; derive all five first-class non-promotion decisions from
   data; remove blanket support claims from README and support documentation.
4. Add interactive PTY confirmation evidence and close any remaining
   diagnostic/distribution conflict cells.
5. Run the complete gates on one committed ref and populate the evidence table;
   the current 224-test working-tree pass is not accepted phase evidence.

## Work

- [x] Implement and validate the catalog support schema without adding per-harness
  Rust conditionals or an external runtime dependency.
- [x] Classify all 25 harnesses and 225 capability rows from evidence.
- [x] Implement the canonical side-effect-free diagnostic surface and versioned JSON.
- [x] Apply the CLI argument, output, stream, exit, and remediation contract.
- [x] Enforce support/platform guards and explicit intent before lifecycle execution.
- [x] Unify distribution provenance, path/cache/catalog diagnosis, and update guidance.
- [x] Generate support documentation from catalog data.
- [x] Add focused tests for every changed contract before collecting Phase 03 evidence.

## Acceptance Criteria

- [x] `TRU-01` All 225 rows have valid support state, evidence mode, side-effect
  class, platform scope, source, and freshness; contradictions fail validation.
- [x] `TRU-02` List, show, plan, check/doctor, and run expose the same catalog
  truth, and no non-verified state is rendered as verified.
- [x] `TRU-03` Diagnostics have deterministic rich/plain/versioned-JSON output
  and stable codes for every declared failure class.
- [x] `TRU-04` Seeded secrets, credential values, home names, and sensitive paths
  are redacted, and diagnostics have no network or child-process side effects.
- [x] `TRU-05` Empty environment variables are not ready; missing, empty,
  malformed, unsupported, and conflicting states remain distinguishable.
- [x] `TRU-06` Unknown arguments and misplaced flags fail clearly; aliases and
  machine output follow the compatibility contract.
- [x] `TRU-07` State-changing and dangerous capabilities support preview and
  require the explicit intent defined by Phase 01 in interactive and automated use.
- [x] `TRU-08` Stubbed, disabled, unsupported, and platform-incompatible rows
  fail closed before process launch with a useful next action.
- [x] `TRU-09` Child stdout, stderr, and exit status are preserved according to
  the contract on success, failure, signals, and non-UTF8-safe output.
- [x] `TRU-10` Cargo, npm/npx, Homebrew, and direct installs report consistent
  version/channel/path/catalog/cache/update truth and actionable conflicts.
- [x] `TRU-11` Generated support tables match the catalog and identify evidence
  freshness and first-class capability guarantees without blanket support claims.

## Evidence

| Covers | Method | Artifact | Ref | UTC | Result | Verified by |
|---|---|---|---|---|---|---|
| TRU-01, TRU-02, TRU-08, TRU-11 | exact-ref 225-row walk and generated drift check | plan/evidence/phase-03/parity/dev.tsv | ddc23b6 | 2026-07-18T22:19:51Z | pass | Codex |
| TRU-03, TRU-04, TRU-05, TRU-06, TRU-07, TRU-09 | `scripts/phase03-adversarial-report.sh` | plan/evidence/phase-03/adversarial.tsv | ddc23b6 | 2026-07-18T22:19:51Z | pass | Codex |
| TRU-10 | exact-ref development/staged identity plus npm wrapper suite | plan/evidence/phase-03/parity/identity.tsv | ddc23b6 | 2026-07-18T22:19:51Z | pass | Codex |

## Exit

This phase is complete when every acceptance criterion is covered on one tested
ref and all product claims derive from validated data. Independent release
review waits for Phase 04.
