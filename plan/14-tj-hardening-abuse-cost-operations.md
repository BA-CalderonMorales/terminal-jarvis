---
id: "14"
target: v0.1.13
title: Abuse, Cost, and Operations
status: proposed
owner: security-finops-owner
reviewer: independent-operations-reviewer
delivery_mode: pending-page-12-decision
depends_on: ["10", "13"]
blocks: ["15", "16"]
---

# 14 - Abuse, Cost, and Operations

## Objective

Prove that the selected delivery mode fails closed, cannot become a public shell
or credential sink, stays within its approved wallet limit, and can be disabled
or recovered without a product release.

## Threat Model

Cover command injection, shell escape, path traversal, environment override,
control/ANSI abuse, output flooding, fork/process abuse, network abuse, mining,
resource exhaustion, session fixation/hijacking, reconnect races, CSRF/origin
abuse, API-key exposure, denial-of-wallet, orphan sessions, log leakage, and
provider compromise/outage.

For `zero-host`, replace broker/provider attack paths with an explicit absence
audit plus remaining static hosting, external scenario, link hijack, content
drift, analytics, and supply-chain threats. Zero-host is not presumed safe merely
because no custom compute is deployed. Unselected external scenarios, analytics,
and feedback services must be absent rather than left configured.

## Mode-Specific Controls

Hosted mode implements project-scoped secrets, broker/session controls,
restricted non-root execution, egress policy, hard duration/concurrency/budget
stops, atomic budget reservation before create, session deletion, orphan
reconciliation, provider alerts, and key rotation. Unknown cost or ambiguous
creation fails closed until reconciliation; only bounded idempotent retries
against the same selected provider are allowed. Before every create/retry,
revalidate selected mode, exact manifest, unexpired opt-in, active reservation,
remaining budget, and kill switch. Opt-in expiry disables creates immediately.

Zero-host mode proves the candidate references/requires no sandbox credential,
broker route, session store, provider account/deployment, billable resource, analytics/feedback
services, external logs, or unapproved scenario integrations; pins the local
static artifact and provides link-removal/content-rollback controls.

## Common Work

- [ ] Replace `delivery_mode: pending-page-12-decision` with the page 12 value.
- [ ] Implement/test every selected-mode control and record other modes `not-selected`.
- [ ] Complete the selected-mode threat model and supply-chain review.
- [ ] Enforce page 08 policy in hosted mode or prove no executable endpoint in zero-host mode.
- [ ] Enforce the compute ceiling in hosted mode or prove zero custom provider
  compute/resources in zero-host mode.
- [ ] Define privacy-safe metrics: outcome codes and timings, not terminal contents.
- [ ] Define retention/deletion for every selected-mode log, usage, and external platform record.
- [ ] Add selected-mode emergency disable, rollback, and incident runbooks.
- [ ] Run selected-mode adversarial, traffic/load, failure, and recovery tests.
- [ ] In hosted private staging, test budget reservation races, partial/ambiguous
  create, duplicate reconnect, worker/process crash, client disconnect, timeout,
  revoked credentials, delete failure, unknown cost, and orphan reconciliation.
- [ ] In zero-host, replace hosted load tests with dependency, account, secret,
  route, deployment, analytics, external-log, and billable-resource absence tests.
- [ ] Define cleanup-only reconciliation after opt-in expiry: it may list/delete
  existing resources with a bounded cleanup credential until its deadline, but
  cannot create/retry sessions, expand budget, or invoke another provider.

## Required Operational Controls

| Control | Required behavior |
|---|---|
| Kill switch | Hosted: disable sessions; zero-host: remove/redirect affected links/content |
| Budget stop | Hosted: reserve before create and reject unknown/exhausted spend; zero-host: prove USD 0 and no billable resource |
| Concurrency | Hosted: enforce cap; zero-host: document external platform limits |
| Cleanup | Hosted: delete sessions; zero-host: prove no session resource is retained |
| Logs | Exclude credentials and terminal command/body content by default |
| Alerts | Hosted: denial/cleanup/provider/spend alerts; zero-host: local drift/link checks with no analytics service required |
| Fallback | Any failure routes to verified static zero-host; no alternate provider is attempted |

For zero-host, hosted-only budget-race/session criteria close as
`not-selected-zero-host` only after tests prove there is no create/retry/session
path, provider dependency/binding, mutable budget reservation, or billable
resource. Static asset/manifest failure and rollback tests remain mandatory.

## Acceptance Criteria

- [ ] `OPS-01` Threat model has mitigations and owners for every listed class.
- [ ] `OPS-02` Tests find no arbitrary command/secret escape or executable zero-host route.
- [ ] `OPS-03` Hosted ceilings fail closed; zero-host proves no billable custom compute endpoint.
- [ ] `OPS-04` Hosted reconciliation leaves zero orphans; zero-host inventory finds no session resources.
- [ ] `OPS-05` Logs/metrics pass privacy and seeded-secret review.
- [ ] `OPS-06` Selected-mode disable, credential/absence, outage, and incident drills pass.
- [ ] `OPS-07` Hosted cost matches the inclusive model; zero-host verifies USD 0
  maintainer demo spend and no provider/external-service billing resource.
- [ ] `OPS-08` Budget races and uncertain lifecycle states fail closed, reconcile
  before retry, and never invoke another provider or exceed the approved reservation.

## Evidence

| Criterion | Test/drill | Artifact | Ref | UTC | Result | Reviewer |
|---|---|---|---|---|---|---|
| OPS-01 | threat model review | pending | pending | pending | pending | pending |
| OPS-02 | adversarial or route-absence suite | pending | pending | pending | pending | pending |
| OPS-03 | quota test or zero-resource audit | pending | pending | pending | pending | pending |
| OPS-04 | orphan drill or session-resource inventory | pending | pending | pending | pending | pending |
| OPS-05 | redaction/privacy test | pending | pending | pending | pending | pending |
| OPS-06 | selected-mode operational drills | pending | pending | pending | pending | pending |
| OPS-07 | provider usage or zero-cost reconciliation | pending | pending | pending | pending | pending |
| OPS-08 | budget-race/ambiguous-lifecycle failure suite | pending | pending | pending | pending | pending |

## Risks and Rollback

- Risk: account-level provider caps are advisory. Enforce broker-side counters and
  a default-deny hard stop as well.
- Risk: retrying after an ambiguous create duplicates spend/resources. Reconcile
  by idempotency key or inventory before any same-provider retry; never fail over.
- Risk: observability records sensitive commands. Collect structured outcomes only.
- Rollback trigger: any escape, secret leak, uncontrolled spend, or orphan breach.
- Rollback action: atomically route to static zero-host and disable creates/retries;
  run bounded cleanup-only reconciliation; then revoke provider credentials and
  deployment bindings, preserve minimal incident evidence, and remain on page 09.

## Completion Gate

Complete only after security and FinOps reviewers reproduce every selected-mode
control. Zero-host requires positive absence evidence for all external and
billable surfaces; hosted traffic tests run only in private synthetic staging.
The page cannot complete by leaving unselected-mode evidence pending.
