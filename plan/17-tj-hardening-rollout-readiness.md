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
candidate, rehearse staged rollout and rollback, verify aligned metadata, and
authorize the index to complete. This page does not itself tag or publish.

## Preconditions

- Every page 01-16 is `complete` with current reviewed evidence.
- The provider outcome from page 12 is implemented exactly, including a valid
  zero-host outcome.
- `delivery_mode` matches the non-pending value bound by page 12.
- Cargo/npm/lock/formula/changelog metadata already agrees on `0.1.13`; this
  page collects evidence after that candidate metadata commit.
- No unresolved blocker, placeholder owner, stale evidence, or unapproved scope
  change remains.
- The working tree and branch history contain only intended v0.1.13 work.

## Hosted-Mode Rollout Stages

1. Build and verify the zero-host candidate and rollback manifest first; publish
   it only with `OI-PUBLISH`.
2. Revalidate `OI-PROVIDER`, exact manifest/protected-input agreement, budget,
   credentials, kill switch, and cleanup ownership.
3. Protected hosted staging using synthetic traffic.
4. Internal/invite-only walkthrough with kill switch armed.
5. Limited public canary within approved concurrency and budget.
6. General public link only after canary thresholds and `OI-PUBLISH` pass.
7. Package release candidate approval; merge/tag/publish remains a separate operator action.

## Zero-Host Rollout Stages

1. Fixture-only local and deterministic CI validation.
2. Self-contained local/loopback candidate preview with final cast, transcript,
   checksums, and `provider: none` version manifest.
3. Prove the walkthrough with Pages, Killercoda, Codespaces, analytics, network,
   provider APIs, credentials, and both adapters unavailable.
4. If `OI-PUBLISH` exists, preview only the selected external static/scenario links.
5. Remove/restore optional links and manifests to rehearse rollback; verify no provider
   secrets, deployments, sessions, or custom compute references exist in the
   candidate, and obtain release-owner attestation that no v0.1.13 external
   resource/billing configuration was created.
6. Package release candidate approval; merge/tag/publish remains a separate operator action.

## Work

- [ ] Replace `delivery_mode: pending-page-12-decision` with the page 12 value
  and audit pages 13-16 for the same value.
- [ ] Audit every child status and evidence row against the final candidate ref.
- [ ] Resolve all page/index registry mismatches and stale evidence.
- [ ] Rehearse the exact hosted-mode or zero-host rollout, abort, fallback, and rollback sequence.
- [ ] Run representative install, diagnostics, support, conformance, distribution,
  fixture, showcase, security, and selected-mode cost/absence and cleanup journeys.
- [ ] Confirm page 15 aligned `Cargo.toml`, Cargo lock metadata where applicable,
  npm package/lock, formula/version references, and `CHANGELOG.md` before this
  page's evidence collection began.
- [ ] Verify release notes describe behavior, limits, provider outcome, and recovery.
- [ ] Run all required local or equivalent non-billable hosted gates without
  tagging, publishing, or uploading release assets.
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

The strict local command and the complete mutation result may be supplied by an
equivalent successful public-repository standard-runner workflow when local
tools are unavailable. Required checks are never silently skipped. A billing or
runner-policy change blocks the release until a human supplies local evidence or
approves `OI-RELEASE-CI`; it never permits automatic spend or weaker evidence.

Every target below is mandatory. Accepted evidence is exactly either a
successful, nonpublishing standard-runner workflow URL and named job at the
candidate ref, or a committed local transcript produced on a host with the
matching OS and architecture. Cross-compilation and package-shape inspection do
not count as native execution.

| Required target | Matching native execution |
|---|---|
| `linux-x64-gnu` | Linux x86_64 standard public-repository runner or matching local host |
| `linux-arm64-gnu` | Linux AArch64 standard public-repository runner or matching local host |
| `macos-x64` | macOS x86_64 standard public-repository runner or matching local host |
| `macos-arm64` | macOS arm64 standard public-repository runner or matching local host |
| `win32-x64` | Windows x86_64 standard public-repository runner or matching local host |

