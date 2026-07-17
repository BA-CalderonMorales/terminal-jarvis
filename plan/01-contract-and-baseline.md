---
id: "01"
target: v0.1.13
title: Contract and Baseline
status: in-progress
owner: core-maintainer
starts_after: []
completion_requires: []
independent_review_required: false
---

# 01 - Contract and Baseline

## Objective

Turn the current v0.1.12 behavior and issue history into one measurable release
contract. This phase decides what Terminal Jarvis claims before implementation
changes those claims.

## Baseline to Reproduce

- Release: `v0.1.12`; branch product metadata remains `0.1.12` until Phase 04.
- Catalog: 25 harnesses and 9 capability descriptors per harness.
- Claimed native targets: Linux x64/ARM64 GNU, macOS x64/ARM64, and Windows x64.
- Claimed channels: Cargo, npm/npx, Homebrew, and direct release assets.
- Existing gates: Rust and npm tests, integration hardening, core-command matrix,
  package checks, release preflight, security checks, and mutation testing.
- Current catalog shape overstates behavior: at least 121 of 225 capability rows
  are help fallbacks or explicit fail-closed placeholders while the public docs
  present all nine capabilities uniformly.

The baseline artifact must also reproduce argument, stream, exit, readiness,
redaction, width, and distribution behavior. A passing historical test is not
proof that the behavior is desirable; record gaps separately from regressions.

## Known Gaps to Reproduce

- There is no JSON output mode.
- Presentation flags are position-sensitive; forms such as `list --plain` may
  ignore the flag instead of failing or producing plain output.
- Read-only commands may ignore unexpected arguments, and `check` can exit zero
  even when nothing is ready.
- An environment variable may count as present when its value is empty.
- Install, harness update, and dangerous lifecycle paths lack one consistent
  preview, confirmation, and noninteractive-intent contract.
- Child-process success stderr can be discarded, while failure diagnostics can
  cross the stdout/stderr boundary.

These are starting hypotheses backed by the current implementation audit. The
baseline records exact repro commands and results before any fix.

## Decisions to Freeze

### Support states

Every harness capability uses exactly one state:

- `verified`: passed current evidence in a declared environment;
- `expected`: matches current upstream documentation but lacks current real smoke;
- `manual`: requires safe human interaction or credentials and has documented steps;
- `stub`: intentionally returns guidance rather than the claimed operation;
- `unsupported`: known not to work in the declared environment;
- `disabled`: blocked by Terminal Jarvis safety policy;
- `unknown`: insufficient evidence and never displayed as supported.

Each row also records evidence mode (`deterministic`, `disposable-real`,
`manual`, or `unsupported`), side-effect class, supported platforms, required
binary, environment requirements, upstream source, and verification freshness.

### First-class harnesses

Audit OpenCode, Codex, Claude Code, Gemini, and Hermes as the candidate
first-class set named in issue #135. A harness becomes first-class only when
the phase records the exact capability guarantee and safe smoke evidence. Other
harnesses remain useful catalog entries with honest per-capability states.

### CLI contract

Freeze the expected behavior for:

- commands, aliases, global options, option position, and unexpected arguments;
- rich, plain, JSON, no-color, TTY/non-TTY, `TERM=dumb`, and width handling;
- stdout, stderr, child stream forwarding, stable exit classes, and remediation;
- missing, empty, malformed, unsupported, unavailable, stubbed, disabled, and
  dangerous states;
- read-only, state-changing, networked, interactive, and dangerous operations;
- compatibility and deprecation rules for aliases and machine-readable fields.

The contract must resolve whether diagnostics extend `check` or use `doctor`.
It must include a versioned JSON schema; silently ignoring a flag or argument is
never compatible behavior.

## Work

- [x] Reproduce the baseline on the exact branch ref and record passed, failed,
  and unavailable checks without converting unavailable checks into passes.
- [x] Inventory every public command, alias, option, exit class, output mode,
  state-changing operation, install channel, and claimed platform.
- [x] Freeze the support states, evidence modes, side-effect classes, freshness
  policy, and first-class-harness definition.
- [x] Freeze the CLI and diagnostic contract, including stable JSON and stream rules.
- [x] Define the exact native, shell, libc, container, and distribution matrix;
  mark Termux, musl, WSL, and other undecided combinations explicitly.
- [x] Define numeric success and abort thresholds for every acceptance criterion.
- [x] Map the remaining open work in issue #135 to Phases 02-04 and record what
  v0.1.12 already satisfies.
- [x] Confirm zero-host, zero-provider, zero-secret, and USD 0 maintainer spend
  for this release.

## Acceptance Criteria

- [x] `CTR-01` The recorded baseline identifies the tested ref, environment,
  commands, results, and every skipped or unavailable check.
- [x] `CTR-02` Every public CLI surface maps to one defined success/error,
  stream, exit, and rich/plain/JSON behavior.
- [x] `CTR-03` Support states and evidence modes cannot present unknown, stubbed,
  manual, unsupported, or disabled behavior as verified.
- [x] `CTR-04` The exact platform, architecture, libc, shell, and distribution
  claims are explicit; every other combination has deterministic handling.
- [x] `CTR-05` First-class status has a capability-level guarantee and does not
  require unsafe execution or credentials on a maintainer machine.
- [x] `CTR-06` Success and abort thresholds are numeric, and issue #135 has no
  unmapped release-confidence requirement.
- [x] `CTR-07` v0.1.13 contains no hosted execution/provider scope and requires
  no account, credential, paid service, or publication action to validate.

## Evidence

| Covers | Method | Artifact | Ref | UTC | Result | Verified by |
|---|---|---|---|---|---|---|
| pending | pending | pending | pending | pending | pending | pending |

## Exit

The core maintainer may mark this phase complete when all contract decisions,
matrices, thresholds, and baseline evidence are reproducible. No independent
review is required to begin implementation.
