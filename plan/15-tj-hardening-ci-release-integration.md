---
id: "15"
target: v0.1.13
title: CI and Release Integration
status: proposed
owner: ci-release-owner
reviewer: security-permissions-reviewer
delivery_mode: pending-page-12-decision
depends_on: ["07", "08", "13", "14"]
blocks: ["16", "17"]
---

# 15 - CI and Release Integration

## Objective

Make product contracts, fixture integrity, distribution parity, showcase
promotion, and rollback verifiable through hosted automation without weakening
the existing operator-controlled tag release boundary.

## Work

- [ ] Replace `delivery_mode: pending-page-12-decision` with the page 12 value
  and record unselected alternatives as `not-selected`.
- [ ] Add deterministic UX, diagnostics, support-matrix, conformance, fixture,
  and policy tests to the appropriate blocking CI jobs.
- [ ] Add native install/package smoke coverage from page 07 without publishing.
- [ ] Validate workflow syntax, permissions, concurrency, timeouts, and artifact retention.
- [ ] Generate a versioned demo manifest containing release tag, commit, asset URL,
  checksum, fixture version, protocol version, and rollback version.
- [ ] Promote the selected delivery manifest only after required assets/checksums
  exist and its hosted or zero-host candidate walkthrough passes.
- [ ] Ensure hosted deployment or zero-host static/scenario content can roll back
  independently to the prior manifest.
- [ ] Refresh static recording and scenario version through reviewed automation,
  while detecting stale external scenario content.
- [ ] Keep provider credentials out of pull-request contexts and fork-accessible
  jobs in hosted mode; prove no provider credentials/jobs exist in zero-host mode.
- [ ] Add a plan-status verification that checks child/index status consistency
  without treating checkboxes alone as implementation evidence.
- [ ] Preserve release preflight requirements that tag, Cargo, npm, changelog,
  HEAD, and expected main ref agree.

## Workflow Boundaries

- Pull request workflows: read-only, deterministic, no provider mutation.
- Scheduled upstream smoke: disposable, no provider API keys for coding agents.
- Hosted mode: protected staging environment and scoped sandbox credentials.
- Zero-host mode: static preview plus private/unlisted guided-scenario review,
  with no provider credential or sandbox deployment job.
- Tag CD: existing package/crate/release/Homebrew/npm publication order.
- Demo promotion: protected, reversible, and never a prerequisite for local CLI use.

## Acceptance Criteria

- [ ] `CIC-01` All new deterministic gates block regressions on pull requests.
- [ ] `CIC-02` Native install smoke jobs cannot publish or mutate registries/taps.
- [ ] `CIC-03` Demo manifests are generated from and pinned to verified release artifacts.
- [ ] `CIC-04` Stale, mismatched, or corrupt fixture/showcase artifacts fail promotion.
- [ ] `CIC-05` Hosted secrets are protected; zero-host proves none are configured.
- [ ] `CIC-06` Selected delivery rollback works without changing package registry versions.
- [ ] `CIC-07` Existing release preflight and operator approval are not weakened.
- [ ] `CIC-08` Docs-only workflow skips are compensated by explicit dispatch evidence.

## Evidence

| Criterion | Command/workflow | Artifact/URL | Ref | UTC | Result | Reviewer |
|---|---|---|---|---|---|---|
| CIC-01 | PR workflow run | pending | pending | pending | pending | pending |
| CIC-02 | permission/side-effect audit | pending | pending | pending | pending | pending |
| CIC-03 | manifest verification | pending | pending | pending | pending | pending |
| CIC-04 | stale/corrupt promotion tests | pending | pending | pending | pending | pending |
| CIC-05 | protected-secret or secret-absence review | pending | pending | pending | pending | pending |
| CIC-06 | selected delivery rollback drill | pending | pending | pending | pending | pending |
| CIC-07 | release preflight tests | pending | pending | pending | pending | pending |
| CIC-08 | workflow_dispatch run | pending | pending | pending | pending | pending |

## Risks and Rollback

- Risk: hosted checks increase CI cost. Separate deterministic blockers from
  scheduled/provider tests and enforce time/concurrency limits.
- Risk: demo deployment becomes coupled to package publishing. Keep manifests
  and provider promotion independently reversible.
- Rollback trigger: workflow gains unintended write access or release ordering regresses.
- Rollback action: disable affected workflow, preserve existing tag CD, and keep
  the previous demo manifest active.

## Completion Gate

Complete only after selected-mode staging/preview, permissions audit,
stale-artifact tests, and rollback drill pass on the final implementation ref.
Zero-host requires explicit evidence that no provider workflow or secret exists.