If standard public-repository runner terms stop being non-billable, the row
blocks until matching local evidence exists or a human authorizes
`OI-RELEASE-CI`; the workflow never opts into a larger/metered runner itself.
The tag-aware preflight can be fully satisfied only when a candidate tag exists
at expected `main`; before operator tagging, record metadata preflight plus the
exact proposed tag target, then rerun the tag-aware gate during release.

## Acceptance Criteria

For `REL-05`, a zero-host canary is the local/loopback static walkthrough,
offline/corrupt-asset failure matrix, optional-link disable test, and manifest
rollback. It does not require public traffic, a provider session, Killercoda, or
Codespaces. A criterion uses `not-selected-zero-host` only when it explicitly
requires evidence for an unselected provider outcome; common criteria require
positive evidence.

- [ ] `REL-01` All child pages and index registry statuses agree and are complete.
- [ ] `REL-02` Full local verification, strict security, mutation, package, and local-CD gates pass.
- [ ] `REL-03` Standard native CI or equivalent nonpublishing native evidence
  passes on the candidate ref without an unapproved metered runner/service.
- [ ] `REL-04` Version metadata and changelog agree on `0.1.13`.
- [ ] `REL-05` Selected-mode canary and abort thresholds pass; its rollback is rehearsed.
- [ ] `REL-06` Selected-mode security, privacy, terms, cost/absence, accessibility,
  and operations approvals are recorded.
- [ ] `REL-07` Previous `v0.1.12` package and showcase recovery paths remain usable.
- [ ] `REL-08` Exact proposed merge and tag commit is recorded; no tag has been pushed by this page.
- [ ] `REL-09` Release owner and independent reviewer authorize index completion.
- [ ] `REL-10` Missing/corrupt manifest, missing adapter/credential, expired
  opt-in, provider outage/quota/budget stop, and uncertain cleanup all preserve
  or restore zero-host without invoking another provider.

## Evidence

| Criterion | Command/workflow | Artifact/URL | Ref | UTC | Result | Reviewer |
|---|---|---|---|---|---|---|
| REL-01 | plan status audit | pending | pending | pending | pending | pending |
| REL-02 | local gate transcript | pending | pending | pending | pending | pending |
| REL-03 | native workflow/local evidence | pending | pending | pending | pending | pending |
| REL-04 | release preflight metadata | pending | pending | pending | pending | pending |
| REL-05 | selected-mode canary/rollback report | pending | pending | pending | pending | pending |
| REL-06 | approval record | pending | pending | pending | pending | pending |
| REL-07 | recovery walkthrough | pending | pending | pending | pending | pending |
| REL-08 | proposed commit/tag record | pending | pending | pending | pending | pending |
| REL-09 | signed completion approval | pending | pending | pending | pending | pending |
| REL-10 | default/adapter/outage/rollback failure matrix | pending | pending | pending | pending | pending |

## Abort and Rollback

Abort the candidate for any failed hard gate, unexplained regression, secret
exposure, command escape, unsupported support claim, distribution mismatch,
uncontrolled cost, orphan breach, inaccessible critical journey, provider terms
change, or failed recovery rehearsal.

Rollback actions:

1. Atomically restore the verified `provider: none` manifest/static routes and
   disable hosted creates/retries or affected optional external links.
2. Reconcile/delete selected-provider resources by inventory/idempotency record
   with the bounded cleanup-only credential; do not create or start another provider.
3. Revoke provider/cleanup credentials after reconciliation or its hard deadline,
   disable deployment bindings, and record unresolved resources for incident response.
4. Confirm the static fixture/content remains available throughout cleanup.
5. Stop release promotion; do not create or push `v0.1.13`.
6. Keep v0.1.12 as the documented stable recovery target.
7. Return affected pages and the index to `in-progress` with the failure evidence.

## Completion Gate

When and only when `REL-01` through `REL-10` pass, mark this page `complete`,
update every registry row and master checkbox in `plan/index.md`, reproduce the
index completion algorithm, record both approvals, and set the index status to
`complete`. Merge, tag, push, and publish still require explicit operator action.
