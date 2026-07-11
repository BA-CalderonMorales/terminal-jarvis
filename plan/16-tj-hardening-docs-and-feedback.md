---
id: "16"
target: v0.1.13
title: Documentation and Feedback
status: proposed
owner: documentation-owner
reviewer: product-support-reviewer
delivery_mode: pending-page-12-decision
depends_on: ["05", "07", "09", "14", "15"]
blocks: ["17"]
---

# 16 - Documentation and Feedback

## Objective

Make support boundaries, install behavior, diagnostics, safe testing, demo
simulation, privacy, cost fallback, and recovery understandable without
overclaiming third-party harness compatibility.

## Audiences

- First-time user installing through Cargo, npm/npx, Homebrew, or direct asset.
- Existing user diagnosing PATH, cache, catalog, environment, or update issues.
- User evaluating Terminal Jarvis through required static playback or a selected
  Killercoda, Codespaces, or hosted option.
- Harness maintainer updating support metadata and conformance evidence.
- Release/operator owner deploying, disabling, or recovering the showcase.

## Work

- [ ] Replace `delivery_mode: pending-page-12-decision` with the page 12 value
  and make every claim/runbook follow that selected mode.
- [ ] Update README quick start and safe-testing guidance with the support-state vocabulary.
- [ ] Generate or verify supported-agent/platform tables from catalog metadata.
- [ ] Document diagnostics/support-bundle usage and privacy guarantees.
- [ ] Document install/update parity, PATH conflicts, checksum behavior, and recovery.
- [ ] Document demo command limits, simulation, no-credential policy, session expiry,
  data handling, accessibility, and zero-cost alternatives.
- [ ] Document `zero-host` as the default; identify each optional surface as
  `external-free-tier`, `user-metered`, or `maintainer-metered`, including who
  owns quota/billing and how to opt out.
- [ ] Audit any existing README Codespaces recommendation: remove it when
  unselected, or label it optional/user-metered with no maintainer sponsorship.
- [ ] Document provider architecture only after page 12 selection; avoid presenting
  rejected experiments as supported infrastructure.
- [ ] For hosted mode, document kill switch, budget response, key rotation,
  cleanup, outage, rollback, and provider exit. For zero-host, document static
  rollback, scenario/link disablement, content drift, and absence of custom compute.
- [ ] Document manifest mismatch, missing adapter/credential, expired opt-in,
  quota/budget exhaustion, provider outage, ambiguous cleanup, and the invariant
  that each resolves to static zero-host rather than another provider.
- [ ] Add troubleshooting keyed by stable diagnostic/error codes.
- [ ] Add a privacy-safe feedback path that can include the redacted support bundle.
- [ ] Update changelog/release notes only after behavior and target version are final.
- [ ] Run command/link/asset/version checks against local docs and every selected
  external showcase source; unselected sources must have no public link.

## Likely Areas

`README.md`, `docs/`, npm README and binary notice, Homebrew README, release notes,
external scenario content, static player assets, and operator runbooks.

## Acceptance Criteria

- [ ] `DOC-01` Install and first-run instructions are verified for every supported
  channel using nonpublishing staged artifacts before live registry publication.
- [ ] `DOC-02` Support tables match generated metadata and expose freshness/state.
- [ ] `DOC-03` Unsupported, expected, stubbed, simulated, and dangerous behavior is explicit.
- [ ] `DOC-04` Diagnostics and support bundles are documented without secret-handling ambiguity.
- [ ] `DOC-05` Every showcase path states limits, data handling, expiry, and fallback.
- [ ] `DOC-06` Operator runbooks cover every control and recovery action in the selected mode.
- [ ] `DOC-07` Accessibility review covers terminal and static/guided content.
- [ ] `DOC-08` Commands, links, versions, assets, and selected external scenario
  content pass drift checks; unselected surfaces have no active link.
- [ ] `DOC-09` Every optional surface states its cost class, human opt-in,
  billing owner, disable path, and static fallback without implying paid failover.

## Evidence

| Criterion | Method | Artifact/URL | Ref | UTC | Result | Reviewer |
|---|---|---|---|---|---|---|
| DOC-01 | fresh install walkthroughs | pending | pending | pending | pending | pending |
| DOC-02 | generated diff | pending | pending | pending | pending | pending |
| DOC-03 | terminology review | pending | pending | pending | pending | pending |
| DOC-04 | privacy/support review | pending | pending | pending | pending | pending |
| DOC-05 | showcase content review | pending | pending | pending | pending | pending |
| DOC-06 | runbook drills | pending | pending | pending | pending | pending |
| DOC-07 | accessibility audit | pending | pending | pending | pending | pending |
| DOC-08 | docs/drift workflow | pending | pending | pending | pending | pending |
| DOC-09 | cost/opt-in/fallback content audit | pending | pending | pending | pending | pending |

## Risks and Rollback

- Risk: hand-maintained support tables drift. Generate from reviewed metadata.
- Risk: docs encourage credentials in public sandboxes. Repeat the prohibition at
  every entry point and in the terminal banner.
- Rollback trigger: instructions are unverified or contradict runtime behavior.
- Rollback action: remove the affected claim/link and point users to the stable
  v0.1.12 recovery path until corrected.

## Completion Gate

Complete only after fresh-user, maintainer, accessibility, security, and operator
reviews pass and docs-only CI is explicitly dispatched. Unselected external
surfaces require decision records and no public link; they do not block the
required local static documentation path.
