---
id: "07"
target: v0.1.13
title: Distribution Parity
status: proposed
owner: distribution-owner
reviewer: release-engineering-reviewer
depends_on: ["03", "04", "06"]
blocks: ["09", "15", "16"]
---

# 07 - Distribution Parity

## Objective

Make installation, version reporting, catalog discovery, caching, diagnostics,
and update behavior consistent across Cargo, npm/npx, Homebrew, and direct
release assets on every declared platform.

## Work

- [ ] Define one expected post-install state and channel identifier.
- [ ] Test isolated `cargo install`, npm package staging, `npx`, global npm,
  Homebrew formula install/upgrade, and direct executable use.
- [ ] Exercise PATH shadowing among Cargo, npm, Homebrew, and manual copies.
- [ ] Test cache miss, hit, checksum mismatch, partial download, corruption,
  offline reuse, unsupported platform, and permission failure.
- [ ] Verify update dry-run and actual command selection without publishing.
- [ ] Compare embedded versus packaged external catalog/gate behavior.
- [ ] Decide whether to publish Linux musl x64/ARM64 assets; record compatibility,
  npm mapping, Homebrew implications, and release-matrix cost.
- [ ] Verify release asset names/checksums match every launcher and formula consumer.
- [ ] Add standard native CI smoke jobs, or equivalent native evidence, with no
  registry publication side effects or unapproved metered runner.

## Likely Areas

`scripts/package-release.sh`, `scripts/check-distribution-payloads.sh`,
`scripts/release-preflight.sh`, npm launcher/postinstall/tests, Homebrew formula
and README, release workflows, `src/cli/version.rs`, and update diagnostics.

## Acceptance Criteria

- [ ] `DST-01` Every supported install channel reaches the same reported version and catalog.
- [ ] `DST-02` PATH conflicts are diagnosed with channel-specific remediation.
- [ ] `DST-03` Corrupt or mismatched assets fail before execution.
- [ ] `DST-04` Update routes are correct, dry-runnable, and never selected ambiguously.
- [ ] `DST-05` Unsupported platform/channel combinations fail explicitly.
- [ ] `DST-06` A documented musl decision is implemented consistently or rejected with evidence.
- [ ] `DST-07` Standard native smoke jobs perform no tag, publish, tap, or
  dist-tag mutation and use no unapproved metered runner/service.

## Evidence

| Criterion | Command/workflow | Artifact | Ref | UTC | Result | Reviewer |
|---|---|---|---|---|---|---|
| DST-01 | install matrix | pending | pending | pending | pending | pending |
| DST-02 | shadowing matrix | pending | pending | pending | pending | pending |
| DST-03 | checksum/corruption tests | pending | pending | pending | pending | pending |
| DST-04 | update route matrix | pending | pending | pending | pending | pending |
| DST-05 | unsupported matrix | pending | pending | pending | pending | pending |
| DST-06 | musl decision record | pending | pending | pending | pending | pending |
| DST-07 | permission/side-effect audit | pending | pending | pending | pending | pending |

## Risks and Rollback

- Risk: tests accidentally publish. Use staged packages, dry-run flags, read-only
  credentials, and separate workflows without write permissions.
- Risk: adding musl multiplies release paths. Reject it if portability evidence
  does not justify ongoing matrix cost.
- Rollback trigger: any channel reports or runs a different candidate unexpectedly.
- Rollback action: disable the affected channel promotion and retain v0.1.12 recovery instructions.

## Completion Gate

Complete only when the native distribution matrix and no-side-effect review
pass. Nonpublishing standard CI on this public repository is the default native
evidence path; paid/larger runners require `OI-RELEASE-CI`.
