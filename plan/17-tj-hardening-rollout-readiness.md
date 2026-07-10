---
id: "17"
target: v0.1.13
title: Rollout and Release Readiness
status: proposed
owner: release-owner
reviewer: independent-release-reviewer
delivery_mode: pending-page-12-decision
depends_on: ["01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12", "13", "14", "15", "16"]
blocks: ["index-completion"]
---

# 17 - Rollout and Release Readiness

## Objective

Prove the complete change set is safe to promote as the v0.1.13 release
candidate, rehearse staged rollout and rollback, align version metadata, and
authorize the index to complete. This page does not itself tag or publish.

## Preconditions

- Every page 01-16 is `complete` with current reviewed evidence.
- The provider outcome from page 12 is implemented exactly, including a valid
  zero-host outcome.
- `delivery_mode` matches the non-pending value bound by page 12.
- No unresolved blocker, placeholder owner, stale evidence, or unapproved scope
  change remains.
- The working tree and branch history contain only intended v0.1.13 work.

## Hosted-Mode Rollout Stages

1. Fixture-only local and deterministic CI validation.
2. Protected hosted staging using synthetic traffic.
3. Internal/invite-only walkthrough with kill switch armed.
4. Limited public canary within approved concurrency and budget.
5. General public link only after canary thresholds pass.
6. Package release candidate approval; merge/tag/publish remains a separate operator action.

## Zero-Host Rollout Stages

1. Fixture-only local and deterministic CI validation.
2. Static candidate preview with final cast, transcript, links, and version manifest.
3. Private/unlisted Killercoda scenario walkthrough and Codespaces link review.
4. Limited documentation-link canary with drift, error, and feedback monitoring.
5. Public static/scenario links after canary thresholds pass.
6. Remove/restore links and manifests to rehearse rollback; verify no provider
   secrets, deployments, sessions, or custom compute costs exist.
7. Package release candidate approval; merge/tag/publish remains a separate operator action.

## Work

- [ ] Replace `delivery_mode: pending-page-12-decision` with the page 12 value
  and audit pages 13-16 for the same value.
- [ ] Audit every child status and evidence row against the final candidate ref.
- [ ] Resolve all page/index registry mismatches and stale evidence.
- [ ] Rehearse the exact hosted-mode or zero-host rollout, abort, fallback, and rollback sequence.
- [ ] Run representative install, diagnostics, support, conformance, distribution,
  fixture, showcase, security, and selected-mode cost/absence and cleanup journeys.
- [ ] Align `Cargo.toml`, Cargo lock metadata where applicable, npm package and
  lock versions, formula/version references, and `CHANGELOG.md` to `0.1.13`.
- [ ] Verify release notes describe behavior, limits, provider outcome, and recovery.
- [ ] Run the full local gates without tagging, pushing, publishing, or uploading.
- [ ] Obtain CI, security, distribution, documentation, FinOps/platform, and release approvals.
- [ ] Record the exact candidate commit proposed for merge to `develop`/`main`.
- [ ] Confirm the release operator understands that tag creation/push is still separate.

## Required Verification

Commands may be adjusted only when the corresponding script contract changes;
record exact commands and outputs in evidence.

```sh
scripts/verify.sh
scripts/local-ci.sh --strict --mutation
scripts/package-release.sh --check
scripts/package-release.sh build /tmp/terminal-jarvis-0.1.13
scripts/local-cd.sh --out-dir /tmp/terminal-jarvis-0.1.13-cd
git diff --check
git status --short
```

Run hosted native workflows for Linux x64/ARM64, macOS x64/ARM64, and Windows
x64. The tag-aware preflight can be fully satisfied only when a candidate tag
exists at expected `main`; before operator tagging, record metadata preflight
plus the exact proposed tag target, then rerun the tag-aware gate during release.

## Acceptance Criteria

- [ ] `REL-01` All child pages and index registry statuses agree and are complete.
- [ ] `REL-02` Full local verification, strict security, mutation, package, and local-CD gates pass.
- [ ] `REL-03` Hosted native CI and nonpublishing integration jobs pass on the candidate ref.
- [ ] `REL-04` Version metadata and changelog agree on `0.1.13`.
- [ ] `REL-05` Selected-mode canary and abort thresholds pass; its rollback is rehearsed.
- [ ] `REL-06` Selected-mode security, privacy, terms, cost/absence, accessibility,
  and operations approvals are recorded.
- [ ] `REL-07` Previous `v0.1.12` package and showcase recovery paths remain usable.
- [ ] `REL-08` Exact proposed merge and tag commit is recorded; no tag has been pushed by this page.
- [ ] `REL-09` Release owner and independent reviewer authorize index completion.

## Evidence

| Criterion | Command/workflow | Artifact/URL | Ref | UTC | Result | Reviewer |
|---|---|---|---|---|---|---|
| REL-01 | plan status audit | pending | pending | pending | pending | pending |
| REL-02 | local gate transcript | pending | pending | pending | pending | pending |
| REL-03 | hosted workflow runs | pending | pending | pending | pending | pending |
| REL-04 | release preflight metadata | pending | pending | pending | pending | pending |
| REL-05 | selected-mode canary/rollback report | pending | pending | pending | pending | pending |
| REL-06 | approval record | pending | pending | pending | pending | pending |
| REL-07 | recovery walkthrough | pending | pending | pending | pending | pending |
| REL-08 | proposed commit/tag record | pending | pending | pending | pending | pending |
| REL-09 | signed completion approval | pending | pending | pending | pending | pending |

## Abort and Rollback

Abort the candidate for any failed hard gate, unexplained regression, secret
exposure, command escape, unsupported support claim, distribution mismatch,
uncontrolled cost, orphan breach, inaccessible critical journey, provider terms
change, or failed recovery rehearsal.

Rollback actions:

1. Hosted: disable session creation; zero-host: remove/redirect affected public links.
2. Restore the previous selected-mode manifest/fixture/content.
3. Hosted: revoke keys and delete sessions; zero-host: verify no such resources exist.
4. Stop release promotion; do not create or push `v0.1.13`.
5. Keep v0.1.12 as the documented stable recovery target.
6. Return affected pages and the index to `in-progress` with the failure evidence.

## Completion Gate

When and only when `REL-01` through `REL-09` pass, mark this page `complete`,
update every registry row and master checkbox in `plan/index.md`, reproduce the
index completion algorithm, record both approvals, and set the index status to
`complete`. Merge, tag, push, and publish still require explicit operator action.
