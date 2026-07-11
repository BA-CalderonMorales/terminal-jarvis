---
id: "12"
target: v0.1.13
title: Provider Selection
status: proposed
owner: release-owner
reviewer: security-finops-reviewer
delivery_mode: pending-page-12-decision
depends_on: ["11"]
blocks: ["13"]
---

# 12 - Provider Selection

## Objective

Make an explicit, evidence-backed decision to keep zero-host, opt into Kernel,
or independently opt into Cloudflare Sandbox. No provider is a runtime fallback
for another provider.

## Decision Outcomes

### Outcome K: Kernel selected

Allowed only if every hard requirement in page 10 and every applicable POC
criterion in page 11 passes, including written public-use clearance, and the
current Kernel `OI-PROVIDER` authorizes deployment rather than POC-only use.

### Outcome C: Cloudflare Sandbox selected

Considered only through a new Cloudflare `OI-PROVIDER` when a custom hosted
terminal has approved demand and budget. A Kernel failure cannot trigger it.
Run the same fixture, benchmark, failure suite, and terms review. Account for
the Workers Paid floor, container configuration, terminal/WebSocket model,
scale-to-zero behavior, and Dockerfile/SDK version maintenance.

Cloudflare cannot be selected from documentation or pricing comparison alone.
The `platform-evaluation-owner` runs a bounded independent POC and the
`independent-platform-reviewer` reviews it using the frozen page 10 rubric. The
report must cover release-binary compatibility, PTY/WebSocket fidelity, cold
start and scale-to-zero behavior, isolation and egress controls, teardown and
orphan reconciliation, the inclusive cost worksheet, public-use terms, and the
ongoing container/Dockerfile patching burden. Failed hard gates remain failures.

### Outcome Z: No custom hosted demo

This is the default when no complete provider opt-in exists, demand does not
justify maintenance, or either provider remains unmeasured or fails a hard gate.
Pages 08-09 remain the production showcase. This is a valid successful outcome,
not a plan failure.

## Delivery-Mode Binding

The approved decision writes exactly one value into the `delivery_mode` field
on pages 13-17:

- Outcome K -> `kernel-hosted`
- Outcome C -> `cloudflare-hosted`
- Outcome Z -> `zero-host`

The decision record marks the two unselected alternatives `not-selected` with
the reason and cleanup evidence. It MUST NOT leave later pages with `pending`
mode or use `not applicable` to bypass common acceptance criteria.

## Programmatic Selection Record

Page 12 produces the versioned manifest defined on page 10. Hosted activation
requires this exact equality at deployment:

```text
page_12_selected_mode == manifest.mode == protected_deployment_input
```

The manifest provider must match the selected mode and its unexpired
`OI-PROVIDER`; `zero-host` uses `provider: none`, USD 0, and no opt-in. Missing
selection/manifest defaults to zero-host. Malformed or conflicting explicit
hosted configuration fails deployment and leaves the last verified zero-host
manifest active. Provider replacement is an operator-approved redeployment,
never a request-time choice, retry target, or outage reaction.

## Work

- [ ] Score immutable page 10 criteria for any selected provider using raw POC
  evidence; for zero-host, record the no-provider/absence decision instead.
- [ ] If no provider POC was authorized, select zero-host from page 09 and page
  11 absence evidence without waiting for provider measurements.
- [ ] Record all failed hard gates without averaging them away.
- [ ] If Cloudflare is selected, run and independently review its bounded parity
  POC using the identical fixture, measurements, failure suite, sample size,
  regions, and immutable page 10 thresholds; otherwise record `not-selected`.
- [ ] Compare monthly floor, unit cost, concurrency, maintenance, lock-in, and
  exit cost for selected provider candidates; record zero provider cost for zero-host.
- [ ] Obtain approvals required by the selected outcome. Zero-host requires
  security/release absence approval, not provider billing or provider-terms acceptance.
- [ ] Record selected provider/runtime/opt-in and adapter boundary, or the
  zero-host/no-account decision.
- [ ] Propagate the selected `delivery_mode` to pages 13-17 and record the other
  alternatives as `not-selected`.
- [ ] Record a reevaluation date and triggers such as pricing, terms, API, or demand changes.

## Acceptance Criteria

- [ ] `SEL-01` The decision uses the frozen rubric and raw evidence.
- [ ] `SEL-02` No selected outcome violates a hard security, terms, or budget gate.
- [ ] `SEL-03` The zero-cost fallback remains functional for every outcome.
- [ ] `SEL-04` Selected-provider lock-in is contained behind a documented adapter.
- [ ] `SEL-05` Approval and reevaluation ownership are explicit.
- [ ] `SEL-06` Rejected provider resources and credentials are removed or disabled.
- [ ] `SEL-07` Pages 13-17 are bound to one non-pending delivery mode.
- [ ] `SEL-08` Cloudflare is selected only with a reviewed parity POC covering
  runtime, PTY/WebSocket, scale-to-zero, isolation, cleanup, full cost, terms,
  and container maintenance evidence against the frozen rubric.
- [ ] `SEL-09` Zero-host is the default and no provider call occurs unless the
  selection, manifest, protected input, adapter, credential, and opt-in agree.
- [ ] `SEL-10` Provider outage/failure never attempts another provider; replacement
  requires a new approval and deployment.

## Evidence

| Criterion | Method | Artifact | Ref | UTC | Result | Reviewer |
|---|---|---|---|---|---|---|
| SEL-01 | signed scorecard | pending | pending | pending | pending | pending |
| SEL-02 | hard-gate audit | pending | pending | pending | pending | pending |
| SEL-03 | fallback walkthrough | pending | pending | pending | pending | pending |
| SEL-04 | adapter design review | pending | pending | pending | pending | pending |
| SEL-05 | approval record | pending | pending | pending | pending | pending |
| SEL-06 | resource cleanup audit | pending | pending | pending | pending | pending |
| SEL-07 | delivery-mode propagation audit | pending | pending | pending | pending | pending |
| SEL-08 | Cloudflare parity POC or not-selected record | pending | pending | pending | pending | pending |
| SEL-09 | default-deny manifest/opt-in matrix | pending | pending | pending | pending | pending |
| SEL-10 | no-paid-to-paid-failover test | pending | pending | pending | pending | pending |

## Risks and Rollback

- Risk: sunk-cost bias after POC work. Hard gates and a valid no-host outcome
  prevent forced selection.
- Risk: provider pricing or terms change. Store decision date and reevaluation trigger.
- Rollback trigger: selected provider changes a hard gate before rollout.
- Rollback action: activate the verified static zero-host manifest and reopen
  this page; optional Killercoda remains independent and no other provider starts.

## Completion Gate

Complete only after one outcome is approved, this page's `delivery_mode` is
non-pending, and all nonselected resources are
accounted for. Cloudflare selection additionally requires `SEL-08` as `pass` or
`approved`; Kernel records it as `not-selected`; zero-host records it as
`not-selected-zero-host`. The checker enforces that mode/result mapping. Page
13 follows the bound mode. For `zero-host`, it proves that page 09 is the
required delivery route, any optional external links have explicit opt-ins, and
no broker, provider credential, sandbox endpoint, or billable hosted resource
exists.
