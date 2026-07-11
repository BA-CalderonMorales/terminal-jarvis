---
target: v0.1.13
branch: release/0.1.13
baseline: v0.1.12
status: proposed
status_source: derived-from-child-pages
release_owner: release-owner
default_delivery_mode: zero-host
default_maintainer_budget_usd: 0
provider_opt_in_required: true
last_reviewed_utc: not-reviewed
---

# Terminal Jarvis v0.1.13 UX Consistency Plan

This is the authoritative execution plan for making Terminal Jarvis more
consistent across supported user environments and for delivering a safe,
cost-controlled showcase. Product consistency is the primary goal. A
self-contained zero-host experience is the release default; hosted
demonstrations are optional downstream consumers of the same contracts and test
fixtures.

The index status is derived. It MUST NOT be changed to `complete` until every
numbered page is `complete`, every acceptance criterion has reproducible
evidence, the final audit in page 17 passes, and the release owner and reviewer
approve the result.

## Planning Boundary

Plan maintenance may update, validate, commit, and push files on this release
branch when the operator requests it. Planning work MUST NOT:

- change `Cargo.toml`, npm metadata, changelog versions, or release formulas;
- implement any numbered page;
- create a tag, release, or pull request;
- publish, upload assets, create accounts, accept provider terms, configure
  billing, use provider credentials, or mutate external provider state.

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
- a self-contained static showcase before any external or paid path is considered;
- an explicit provider decision; Kernel or Cloudflare is evaluated only after a
  bounded operator opt-in and is never required to select zero-host;
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

## Cost and Execution Policy

The default release posture is `zero-host` with a USD 0 maintainer budget. It
requires no new provider account, payment method, provider credential, broker,
sandbox, analytics service, or external scenario. Existing public-repository
GitHub standard runners and Pages may be used only while their official terms
continue to make that use non-billable; larger runners, paid storage, and
metered overages are outside the default.

Cost labels are precise:

- `maintainer-zero`: no incremental maintainer bill or new payment method;
- `external-free-tier`: an external account/quota/terms boundary exists even if
  the current price is zero or credits cover usage;
- `user-metered`: the visitor owns any quota or bill, as with Codespaces;
- `maintainer-metered`: the maintainer can incur a subscription or usage bill.

"Zero-cost" in this plan means `maintainer-zero`, not that every dependency is
free, unlimited, or costless to visitors. Static files and a local/loopback
preview are the only mandatory showcase surface. GitHub Pages, Killercoda,
Codespaces, Kernel, Cloudflare, analytics, and hosted feedback are optional and
must have a selection or `not-selected` record.

### Operator opt-ins

Execution agents cannot authorize any opt-in. A named human operator records:

Every record uses a lowercase-slug `id` and includes `kind`, `status: approved`,
distinct named `owner` and `reviewer`, `owner_kind: human`,
`reviewer_kind: human`, `approved_at`, `expires_at`, and `cost_class` before the
kind-specific fields below. All approval, expiry, and cleanup timestamps are
ISO-8601 UTC ending in `Z`.

- `OI-PROVIDER`: provider, purpose, account owner, credential location, allowed
  regions, maximum sessions/concurrency, maximum total and monthly USD, expiry,
  cleanup owner/deadline, kill switch, and whether billable usage is authorized;
- `OI-PUBLISH`: exact static host, external scenario, or feedback surface,
  cost class, account/payment owner, maximum total/monthly USD, permissions,
  data handling, rollback owner, billable-usage authorization, and
  expiry/review date;
- `OI-USER-METERED`: approval to show an optional link only after the visitor's
  quota/billing ownership, a nonempty disclosure, and
  `maintainer_sponsorship: false` posture are recorded;
- `OI-RELEASE-CI`: any nonstandard/larger runner, paid artifact retention, or
  metered CI service, including its maximum spend and stop condition.

Approved records are secret-free JSON under `demo/opt-ins/<id>.json`; secret
values never appear there. Page 10 owns `demo/manifest.json`, and page 13 owns
`demo/surfaces.json`, which lists every active/inactive external link or service
and its required opt-in. The plan checker validates these records once their
owning page completes; deployment preflight additionally checks current expiry,
protected inputs, credentials, budget reservation, and kill switch.

Missing, expired, incomplete, or contradictory opt-in data means `not-selected`.
It does not block the zero-host lane and never authorizes an agent to create an
account, add a payment method, accept terms, fetch credentials, deploy, or spend.

### Luna execution rules

- Work one page and one bounded write set at a time; record prerequisite refs,
  allowed commands, expected artifacts, and the human decision owner.
