---
id: "06"
target: v0.1.13
title: Harness Conformance
status: proposed
owner: integration-test-owner
reviewer: security-reviewer
depends_on: ["04", "05"]
blocks: ["07", "08"]
---

# 06 - Harness Conformance

## Objective

Verify every catalog claim through secret-free deterministic fixtures, then add
bounded upstream smoke checks for the promoted harnesses without running agents
or destructive lifecycle commands on developer machines.

## Test Tiers

1. Schema and command construction for all 25 harnesses and 9 capabilities.
2. Fake executable lifecycle tests for execution, stdin/stdout/stderr, exit codes,
   missing binary, missing environment, signals, and policy denial.
3. Packaged-binary core command matrix with embedded and external catalogs.
4. Optional external no-auth smoke tests for a reviewed promoted subset using
   only disposable standard CI and help/version/read-only invocations. Record
   `not-selected` when safe acquisition or authority is unavailable; never turn
   missing upstream smoke into a `tested` catalog claim.

## Work

- [ ] Build catalog-derived fixtures rather than hand-maintaining duplicate lists.
- [ ] Reuse and centralize current fake executable patterns.
- [ ] Exercise every support state and side-effect class from page 05.
- [ ] Assert download, update, UI, headless, and yolo commands are never invoked
  accidentally by ordinary conformance jobs.
- [ ] Test child exit code, stderr context, signal/timeout handling, and cleanup.
- [ ] Record upstream smoke as `not-selected`, or add a reviewed allowlist for
  real no-auth commands and acquisition methods.
- [ ] If selected, schedule upstream smoke separately from blocking deterministic
  CI with no provider credentials, paid runners, state-changing hooks, or agent execution.
- [ ] Define quarantine, owner notification, and freshness behavior for upstream drift.
- [ ] Produce a machine-readable conformance report consumed by docs and release gates.

## Likely Areas

`scripts/integration-hardening.sh`, `scripts/core-command-matrix.sh`, tests under
`tests/`, `src/runtime/`, `src/cli/invoke.rs`, `src/cli/guard.rs`, and hosted CI.

## Acceptance Criteria

- [ ] `CON-01` Every catalog row is covered by a deterministic test classification.
- [ ] `CON-02` Fixtures require no network, credentials, or user-level configuration.
- [ ] `CON-03` Child failures include harness, capability, exit, and stderr context.
- [ ] `CON-04` Dangerous and unsupported paths fail closed in positive and negative tests.
- [ ] `CON-05` Upstream smoke has reviewed `not-selected` evidence, or every
  promoted command/acquisition path is reviewed, read-only, disposable, and non-billable.
- [ ] `CON-06` Upstream drift cannot silently downgrade blocking deterministic CI.
- [ ] `CON-07` The report records tested ref, platform, result, and freshness.

## Evidence

| Criterion | Command/workflow | Artifact | Ref | UTC | Result | Reviewer |
|---|---|---|---|---|---|---|
| CON-01 | coverage report | pending | pending | pending | pending | pending |
| CON-02 | offline fixture run | pending | pending | pending | pending | pending |
| CON-03 | failure matrix | pending | pending | pending | pending | pending |
| CON-04 | policy negative tests | pending | pending | pending | pending | pending |
| CON-05 | smoke selection/allowlist review | pending | pending | pending | pending | pending |
| CON-06 | simulated upstream failure | pending | pending | pending | pending | pending |
| CON-07 | report schema validation | pending | pending | pending | pending | pending |

## Risks and Rollback

- Risk: upstream installs consume CI time or execute unexpected hooks. Pin reviewed
  acquisition methods, isolate jobs, and never provide provider secrets.
- Risk: fake binaries overstate real compatibility. Label deterministic versus
  upstream evidence separately.
- Rollback trigger: any smoke test performs an unapproved network or agent action.
- Rollback action: disable the upstream job and preserve deterministic coverage.

## Completion Gate

Complete when all catalog rows have deterministic evidence and upstream smoke
has a reviewed selection record. Any selected allowlist needs security approval;
external CLI execution is not required for zero-host or honest `unknown` claims.
