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
- [ ] Validate the page 10 manifest schema and exact selected-mode/protected-input
  agreement before promotion; malformed hosted input preserves zero-host.
- [ ] Promote the selected delivery manifest only after required assets/checksums
  exist and its hosted or zero-host candidate walkthrough passes.
- [ ] Ensure hosted deployment or zero-host static/scenario content can roll back
  independently to the prior manifest.
- [ ] Refresh static recording and scenario version through reviewed automation,
  while detecting stale external scenario content.
- [ ] Keep provider credentials out of pull-request contexts and fork-accessible
  jobs in hosted mode; prove no provider credentials/jobs exist in zero-host mode.
- [ ] Make every provider POC/deploy job `workflow_dispatch`-only, protected by a
  reviewed environment and `OI-PROVIDER`, with explicit mode, opt-in ID, budget,
  expiry, and billable-use confirmation inputs. Do not schedule provider jobs.
- [ ] Scan the zero-host artifact/dependency/route/workflow inventory for provider
  SDKs, config/secret names, broker routes, deployment bindings, and executable endpoints.
- [ ] Validate `demo/surfaces.json` and workflow runner/storage/service cost
  classifications; fail any active external surface or metered CI job without
  the matching unexpired secret-free opt-in record.
- [ ] Add matrix cases for no adapters, each adapter removed, malformed/stale
  manifests, missing/expired opt-in, missing credentials, disabled kill switch,
  provider API outage, and no-cross-provider failover.
- [ ] Add a plan-status verification that checks child/index status consistency
  without treating checkboxes alone as implementation evidence.
- [ ] Pin a reviewed `cargo-mutants` version and retain each run's compressed
  `mutants.out` even on failure; record tool, Rust, runner, and lockfile identity.
  Default CI retention is three days with a 100 MiB pre-upload ceiling. If the
  current storage policy would bill or the ceiling is exceeded, fail before
  upload and use reviewed local release evidence unless `OI-RELEASE-CI` exists.
- [ ] Split mutation feedback into a production-diff PR tier and a mandatory,
  explicit release-candidate full tier without broadening `mutants.toml`
  exclusions. Leave recurring full-suite scheduling disabled by default.
- [ ] Benchmark mutant count, baseline duration, `--jobs` values, and identical
  2/4-way shards before changing parallelism or the 30-second timeout floor.
- [ ] After behavior freezes, align Cargo/npm/lock/formula/changelog metadata to
  `0.1.13` before final candidate evidence is collected on pages 16-17.
- [ ] Preserve release preflight requirements that tag, Cargo, npm, changelog,
  HEAD, and expected main ref agree.

## Mutation Gate Contract

The current CI runs the complete mutation set on every non-docs pull request
with `--jobs 2`, a 30-second minimum timeout, no artifacts, and a 25-minute job
limit. The `mutation_target=90` variable in `scripts/verify.sh` is descriptive;
it is not currently passed to `cargo-mutants`. Do not claim a 90% threshold
until the workflow enforces and reports one.

Future execution should use cargo-mutants' diff input for fast pull-request
feedback on changed production lines. A no-mutants result is not proof that the
complete suite passed, and test-only changes receive mutation evidence from an
explicit full-tier dispatch rather than a fabricated diff result. The complete
tier remains mandatory before release and runs by explicit release-candidate
dispatch. Recurring scheduling is optional and default-off; enabling it requires
a reviewed cadence, runner/storage cost classification, retention cap, and stop
condition. The complete tier may shard one identical deterministic mutant list
across standard runners only after a normal baseline test passes; every shard
must pass and retain its separate bounded output artifact.

Use `--in-place` only in disposable CI checkouts and do not combine it with
local `--jobs`. Keep the existing narrow, explained exclusions until targeted
mutation listing and replacement tests justify removing one. Do not lower
timeouts, switch to nextest, increase local jobs, or choose a shard count from
intuition; benchmark those choices against missed, timeout, unviable, runtime,
and runner-cost results first.

Mutation optimization is not permission to weaken the v0.1.13 gate. Preserving
the existing complete mutation command with a reviewed `redesign-not-selected`
record is valid for this release; workflow redesign may move to a later release.

Revalidate the exact flags against the pinned release before implementation:

- [cargo-mutants CI guidance](https://mutants.rs/ci.html)
- [diff-scoped mutation](https://mutants.rs/pr-diff.html)
- [sharding](https://mutants.rs/shards.html)
- [timeouts](https://mutants.rs/timeouts.html)
- [output artifacts](https://mutants.rs/mutants-out.html)

## Workflow Boundaries

- Pull request workflows: read-only, deterministic, no provider mutation.
- Scheduled upstream smoke: disposable, no provider API keys for coding agents.
- Hosted mode: manual protected staging environment, scoped sandbox credentials,
  exact opt-in inputs, and no automatic production/public promotion.
- Zero-host mode: required local static preview with no provider credential or
  sandbox deployment job; private/unlisted guided scenarios are included only
  when selected by `OI-PUBLISH`.
- Public-repository standard GitHub runners may provide non-billable release
  evidence under current terms. Larger runners, metered services, or paid
  artifact retention require `OI-RELEASE-CI`; a billing change never authorizes spend.
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
- [ ] `CIC-09` Mutation PR feedback is bounded while a complete, artifact-backed
  mutation run remains a release gate with no unreviewed exclusion expansion.
- [ ] `CIC-10` CI proves default zero-host, exact hosted opt-in, adapter removal,
  secret absence, malformed-config rejection, and no paid-to-paid failover.
- [ ] `CIC-11` Version metadata is aligned before the final evidence ref is frozen.

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
| CIC-09 | diff/full mutation workflow evidence | pending | pending | pending | pending | pending |
| CIC-10 | mode/adapter/secret failure matrix | pending | pending | pending | pending | pending |
| CIC-11 | release metadata ordering audit | pending | pending | pending | pending | pending |

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
Zero-host requires explicit evidence that no provider workflow, SDK, route,
deployment binding, account, or secret exists. Provider staging is manual and
private; external publication and tag release remain separate operator actions.
