---
target: v0.1.13
branch: release/0.1.13
baseline: v0.1.12
scope: integration-hardening
owner: core-maintainer
delivery_mode: zero-host
maintainer_budget_usd: 0
status_source: phase-frontmatter
phases: ["01", "02", "03", "04"]
---

# Terminal Jarvis v0.1.13 Integration Hardening Plan

This plan closes the remaining release-confidence work in
[issue #135](https://github.com/BA-CalderonMorales/terminal-jarvis/issues/135).
It replaces the former 17-page waterfall with four gates. The reduction is in
coordination and duplicate evidence, not in product, security, compatibility,
distribution, or release verification.

## Outcome

Release v0.1.13 only when a user can:

- install the same candidate through every claimed channel and receive the
  expected native binary, catalog, version, checksum behavior, and update path;
- see truthful support and lifecycle claims for every harness capability;
- diagnose path, channel, platform, catalog, configuration, environment, and
  harness-readiness failures without exposing secrets or triggering side effects;
- rely on stable help, errors, exit codes, streams, plain output, color rules,
  and width behavior;
- reproduce the claims through deterministic tests, disposable real smoke where
  safe, and explicit manual or unsupported classifications everywhere else.

## Pain Points Being Solved

1. **Distribution drift.** Past failures crossed architecture, libc, executable
   permissions, wrapper caches, path shadowing, formula names, and package channels.
2. **Overstated harness support.** Descriptor presence is currently easy to
   mistake for verified behavior; many capability rows are help fallbacks or
   fail-closed placeholders.
3. **CLI contract drift.** Help, flags, errors, session handling, streams, color,
   width, and mutation coverage have regressed independently.
4. **Unsafe or shallow lifecycle validation.** Metadata shape tests do not prove
   that commands resolve, fail safely, or behave consistently in a clean environment.
5. **Artifact confidence.** Source tests alone do not prove that staged Cargo,
   npm/npx, Homebrew, and direct-release artifacts behave like the tested binary.

## Already Delivered; Audit Instead of Rebuilding

v0.1.12 already has five native release targets, distribution-aware updates,
checksum-verified npm downloads, Homebrew packaging, path-shadow guidance,
width-aware rich output, `--plain`, `--no-color`, catalog embedding checks, a
core-command matrix, release preflight, and mutation testing. Each is an input
to this plan. Reuse it unless evidence identifies a concrete gap.

## Scope Decisions

- v0.1.13 is integration hardening, not a new product surface.
- The release has no hosted terminal, public shell, provider adapter, provider
  account, provider credential, custom compute, analytics service, or paid runner.
- Docker or equivalent containers are disposable validation environments only.
  Publishing images is not required.
- One deterministic fixture may satisfy multiple criteria when its evidence
  record names every criterion it covers.
- The core maintainer may start, execute, and accept Phases 01-03. A distinct
  human review is required only for the final candidate before merge, tag, or publish.
- Tagging, publishing, registry changes, tap changes, and release uploads remain
  separate operator actions after this plan completes.

Hosted/public execution is intentionally deferred. Its activation and safety
boundary are preserved in [deferred-hosted-demo.md](deferred-hosted-demo.md).

## Four-Gate Execution Path

```text
01 Contract and baseline
        |
        +--> 02 Product truth --------+
        |                              |
        +--> 03 Conformance evidence --+--> 04 Release candidate
```

Phases 02 and 03 run in parallel after Phase 01. Phase 04 integration work may
also begin after Phase 01, but it cannot become evidence-ready until Phases 02
and 03 are complete. This separates permission to start from permission to ship.

| ID | Phase | Starts after | Must complete first |
|---|---|---|---|
| 01 | [Contract and baseline](01-contract-and-baseline.md) | None | None |
| 02 | [Product truth](02-product-truth.md) | 01 | 01 |
| 03 | [Conformance evidence](03-conformance-evidence.md) | 01 | 01 |
| 04 | [Release candidate](04-release-candidate.md) | 01 | 02, 03 |

The status in each phase's frontmatter is authoritative. Run
`ruby scripts/check-plan.rb` for the derived overall status.

## WIP Handoff - 2026-07-17

This branch contains an intentional Phase 02/03 implementation checkpoint. It
is not accepted evidence, does not authorize Phase 04 promotion, and remains
subject to the evidence contract and all hard gates below.

| Phase | Current implementation state | Remaining gate |
|---|---|---|
| 01 | Complete at `fab5848`; contract and baseline are frozen. | None. Do not weaken the frozen contract. |
| 02 | In progress: all 225 rows are truth-classified; strict catalog validation, canonical diagnostics, parser/output contracts, execution guards, lifecycle intent, stream/exit preservation, and distribution normalization are implemented with focused tests. | Finish npm cache integrity and architecture revalidation; canonical self-update help; support-aware update guidance; removal of the duplicate check route; generated 225-row public truth and derived 5/5 first-class decisions; documentation correction; interactive PTY evidence; exact-ref evidence. |
| 03 | In progress: the catalog-driven 225-row report, fake-child argv/cwd/environment capture, and 24-test CLI boundary/help/option/output/TTY/exit matrix pass locally. | Complete descriptor execution/failure injection, timeout and side-effect coverage, redaction across every output channel, manual/unsupported procedures, bounded disposable-real first-class decisions, development/staged-package parity, and exact-ref evidence. |
| 04 | Proposed only; no candidate, release, or publication mutation has begun. | Wait for accepted Phase 02 and 03 evidence, then execute the full native/delivery matrix, version/docs closure, CI gates, rollback rehearsal, and independent human review. |

Resume in this order:

1. Close the remaining Phase 02 truth and npm cache-integrity gaps without
   promoting any current catalog row.
2. Complete the Phase 03 deterministic fixture and evidence tiers, then run it
   against both the development binary and a locally staged package.
3. Record Phase 02/03 evidence on one immutable ref and only then begin Phase 04
   candidate closure.

Checkpoint verification: `cargo test --no-fail-fast` passes all 224 tests,
including the 24 Phase 03 matrix tests; the npm wrapper passes 28/28 tests; the
catalog report contains all 225 rows; support-report drift and `git diff --check`
pass; and the plan checker reports four phases, 38 criteria, and overall
`in-progress`. These working-tree results are useful WIP evidence only; phase
evidence must be rerun and recorded against the final committed refs.

## Former Plan Crosswalk

| Former pages | Current disposition |
|---|---|
| 01-02 baseline and UX contract | Phase 01 |
| 03, 05, 07 diagnostics, support, distributions | Phase 02 |
| 04, 06, 08 presentation, conformance, fixture | Phase 03 |
| 09 static showcase | Safe fixture transcript in Phase 03; marketing app deferred |
| 10-14 provider/hosted execution | Deferred hosted-demo contract |
| 15-17 CI, docs, release readiness | Phase 04 |

No former provider criterion is treated as passed. It is inapplicable because
v0.1.13 contains no hosted system. If hosted work is commissioned, the deferred
contract makes those risks mandatory again before implementation.

## Status Model

- `proposed`: scoped but not started;
- `in-progress`: work is active and all `starts_after` phases are complete;
- `blocked`: a specific blocker and recovery condition are recorded;
- `evidence-ready`: work and criteria are satisfied with reproducible evidence;
- `complete`: evidence is accepted; Phase 04 additionally has an independent reviewer.

No `ready` ceremony is required. A phase may move directly from `proposed` to
`in-progress` once its start dependencies are complete.

## Evidence Contract

Every acceptance criterion must be covered exactly once in its phase evidence
table. One row may cover several criteria and one artifact may be reused across
phases, so long as each row records:

1. criterion IDs covered;
2. exact command, workflow job, or evaluation method;
3. committed repository-relative artifact or durable HTTPS workflow/report URL;
4. tested 7-40 character commit SHA;
5. observation time in ISO-8601 UTC ending in `Z`;
6. result: `pass`, `approved`, `manual`, or `unsupported`;
7. the person who verified it.

`manual` and `unsupported` are valid only where the criterion explicitly permits
them. A skipped, unavailable, or failing hard gate is not a pass. Evidence must
be secret-free and reproducible from the recorded ref.

## Hard Gates

- Every public CLI command, alias, option, error class, exit class, and output
  mode is in the contract matrix.
- Every one of the 25 harnesses and 225 capability rows has a truthful support
  state and evidence mode; unknown never renders as supported.
- Diagnostics are allowlisted, redacted, deterministic, and side-effect free.
- Unsupported, stubbed, disabled, and dangerous paths fail closed with a next action.
- Deterministic conformance covers every descriptor; real harness smoke occurs
  only in disposable environments and never requires a secret or agent action.
- Every claimed install channel and release target is tested using staged
  candidate artifacts before any publication side effect.
- Full verification, security checks, mutation evidence, package checks,
  version alignment, documentation drift checks, and rollback rehearsal pass on
  the exact candidate ref.
- A reviewer other than the core maintainer approves the final candidate.

## Definition of Done

The plan is complete only when all four phase documents are `complete`, the
checker passes, the worktree contains only intended release changes, and Phase
04 records the independently reviewed candidate commit. Plan completion does
not authorize a tag or publication.

## Non-Goals

- Hosted terminals, public sandboxes, provider selection, PTY proofs, or FinOps.
- Installing or authenticating all third-party harnesses on a maintainer machine.
- Claiming all 25 harnesses have equal lifecycle support.
- Reintroducing a legacy TUI or undertaking unrelated architecture work.
- Publishing Docker images or a static marketing application.
- Optimizing mutation infrastructure before current full mutation evidence passes.
