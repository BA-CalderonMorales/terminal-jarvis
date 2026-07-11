---
id: "12"
target: v0.1.13
title: Provider Selection
status: proposed
owner: release-owner
reviewer: security-finops-reviewer
depends_on: ["11"]
blocks: ["13"]
---

# 12 - Provider Selection

## Objective

Make an explicit, evidence-backed decision to use Kernel, evaluate and use the
Cloudflare Sandbox fallback, or ship no custom hosted demo for v0.1.13.

## Decision Outcomes

### Outcome A: Kernel selected

Allowed only if every hard requirement in page 10 and every applicable POC
criterion in page 11 passes, including written public-use clearance.

### Outcome B: Cloudflare Sandbox evaluated

Triggered when Kernel fails a hard gate but a custom hosted terminal still has
approved demand and budget. Run the same fixture, benchmark, failure suite, and
terms review. Account for the Workers Paid floor, container configuration,
terminal/WebSocket model, scale-to-zero behavior, and Dockerfile maintenance.

Cloudflare cannot be selected from documentation or pricing comparison alone.
The `platform-evaluation-owner` runs a bounded fallback POC and the
`independent-platform-reviewer` reviews it using the frozen page 10 rubric. The
report must cover release-binary compatibility, PTY/WebSocket fidelity, cold
start and scale-to-zero behavior, isolation and egress controls, teardown and
orphan reconciliation, the inclusive cost worksheet, public-use terms, and the
ongoing container/Dockerfile patching burden. Failed hard gates remain failures.

### Outcome C: No custom hosted demo

Select when neither provider passes or demand does not justify maintenance.
Pages 08-09 remain the production showcase. This is a valid successful outcome,
not a plan failure.

## Delivery-Mode Binding

The approved decision writes exactly one value into the `delivery_mode` field
on pages 13-17:

- Outcome A -> `kernel-hosted`
- Outcome B -> `cloudflare-hosted`
- Outcome C -> `zero-host`

The decision record marks the two unselected alternatives `not-selected` with
the reason and cleanup evidence. It MUST NOT leave later pages with `pending`
mode or use `not applicable` to bypass common acceptance criteria.

## Work

- [ ] Score immutable page 10 criteria using raw POC evidence.
- [ ] Record all failed hard gates without averaging them away.
- [ ] If Cloudflare remains eligible, run and independently review its bounded
  parity POC using the identical fixture, measurements, failure suite, sample
  size, regions, and immutable page 10 thresholds used for Kernel.
- [ ] Compare monthly floor, unit cost, concurrency, maintenance, lock-in, and exit cost.
- [ ] Obtain security, FinOps, terms, platform, and release approvals.
- [ ] Record selected provider/runtime version and adapter boundary, or no-host decision.
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

## Risks and Rollback

- Risk: sunk-cost bias after POC work. Hard gates and a valid no-host outcome
  prevent forced selection.
- Risk: provider pricing or terms change. Store decision date and reevaluation trigger.
- Rollback trigger: selected provider changes a hard gate before rollout.
- Rollback action: activate static/Killercoda fallback and reopen this page.

## Completion Gate

Complete only after one outcome is approved and all nonselected resources are
accounted for. Cloudflare selection additionally requires `SEL-08`; a Kernel or
zero-host outcome records Cloudflare as `not-selected` with the reason. Page 13
follows the bound mode. For `zero-host`, it proves that page 09 is the only
delivery route and that no broker, provider credential, sandbox endpoint, or
billable hosted resource exists.
