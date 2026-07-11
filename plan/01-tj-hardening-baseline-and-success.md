---
id: "01"
target: v0.1.13
title: Baseline and Success Measures
status: proposed
owner: release-owner
reviewer: product-governance-reviewer
depends_on: []
blocks: ["02"]
---

# 01 - Baseline and Success Measures

## Objective

Freeze an evidence-backed v0.1.12 baseline, define the user journeys and
consistency dimensions v0.1.13 will improve, assign owners, and approve hard
success, cost, and safety thresholds before implementation begins.

## Current Baseline to Reconfirm

- Branch base: `develop` at `f0d4612` when this plan was authored.
- Last release: `v0.1.12` at `9b9f720`.
- Product metadata on this branch remains `0.1.12` intentionally.
- Catalog: 25 harnesses with 9 capabilities each.
- Release platforms: Linux x64/ARM64 GNU, macOS x64/ARM64, Windows x64.
- Install channels: Cargo, npm/npx, Homebrew, and direct release assets.
- Existing UX assets: rich tables, `--plain`, `--no-color`, width-aware output,
  core command matrix, fake executable tests, and release preflight.
- Open integration tracker: GitHub issue #135.

These statements are context, not completion evidence. Reproduce them against
the execution ref.

## Role Assignment Record

Role names are accountable placeholders for planning structure. A named person
must accept each role before this page can leave `proposed`; one person may hold
multiple owner roles, but a page reviewer must be different from its owner.
These rows do not require 34 different people. Luna agents may execute bounded
tasks and collect evidence, but they cannot occupy human approval fields or
authorize spend, terms, publication, or release. The zero-host lane needs no
provider account owner or provider-terms approver.

| Page | Owner role | Named owner | Reviewer role | Named reviewer |
|---|---|---|---|---|
| 01 | release-owner | pending | product-governance-reviewer | pending |
| 02 | cli-ux-owner | pending | automation-compatibility-reviewer | pending |
| 03 | diagnostics-owner | pending | security-privacy-reviewer | pending |
| 04 | cli-presentation-owner | pending | cross-platform-reviewer | pending |
| 05 | catalog-owner | pending | harness-maintainer-reviewer | pending |
| 06 | integration-test-owner | pending | security-reviewer | pending |
| 07 | distribution-owner | pending | release-engineering-reviewer | pending |
| 08 | demo-fixture-owner | pending | security-reviewer | pending |
| 09 | docs-demo-owner | pending | accessibility-reviewer | pending |
| 10 | platform-evaluation-owner | pending | security-finops-reviewer | pending |
| 11 | kernel-poc-owner | pending | independent-platform-reviewer | pending |
| 12 | release-owner | pending | security-finops-reviewer | pending |
| 13 | demo-platform-owner | pending | security-accessibility-reviewer | pending |
| 14 | security-finops-owner | pending | independent-operations-reviewer | pending |
| 15 | ci-release-owner | pending | security-permissions-reviewer | pending |
| 16 | documentation-owner | pending | product-support-reviewer | pending |
| 17 | release-owner | pending | independent-release-reviewer | pending |

## Work

- [ ] Bind one named human to each owner/reviewer role in both the table and
  `plan/ownership.json`; record rollback owners and keep each page's people distinct.
- [ ] Inventory user journeys: install, first run, inspect, select, check, plan,
  execute, update, diagnose, recover, and try the required/selected showcase surfaces.
- [ ] Define the consistency matrix across OS, architecture, shell, install
  channel, TTY/non-TTY, color mode, terminal width, and harness support level.
- [ ] Capture baseline rich/plain/error outputs at 40, 80, and 120 columns.
- [ ] Record baseline install/update behavior for each supported distribution.
- [ ] Record known unsupported, untested, stubbed, and dangerous paths.
- [ ] Define success measures and the exact collection method for each.
- [ ] Approve `zero-host` and USD 0 maintainer spend as the defaults; record any
  provider, publication, user-metered, or nonstandard-CI opt-in separately using
  the index schema.
- [ ] Freeze exact denominators for every matrix, catalog, command, distribution,
  viewport, and external-surface claim so smaller agents cannot redefine "all."
- [ ] Approve the non-goals in `plan/index.md`.
- [ ] Record full and skipped baseline checks with reasons.

## Required Success Measures

The owner must replace `pending` thresholds before this page can be ready.

| Measure | Required threshold | Collection method |
|---|---|---|
| Core command contract | 100% expected commands pass | core command matrix |
| Harness metadata coverage | 100% of harness/capability rows classified | generated report |
| Supported distribution smoke | 100% supported matrix rows pass | standard native CI/local jobs |
| Rich output overflow | 0 lines exceed declared width | presentation tests |
| Plain output compatibility | 0 unapproved schema changes | golden fixtures |
| Diagnostic redaction | 0 seeded secrets emitted | adversarial tests |
| Demo command policy | 0 denied command escapes | policy tests |
| Static showcase readiness | local artifact opens and canonical walkthrough completes | timed local/loopback sample |
| Maintainer demo spend | USD 0 in zero-host; explicit approved ceiling otherwise | billing/resource absence or usage evidence |
| Provider session/orphan count | 0 in zero-host; 0 after hosted reconciliation | resource inventory/cleanup audit |

## Acceptance Criteria

- [ ] `BAS-01` Baseline ref, release ref, matrix, and inventory are recorded.
- [ ] `BAS-02` All user journeys have an owner and measurable expected outcome,
  and `plan/ownership.json` matches every page role with distinct named humans.
- [ ] `BAS-03` Success and abort thresholds are numeric and approved.
- [ ] `BAS-04` Scope, non-goals, USD 0 default budget, opt-in ceilings, and
  secrets policy are approved.
- [ ] `BAS-05` Baseline checks distinguish passed, failed, and skipped work.
- [ ] `BAS-06` Issue #135 is mapped to pages in this plan without losing open work.

## Evidence

| Criterion | Command or method | Artifact/URL | Ref | UTC | Result | Reviewer |
|---|---|---|---|---|---|---|
| BAS-01 | pending | pending | pending | pending | pending | pending |
| BAS-02 | pending | pending | pending | pending | pending | pending |
| BAS-03 | pending | pending | pending | pending | pending | pending |
| BAS-04 | pending | pending | pending | pending | pending | pending |
| BAS-05 | pending | pending | pending | pending | pending | pending |
| BAS-06 | pending | pending | pending | pending | pending | pending |

## Risks and Rollback

- Risk: broad goals create feature sprawl. Mitigation: freeze journeys and
  require an index decision for scope changes.
- Risk: hosted-demo work masks product defects. Mitigation: product metrics are
  gates before provider work.
- Risk: agents wait on unnecessary provider or publication authority.
  Mitigation: missing opt-ins close those paths as `not-selected`; zero-host continues.
- Rollback trigger: an approved threshold or owner is missing.
- Rollback action: return this page to `proposed`; do not start page 02.
- Recovery target: unchanged v0.1.12 behavior and current `develop` base.

## Completion Gate

This page is complete only when all criteria have reviewed evidence, all owners
are assigned, and the release owner approves `G0` in the index.
