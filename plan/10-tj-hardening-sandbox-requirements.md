---
id: "10"
target: v0.1.13
title: Sandbox Requirements and Evaluation Rubric
status: proposed
owner: platform-evaluation-owner
reviewer: security-finops-reviewer
depends_on: ["08", "09"]
blocks: ["11", "14"]
---

# 10 - Sandbox Requirements and Evaluation Rubric

## Objective

Approve a provider-neutral hosted-demo contract, budget, security boundary, and
evaluation rubric before spending credits or writing provider-specific code.

## Mandatory Requirements

- Fresh isolated Linux environment per visitor session.
- Exact release binary installation with checksum verification.
- Interactive PTY: spawn, stdin, stdout/stderr streaming, resize, signal, and exit.
- Server-side secret handling; provider credentials never reach the browser.
- Hard session timeout, explicit deletion, orphan reconciliation, and kill switch.
- Concurrency and creation-rate controls within the approved budget.
- Restricted command policy from page 08; no unrestricted shell.
- No visitor provider credentials, persistent home, or cross-session state.
- Observable startup, command, denial, failure, cleanup, duration, and cost events.
- Public-demo terms, privacy, data-region, and acceptable-use approval.
- Provider adapter boundary and documented fallback/exit path.

## Evaluation Candidates

- Kernel headless browser-session VM and process/PTTY APIs.
- Cloudflare Sandbox as the general-purpose fallback.
- Killercoda/static-only as the no-hosted-service outcome.
- Additional providers require an index scope decision before evaluation.

## Research References

- [Kernel pricing and limits](https://www.kernel.sh/docs/info/pricing)
- [Kernel browser process controls](https://www.kernel.sh/docs/reference/cli/browsers)
- [Kernel termination and timeout behavior](https://www.kernel.sh/docs/browsers/termination)
- [Kernel terms](https://www.kernel.sh/docs/tos)
- [Cloudflare Sandbox SDK](https://developers.cloudflare.com/sandbox/)
- [Cloudflare Containers pricing](https://developers.cloudflare.com/containers/pricing/)

## Work

- [ ] Set weighted scoring for security, PTY fidelity, startup latency, reliability,
  cleanup, cost, concurrency, observability, maintenance, portability, and terms.
- [ ] Set hard go/no-go thresholds before measurements are collected.
- [ ] Set monthly budget, per-session ceiling, concurrency ceiling, and alert thresholds.
- [ ] Define one inclusive cost worksheet covering provider/plan minimums,
  standby and idle time, image storage, egress, logs, broker/web hosting, failed
  starts, failed cleanup, and taxes; credits and temporary discounts are recorded
  separately and cannot make a provider pass.
- [ ] Define a common benchmark fixture, regions, sample size, and timing method.
- [ ] Define failure injections: provider timeout, stream loss, setup failure,
  process crash, client disconnect, delete failure, and exhausted quota.
- [ ] Define required written terms/acceptable-use confirmation for public access.
- [ ] Define data retention and telemetry minimization requirements.
- [ ] Define adapter interface and artifacts required for provider exit.

## Proposed Initial Thresholds Requiring Owner Approval

| Metric | Proposed gate |
|---|---|
| Session duration | 5 minutes default, 10 minutes absolute maximum |
| Per five-minute session compute | <= USD 0.01 |
| Monthly maintainer spend | <= approved hard budget; default proposal USD 5 |
| Concurrent public sessions | <= provider/account cap and approved budget |
| Cold interactive readiness | p95 <= 8 seconds |
| Command round-trip after ready | p95 <= 250 ms excluding command runtime |
| Cleanup | 100% deleted within 60 seconds of close/expiry |
| Arbitrary command escape | 0 successful attempts |
| Credential exposure | 0 occurrences |

## Acceptance Criteria

- [ ] `REQ-01` Mandatory requirements and weighted rubric are approved before POC work.
- [ ] `REQ-02` Numeric cost, latency, reliability, concurrency, and cleanup gates,
  including the complete cost-accounting boundary, are approved.
- [ ] `REQ-03` The benchmark fixture and failure suite are provider-neutral.
- [ ] `REQ-04` Terms and public-use clearance are hard decision gates.
- [ ] `REQ-05` Data retention and telemetry fields are explicitly minimized.
- [ ] `REQ-06` Provider exit can preserve the zero-cost fallback.

## Evidence

| Criterion | Method | Artifact | Ref | UTC | Result | Reviewer |
|---|---|---|---|---|---|---|
| REQ-01 | rubric approval | pending | pending | pending | pending | pending |
| REQ-02 | threshold approval | pending | pending | pending | pending | pending |
| REQ-03 | fixture comparison | pending | pending | pending | pending | pending |
| REQ-04 | terms checklist | pending | pending | pending | pending | pending |
| REQ-05 | data inventory | pending | pending | pending | pending | pending |
| REQ-06 | exit design review | pending | pending | pending | pending | pending |

## Risks and Rollback

- Risk: scoring is changed to justify a preferred provider. Freeze weights and
  thresholds before measurement.
- Risk: low unit price hides abuse exposure. Budget and security are hard gates,
  not weighted tradeoffs.
- Rollback trigger: requirements remain unowned or public-use terms are unclear.
- Rollback action: stop hosted evaluation and ship only page 09 surfaces.

## Completion Gate

Complete only when release, security, FinOps, and platform reviewers approve the
rubric and all numeric placeholders are resolved.