- Use repository-declared locked dependencies and existing test infrastructure.
  Do not add a provider SDK, runtime, service, or global tool merely to bypass a
  missing prerequisite.
- Never treat a skipped hard gate as a pass. Use existing non-billable CI or
  return the page to `blocked` with the exact missing evidence.
- Prepare decision packets for terms, billing, security, accessibility, and
  release approval; only a named human can approve them.
- The furthest an execution agent may move a page is `review`. A distinct named
  human reviewer reproduces/inspects evidence and alone moves it to `complete`.
- Provider errors, missing credentials, quota exhaustion, malformed manifests,
  uncertain cleanup, and unavailable APIs always resolve to the static
  zero-host path. They never trigger another provider.

### Delivery lanes

1. `Z - required`: pages 01-09 establish product consistency, an offline
   fixture, and a self-contained static showcase.
2. `E - optional external`: Pages, Killercoda, Codespaces links, and a bounded
   Kernel POC require the matching opt-in but cannot block lane Z.
3. `P - optional hosted`: any Kernel or Cloudflare public deployment requires
   `OI-PROVIDER`, security/terms approval, a nonzero approved budget when
   applicable, and successful staging. Failure returns only to lane Z.

Pages 10-14 still close in `zero-host` mode through provider non-selection and
positive absence evidence. Optional work is checked only after either selected
evidence or a reviewed `not-selected` record is present; it is never silently
treated as not applicable.

Release cut rule: `zero-host` is the v0.1.13 release floor. A hosted mode may be
included only when its opt-in, POC, security/terms, implementation, operations,
and rollback evidence are complete before implementation freeze. Otherwise
page 12 binds `zero-host`, records hosted work `not-selected-for-v0.1.13`, and
the hosted track moves to a later release without delaying this candidate.

Before assigning work to a Luna agent, the owner records a task packet with the
page and criterion IDs, base ref, allowed write paths, allowed network/cost
class, exact commands, expected evidence, stop conditions, and human decision
owner. The agent changes implementation/evidence only within that packet and
does not approve or complete the page itself.

Human finalization is a deliberate phase, not an agent task: before page 02, a
human binds accountable owners/reviewers in page 01; at each page review, a
distinct human accepts or rejects evidence; at page 12, a human selects the
delivery mode; at page 17, the release owner and independent reviewer authorize
the candidate. With no independent human reviewer, this plan can become
evidence-ready but cannot honestly become complete.

Named assignments live in secret-free `plan/ownership.json` with
`schema_version` and one entry per page: page ID, metadata owner/reviewer roles,
distinct `owner_name`/`reviewer_name`, and `owner_kind: human` plus
`reviewer_kind: human`. The checker requires this file before any page can be
complete and requires each completed evidence row's reviewer to match the page
assignment. Execution-agent identities may appear in task packets/evidence
provenance, never in human approval fields.

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

Before page 12 completes, the effective mode is always the index
`default_delivery_mode`. A hosted mode is valid only when its page 12 decision,
versioned manifest, protected deployment input, adapter, credential preflight,
budget policy, and kill switch all agree. Visitors cannot select a provider.
Provider replacement requires a new operator decision and deployment; there is
no runtime provider list, paid-to-paid failover, or automatic retry to another
service.

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

For a complete page, `Ref` is a 7-40 character commit SHA, `UTC` is ISO-8601
ending in `Z`, and `Result` is exactly `pass`, `approved`, `not-selected`, or
`not-selected-zero-host`. The artifact is a durable HTTPS URL or a committed
repository-relative path. The evidence reviewer is named and distinct from the
page owner. The checker resolves every ref as a local commit; for a local
artifact, it also verifies that the path is tracked and exists in that commit.
HTTPS evidence remains a human durability/content check. These checks do not
prove truth; the human reviewer still reproduces or inspects the evidence.

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
| 11 | [Kernel evaluation gate](11-tj-hardening-kernel-pty-proof.md) | 10 | Platform evaluator | proposed |
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
- `G3 - Safe showcase base`: pages 08-09 complete; the maintainer-zero static
  path works without secrets or external services.
- `G4 - Provider decision`: pages 10-12 complete; zero-host may be selected with
  no provider account/POC, while any provider selection has approved evidence.
- `G5 - Delivery safety`: pages 13-14 complete; hosted work is secure and
  bounded, or zero-host has positive absence and adapter-removal evidence.
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
- [ ] 11 Kernel evaluation gate complete
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
4. Selected-mode security, cost/absence, legal/terms/nonselection,
   accessibility, and rollback approvals are recorded.
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
