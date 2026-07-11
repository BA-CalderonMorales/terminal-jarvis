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

Approve a provider-neutral demo contract, zero-spend default, optional hosted
budget, security boundary, and evaluation rubric before using credits, creating
an account, or writing provider-specific code.

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
- Provider adapter boundary, zero-host-only failure path, and operator-controlled exit.
- `zero-host` when configuration is absent; USD 0 and no provider call are the defaults.
- No request-time provider selection, provider list, cross-provider retry, or
  paid-to-paid automatic fallback.

## Evaluation Candidates

- Kernel headless browser-session VM and process/PTTY APIs.
- Cloudflare Sandbox as an independently evaluated, explicitly selected opt-in.
- Killercoda/static-only as the no-hosted-service outcome.
- Additional providers require an index scope decision before evaluation.

## Research References

- [Kernel pricing and limits](https://www.kernel.sh/docs/info/pricing)
- [Kernel browser process controls](https://www.kernel.sh/docs/reference/cli/browsers)
- [Kernel termination and timeout behavior](https://www.kernel.sh/docs/browsers/termination)
- [Kernel terms](https://www.kernel.sh/docs/tos)
- [Cloudflare Sandbox SDK](https://developers.cloudflare.com/sandbox/)
- [Cloudflare Containers pricing](https://developers.cloudflare.com/containers/pricing/)

Current official-source observations are planning inputs, not guarantees:
Kernel advertises a usage-metered Developer tier with monthly credits and no
card required to start; Cloudflare Sandbox is available on Workers Paid and its
container flow requires Docker. Revalidate price, credit, account, API, image,
and terms details before any `OI-PROVIDER` approval.

## Provider-Neutral Demo Contract

The implementation defines `demo/manifest.json` and validates it before any
provider code can run. Required fields are:

```text
schema_version
mode: zero-host | kernel-hosted | cloudflare-hosted
provider: none | kernel | cloudflare
fixture_version
fixture_hash
binary_ref
binary_checksum
protocol_version
budget_policy
kill_switch_state
rollback_manifest
provider_opt_in_id
requires_explicit_paid_opt_in
```

`budget_policy` contains numeric `maintainer_budget_usd`, `max_total_usd`,
`max_monthly_usd`, `max_sessions`, and `max_concurrency`. Zero-host sets all
five to zero. Hosted mode copies all five ceilings exactly from its unexpired
provider opt-in; a smaller/larger or stale value fails validation instead of
being interpreted by provider-specific code.

The manifest contains no secret, raw provider URL/session ID, or
visitor-controlled field. `zero-host` requires `provider: none`, budget USD 0,
`provider_opt_in_id: null`, `requires_explicit_paid_opt_in: true`, a
hosted-disabled kill-switch state, and no adapter/broker route. A hosted mode is active only
when the page 12 selection, manifest mode/provider, protected deployment input,
matching adapter, credential preflight, budget reservation, and kill switch all
agree.

`rollback_manifest` is a distinct, tracked repository-relative zero-host
manifest with `provider: none`, USD 0, and hosted execution disabled. Manifest,
rollback, and opt-in records are tracked regular files, never symlinks. A hosted
manifest cannot point to itself or to another hosted mode as rollback.

No manifest means `zero-host`. An unknown mode, mode/provider mismatch, stale or
corrupt checksum, expired opt-in, missing adapter/credential, or disabled kill
switch fails the hosted deployment before traffic changes; the previously
verified zero-host manifest remains active. Visitor input cannot change mode.

The common adapter contract is limited to create, ready, input, output, resize,
interrupt, close, expire, reconnect, usage, and reconcile. Common code contains
no provider SDK imports, provider configuration types, secret names, raw URLs,
or session identifiers. Each optional adapter owns those details and can be
removed without changing the fixture, manifest schema, static UI, or protocol.
Any selected-provider failure routes to zero-host; another provider is never
attempted automatically.

## Selected-Mode Evidence Map

| Hosted requirement | Zero-host closure evidence |
|---|---|
| isolated runtime and PTY lifecycle | no executable route/broker/adapter plus offline static walkthrough |
| provider credential boundary | dependency/workflow/secret-name scan and `provider: none` manifest |
| timeout, deletion, orphan reconciliation | no create/retry/session code path and zero session-resource references |
| concurrency, usage, and cost controls | USD 0 policy, no provider binding, and release-owner billing attestation when applicable |
| provider terms/public-use approval | terms not accepted and provider recorded `not-selected-zero-host` |
| outage and provider exit | missing-adapter/API tests preserve the verified static manifest |

Where a criterion explicitly names an unselected provider outcome, it closes in
zero-host with `not-selected-zero-host` plus the local evidence above, never a
blank or generic not-applicable value. Common mode-independent criteria still
need positive `pass` or `approved` evidence.

## Work

For zero-host, provider-only work below closes through reviewed `not-selected`,
USD 0, no-account/resource, and adapter-absence evidence. Do not create provider
measurements merely to fill a row.

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
- [ ] Define required written terms/acceptable-use confirmation for any selected
  public provider; record terms as not accepted/not selected for zero-host.
- [ ] Define data retention and telemetry minimization requirements.
- [ ] Define adapter interface and artifacts required for provider exit.
- [ ] Define and test the manifest schema, startup/deployment validation, static
  rollback manifest, and exact-match opt-in rule.
- [ ] Define package/module and dependency boundaries that let zero-host build
  and run with each provider adapter and SDK removed.
- [ ] Add fake-adapter contract tests and negative cases for missing/malformed
  config, inherited credentials, 401/403/404/429/5xx, DNS/TLS/schema errors,
  ambiguous creation, quota exhaustion, and cleanup uncertainty.

## Proposed Initial Thresholds Requiring Owner Approval

| Metric | Proposed gate |
|---|---|
| Session duration | 5 minutes default, 10 minutes absolute maximum |
| Per five-minute session compute | USD 0 in zero-host; <= approved opt-in ceiling otherwise |
| Monthly maintainer spend | USD 0 default; any nonzero amount requires `OI-PROVIDER` |
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
- [ ] `REQ-07` Manifest validation defaults to zero-host and requires exact
  human-selection/deployment agreement before any provider call.
- [ ] `REQ-08` Removing either or both provider adapters leaves the fixture,
  static showcase, manifest validation, and zero-host rollback operational.

## Evidence

| Criterion | Method | Artifact | Ref | UTC | Result | Reviewer |
|---|---|---|---|---|---|---|
| REQ-01 | rubric approval | pending | pending | pending | pending | pending |
| REQ-02 | threshold approval | pending | pending | pending | pending | pending |
| REQ-03 | fixture comparison | pending | pending | pending | pending | pending |
| REQ-04 | terms checklist | pending | pending | pending | pending | pending |
| REQ-05 | data inventory | pending | pending | pending | pending | pending |
| REQ-06 | exit design review | pending | pending | pending | pending | pending |
| REQ-07 | manifest/opt-in negative test matrix | pending | pending | pending | pending | pending |
| REQ-08 | adapter/SDK removal build and walkthrough | pending | pending | pending | pending | pending |

## Risks and Rollback

- Risk: scoring is changed to justify a preferred provider. Freeze weights and
  thresholds before measurement.
- Risk: low unit price hides abuse exposure. Budget and security are hard gates,
  not weighted tradeoffs.
- Risk: a provider named as fallback becomes an implicit paid dependency.
  Mitigation: all failures route only to zero-host; provider replacement is a new opt-in.
- Rollback trigger: requirements remain unowned or public-use terms are unclear.
- Rollback action: stop hosted evaluation and ship only page 09 surfaces.

## Completion Gate

Complete when the USD 0 default and provider-neutral contract are approved. A
hosted rubric additionally requires release, security, FinOps, and platform
approval with all numeric placeholders resolved; no provider evaluation is
required to complete this page for zero-host.
