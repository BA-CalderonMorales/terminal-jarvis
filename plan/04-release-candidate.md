---
id: "04"
target: v0.1.13
title: Release Candidate
status: in-progress
owner: core-maintainer
reviewer: pending
starts_after: ["01"]
completion_requires: ["02", "03"]
independent_review_required: true
---

# 04 - Release Candidate

## Objective

Run the same product and conformance contracts against the actual v0.1.13
candidate artifacts, close documentation and version drift, rehearse recovery,
and obtain one independent review before any merge, tag, or publication.

Integration work may begin after Phase 01, but this phase cannot become
evidence-ready until Phases 02 and 03 are complete.

## CI and Artifact Integration

- Make the deterministic contract, support report, diagnostics/redaction,
  conformance, and distribution tests blocking CI gates.
- Build staged, nonpublishing artifacts for Linux x64/ARM64 GNU, macOS x64/ARM64,
  and Windows x64 using the release workflow's native environments.
- Run the same fixture against each native binary and packaged catalog.
- Exercise Cargo source install, npm/npx global and wrapper behavior, Homebrew
  formula install shape, and direct assets using staged candidate inputs.
- Verify architecture, executable permissions, archive shape, checksums,
  embedded catalog identity, channel/version reporting, cache recovery, path
  shadowing, update preview, uninstall/recovery guidance, and corrupt inputs.
- Keep all registry, release, tag, tap, and external-host mutations disabled.

Cross-compilation proves buildability, not native execution. Each claimed target
needs either a successful standard-runner native job or a committed transcript
from a matching host. If a required non-billable runner is unavailable, the
release blocks until equivalent local evidence exists; it does not weaken the gate.

## Documentation and Version Closure

- Generate supported-harness/platform tables from the tested catalog.
- Update quick start, diagnostics, support states, safe lifecycle use, install,
  update, path/cache recovery, manual evidence, and unsupported combinations.
- Remove blanket 25-by-9 support language that is not backed by the report.
- Align `Cargo.toml`, lock metadata where applicable, npm package and lock,
  formula/version references, release configuration, and `CHANGELOG.md` to `0.1.13`.
- Verify every documented command, link, version, and generated table against
  staged artifacts. Documentation must distinguish simulated, manual, expected,
  unsupported, dangerous, and verified behavior.

## Candidate Verification

Run the repository's required commands at the exact candidate ref. Command
names may change only when the same contract is preserved and the evidence
records the replacement.

```sh
ruby scripts/check-plan.rb
scripts/verify.sh
scripts/local-ci.sh --strict --mutation
scripts/package-release.sh --check
scripts/package-release.sh build /tmp/terminal-jarvis-0.1.13
scripts/local-cd.sh --out-dir /tmp/terminal-jarvis-0.1.13-cd
git diff --check
git status --short
```

The full mutation suite remains a release gate. Diff-tier redesign, sharding,
new scheduling, and timeout optimization are outside v0.1.13 unless the current
gate cannot provide valid evidence and a separate measured decision is recorded.

## Recovery and Review

- Rehearse rollback to the v0.1.12 package/catalog behavior without publishing.
- Verify interrupted/corrupt installs and caches recover through documented steps.
- Record the exact candidate commit proposed for merge and tag.
- A reviewer other than the core maintainer inspects criterion coverage,
  candidate jobs, artifacts, support claims, security/redaction results,
  distribution evidence, version alignment, and rollback.
- Plan completion authorizes only a release decision. The maintainer must make a
  separate explicit decision to merge, tag, publish, upload, or update the tap.

## Work

- [ ] Wire the Phase 02 and 03 gates into CI without weakening existing security,
  package, preflight, or mutation coverage.
- [ ] Build all staged native artifacts and run the shared fixture on each target.
- [ ] Validate Cargo, npm/npx, Homebrew, and direct-asset install/update/recovery
  behavior from staged inputs without publication side effects.
- [ ] Generate and verify support/platform documentation from tested metadata.
- [ ] Align every v0.1.13 metadata and release-note source.
- [ ] Run the full candidate verification suite and record every unavailable check.
- [ ] Rehearse v0.1.12 rollback plus corrupt/interrupted install and cache recovery.
- [ ] Freeze the candidate ref and obtain one independent review.

## Acceptance Criteria

- [ ] `REL-01` Every Phase 02 and 03 criterion has accepted evidence on a commit
  that is an ancestor of the exact candidate ref.
- [ ] `REL-02` Every claimed native target runs the staged candidate binary and
  catalog successfully in a matching native environment.
- [ ] `REL-03` Staged Cargo, npm/npx, Homebrew, and direct-asset paths report the
  same version/support truth and pass install, checksum, cache, path, and update tests.
- [ ] `REL-04` Unsupported targets, corrupt assets, wrong architecture, missing
  catalogs, stale caches, and path conflicts fail deterministically with recovery.
- [ ] `REL-05` Full tests, strict verification, security checks, complete mutation,
  package checks, release preflight, and documentation drift checks pass.
- [ ] `REL-06` Cargo, lock, npm, formula, release configuration, changelog, and
  generated documentation agree on v0.1.13 and the tested support matrix.
- [ ] `REL-07` No required check is silently skipped, no evidence contains a
  secret, and no validation step tags, publishes, uploads, or mutates a registry/tap.
- [ ] `REL-08` The v0.1.12 rollback and interrupted/corrupt install/cache recovery
  procedures are reproduced successfully.
- [ ] `REL-09` The exact candidate commit and intended merge/tag target are
  recorded while no tag or release has been created by this phase.
- [ ] `REL-10` A reviewer distinct from the core maintainer approves the complete
  criterion map, artifacts, support claims, security results, and recovery evidence.

## Evidence

| Covers | Method | Artifact | Ref | UTC | Result | Verified by |
|---|---|---|---|---|---|---|
| pending | pending | pending | pending | pending | pending | pending |

## Exit

Set `reviewer` to the independent reviewer's identity and mark this phase
`complete` only after all evidence is accepted. Tagging and publishing remain
outside this plan.
