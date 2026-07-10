---
target: v0.1.13
branch: release/0.1.13
baseline: v0.1.12
status: proposed
status_source: derived-from-child-pages
release_owner: release-owner
last_reviewed_utc: not-reviewed
---

# Terminal Jarvis v0.1.13 UX Consistency Plan

This is the authoritative execution plan for making Terminal Jarvis more
consistent across supported user environments and for delivering a safe,
cost-controlled interactive showcase. Product consistency is the primary goal;
hosted demonstrations are a downstream consumer of the same contracts and test
fixtures.

The index status is derived. It MUST NOT be changed to `complete` until every
numbered page is `complete`, every acceptance criterion has reproducible
evidence, the final audit in page 17 passes, and the release owner and reviewer
approve the result.

## Authoring Boundary

This planning pass creates local documentation only. It MUST NOT:

- change `Cargo.toml`, npm metadata, changelog versions, or release formulas;
- implement any numbered page;
- create a commit, tag, release, pull request, or remote branch;
- push, publish, upload assets, or mutate external provider state.

Future execution may commit reviewed implementation work normally. Tagging and
publishing remain separate operator decisions after this plan is complete.
Every numbered `Work` checklist describes that later execution; no unchecked
item authorizes implementation during this plan-authoring task.

## Outcome

The v0.1.13 release candidate will provide:

- explicit, testable support claims for each harness and platform;
- stable human and machine output contracts across terminals and install paths;
- deterministic, redacted diagnostics that make environment failures actionable;
- secret-free conformance fixtures reused by CI and every demonstration surface;
- a free static/guided showcase before any paid hosted path is required;
- a measured provider decision, with Kernel evaluated through a bounded PTY
  proof of concept rather than assumed suitable;
- a hosted demo only if its security, terms, reliability, and cost gates pass;
- release automation, documentation, rollback, and operational evidence.

## Non-Goals

- Providing persistent cloud workspaces for every user.
- Installing or authenticating all 25 third-party agent CLIs in a public demo.
- Accepting visitor API keys or storing provider credentials.
- Exposing an unrestricted shell, arbitrary command runner, or `yolo` mode.
- Treating a container or sandbox image as proof of local cross-platform support.
- Replacing the Rust CLI with a web reimplementation or second runtime.
- Publishing v0.1.13 merely because the branch name exists.

## Governance

### Status values

Each numbered page uses exactly one status:

- `proposed`: scoped but not approved to start;
- `ready`: dependencies are complete, owner/reviewer assigned, and scope approved;
- `in-progress`: implementation or evidence collection is active;
- `blocked`: progress cannot continue; blocker, owner, impact, and review date are recorded;
- `review`: implementation is done and independent evidence review is active;
- `complete`: every criterion passed, evidence is current, and rollback is verified.

Allowed transitions are:

```text
proposed -> ready -> in-progress -> review -> complete
                        |              |
                        v              v
                     blocked       in-progress
blocked -> in-progress
```

A page cannot become `ready` until all `depends_on` pages are complete. There
is no `complete-with-exceptions` state. Conditional work is completed through a
documented decision and the acceptance criteria defined on that page, not by
silently marking it not applicable.

Page 12 binds exactly one `delivery_mode`: `kernel-hosted`,
`cloudflare-hosted`, or `zero-host`. Pages 13-17 then replace their pending mode
with that value and satisfy every common criterion using the mode-specific
evidence defined on each page. The unselected alternatives are recorded as
`not-selected` in the page 12 decision record; they are not left pending and do
not create a completion exception.

### Ownership

- Every page requires one accountable owner and one different reviewer.
- Role ownership is assigned in this draft. Page 01 binds each role to a named
  person before any page can transition beyond `proposed`.
- Owners collect evidence; reviewers reproduce or inspect it.
- A blocked page identifies the decision owner and next review date.

### Evidence

Every acceptance criterion requires an evidence row containing:

1. criterion ID;
2. exact command, workflow, or evaluation method;
3. artifact, log, report, or URL;
4. tested commit or release-candidate ref;
5. observed UTC timestamp;
6. result and reviewer.

Evidence MUST be reproducible, secret-free, and tied to the current code. A
change that affects a criterion invalidates its evidence until rerun. Provider
evidence additionally records provider/runtime version, region, fixture,
latency, usage duration, estimated cost, cleanup result, and external side
effects. Credentials are never copied into plan files.

### Scope changes

New public commands, providers, runtime layers, data retention, credential
handling, or release surfaces require an index decision before implementation.
The decision records motivation, affected pages, dependency changes, owner,
reviewer, cost impact, security impact, and rollback impact. Page IDs are
immutable after approval; added work receives the next number.

## Dependency-Ordered Registry

| ID | Page | Depends on | Primary owner | Status |
|---|---|---|---|---|
| 01 | [Baseline and success](01-tj-hardening-baseline-and-success.md) | None | Release owner | proposed |
| 02 | [UX contract](02-tj-hardening-ux-contract.md) | 01 | CLI/UX owner | proposed |
| 03 | [Environment diagnostics](03-tj-hardening-environment-diagnostics.md) | 02 | Diagnostics owner | proposed |
| 04 | [Presentation consistency](04-tj-hardening-presentation-consistency.md) | 02 | CLI/UX owner | proposed |
| 05 | [Harness support model](05-tj-hardening-harness-support-model.md) | 02, 03 | Catalog owner | proposed |
| 06 | [Harness conformance](06-tj-hardening-harness-conformance.md) | 04, 05 | Test owner | proposed |
| 07 | [Distribution parity](07-tj-hardening-distribution-parity.md) | 03, 04, 06 | Distribution owner | proposed |
| 08 | [Deterministic demo fixture](08-tj-hardening-demo-fixture.md) | 03, 05, 06 | Demo/QA owner | proposed |
| 09 | [Zero-cost showcase](09-tj-hardening-zero-cost-showcase.md) | 07, 08 | Docs/demo owner | proposed |
| 10 | [Sandbox requirements](10-tj-hardening-sandbox-requirements.md) | 08, 09 | Platform evaluator | proposed |
| 11 | [Kernel PTY proof](11-tj-hardening-kernel-pty-proof.md) | 10 | Platform evaluator | proposed |
| 12 | [Provider selection](12-tj-hardening-provider-selection.md) | 11 | Release owner | proposed |
| 13 | [Hosted demo implementation](13-tj-hardening-hosted-demo.md) | 08, 12 | Demo platform owner | proposed |
| 14 | [Abuse, cost, and operations](14-tj-hardening-abuse-cost-operations.md) | 10, 13 | Security/FinOps owner | proposed |
| 15 | [CI and release integration](15-tj-hardening-ci-release-integration.md) | 07, 08, 13, 14 | CI/release owner | proposed |
| 16 | [Documentation and feedback](16-tj-hardening-docs-and-feedback.md) | 05, 07, 09, 14, 15 | Documentation owner | proposed |
| 17 | [Rollout and release readiness](17-tj-hardening-rollout-readiness.md) | 01-16 | Release owner | proposed |

## Release Gates

- `G0 - Charter`: page 01 complete; scope, owners, baseline, metrics, and budget approved.
- `G1 - UX contract`: pages 02-04 complete; diagnostics and visible behavior are stable.
- `G2 - Product parity`: pages 05-07 complete; support truth and distributions are verified.
- `G3 - Safe showcase base`: pages 08-09 complete; free paths work without secrets.
- `G4 - Provider decision`: pages 10-12 complete; a measured go/no-go is recorded.
- `G5 - Hosted operations`: pages 13-14 complete; any hosted path is secure and bounded.
- `G6 - Delivery`: pages 15-16 complete; automation and documentation match behavior.
- `G7 - Release candidate`: page 17 and every prior page complete; index may be completed.

## Master Checklist

- [ ] 01 baseline and success measures complete
- [ ] 02 UX contract complete
- [ ] 03 environment diagnostics complete
- [ ] 04 presentation consistency complete
- [ ] 05 harness support model complete
- [ ] 06 harness conformance complete
- [ ] 07 distribution parity complete
- [ ] 08 deterministic demo fixture complete
- [ ] 09 zero-cost showcase complete
- [ ] 10 sandbox requirements complete
- [ ] 11 Kernel PTY proof complete
- [ ] 12 provider selection complete
- [ ] 13 hosted demo implementation complete
- [ ] 14 abuse, cost, and operations complete
- [ ] 15 CI and release integration complete
- [ ] 16 documentation and feedback complete
- [ ] 17 rollout and release readiness complete
- [ ] All evidence rows contain current commit/ref, UTC time, result, and reviewer
- [ ] No unresolved blocker, placeholder owner, or unapproved scope change remains
- [ ] Release owner independently confirms all child statuses and links
- [ ] Index status changed from `proposed`/`in-progress` to `complete`

## Completion Algorithm

The index may be marked `complete` only when all of the following are true:

1. Every registry row is `complete` and matches its child page front matter.
2. Every child checklist and evidence table is complete with no placeholder text.
3. Page 17 reproduces the required verification against the final candidate ref.
4. Security, cost, legal/terms, accessibility, and rollback approvals are recorded.
5. The working tree contains only intended candidate changes.
6. The release owner and independent reviewer sign the completion record below.

Bulk checkbox edits are not completion evidence. If any condition becomes false,
the index returns to `in-progress` and affected child pages return to review.

## Decision Log

| ID | UTC date | Decision | Affected pages | Owner | Evidence |
|---|---|---|---|---|---|
| D-001 | pending | Approve or revise this execution plan | 01-17 | release-owner | pending |

## Completion Record

| Field | Value |
|---|---|
| Final candidate ref | pending |
| All child pages complete | no |
| Release owner | release-owner (named person pending page 01) |
| Independent reviewer | independent-release-reviewer (named person pending page 01) |
| Approval UTC | pending |
| Verification evidence | pending |
