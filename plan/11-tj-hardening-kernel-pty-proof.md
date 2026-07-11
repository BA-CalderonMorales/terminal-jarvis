---
id: "11"
target: v0.1.13
title: Kernel Evaluation Gate and Optional PTY Proof
status: proposed
owner: kernel-poc-owner
reviewer: independent-platform-reviewer
depends_on: ["10"]
blocks: ["12"]
---

# 11 - Kernel Evaluation Gate and Optional PTY Proof

## Objective

Close the Kernel decision without forcing an external account or spend. The
default outcome is a reviewed no-evaluation record. An optional bounded proof
may determine whether Kernel can host the restricted demo through headless
process/PTY APIs, but it is default-off and cannot launch publicly.

## Evaluation Outcomes

### Outcome N: no provider evaluation (default)

Record Kernel as `not-selected-no-opt-in`. Do not create an account, accept
terms, generate/store credentials, call an API, consume credits, install a
provider CLI/SDK, or create a session. Record the decision date and positive
local evidence that zero-host builds/runs with the Kernel adapter absent. A
fresh provider-doc fetch is optional and cannot block nonselection. This outcome
completes the page without provider measurements.

### Outcome P: bounded proof authorized

Requires a current `OI-PROVIDER` naming Kernel. The record must say whether
billable usage is authorized; maximum spend defaults to USD 0 even when credits
are available. Missing hard spend controls, account authority, terms authority,
or cleanup ownership returns to Outcome N. The proof cannot trigger Cloudflare
or any other provider automatically.

## API References to Revalidate at Execution Time

- [Kernel process and filesystem CLI reference](https://www.kernel.sh/docs/reference/cli/browsers)
- [PTY resize endpoint](https://www.kernel.sh/docs/api-reference/browsers/resize-a-pty-backed-process-terminal)
- [Standby behavior](https://www.kernel.sh/docs/browsers/standby)
- [Live-view and CDP lifecycle](https://www.kernel.sh/docs/introduction/create)
- [Kernel pricing and concurrency](https://www.kernel.sh/docs/info/pricing)

Provider APIs, pricing, limits, and terms are time-sensitive. Capture fresh
official-source evidence when this page moves to `ready`.

## Experiment Sequence (Outcome P Only)

1. Validate the unexpired opt-in, no-card/billing posture, hard maximum spend,
   project-scoped credential, kill switch, and lowest safe concurrency cap.
2. Create one headless session with a five-minute timeout and reconcile before continuing.
3. Record `uname`, architecture, libc/runtime compatibility, filesystem, user,
   network, and available shell evidence.
4. Download the pinned Linux release asset and checksum from the approved origin.
5. Verify checksum before marking executable or running it.
6. Run version and fixture preflight without a PTY.
7. Spawn the restricted demo process with `allocate_tty`, fixed rows/columns,
   isolated environment, non-root user, and execution timeout.
8. Stream stdout/stderr, send stdin, resize, interrupt, and observe exit status.
9. Test whether PTY/process streaming prevents standby; if not, test a bounded
   CDP keepalive and measure its cost.
10. Delete on normal close, browser disconnect, process crash, timeout, and broker failure.
11. Stop immediately on uncertain creation/deletion, unknown cost, quota/terms
    change, or failed hard gate; reconcile before any retry.

## Measurements (Outcome P Only)

- Stage samples: one compatibility session, then five lifecycle/failure
  sessions; run 30 cold starts/commands only after both stages pass and the
  remaining opt-in budget is confirmed.
- p50/p95/max create, setup, ready, command round-trip, and delete latency.
- Active seconds and provider-reported usage/cost per session.
- Setup/download failure rate and checksum mismatch behavior.
- Stream ordering, UTF-8/ANSI fidelity, resize behavior, backpressure, and reconnect behavior.
- Orphan count after deliberate disconnect and cleanup failure injection.
- Start with one concurrent session; test five only if the opt-in explicitly
  authorizes it and earlier stages pass.

## Unknowns That Must Be Resolved Before Outcome P Selection

- Whether process/SSE activity keeps a headless session active or requires CDP.
- Whether the current Linux GNU binary runs on Kernel's current image.
- Whether process, filesystem, and PTY APIs are available on the intended plan.
- Whether egress can be restricted sufficiently for a public demo.
- Whether project concurrency and spend controls are hard or advisory.
- Whether Kernel confirms this public end-user demo under applicable terms.

## Work

- [ ] Record exactly one outcome and its decision owner; do not leave a POC pending.
- [ ] For Outcome N, prove this candidate references/requires no Kernel account,
  credential, SDK, workflow, session, deployment, or billable resource. If the
  operator has an unrelated Kernel account, obtain a human attestation that no
  v0.1.13 resource or billing configuration was created.
- [ ] For Outcome P, validate `OI-PROVIDER`, keep spike code in the optional
  Kernel adapter, and use only synthetic data plus the page 08 fixture.
- [ ] For Outcome P, run staged page 10 measurements without changing thresholds;
  capture provider usage/cost records rather than estimates alone.
- [ ] For Outcome P, obtain written public-demo/terms confirmation before a go decision.
- [ ] Delete/reconcile all experiment sessions, credentials not needed afterward,
  SDK/config artifacts, and other resources for either outcome.
- [ ] Produce a reviewed `not-selected` record or signed secret-free POC report.
- [ ] Prove quota, credential, API, timeout, or cleanup failures route to zero-host
  and never invoke Cloudflare or another provider.

## Acceptance Criteria

- [ ] `KER-01` One outcome is reviewed; Outcome P has a complete unexpired opt-in.
- [ ] `KER-02` Outcome N proves no candidate account/API/resource reference or
  use, with owner attestation for external billing state when applicable, or
  Outcome P proves exact-binary checksum/runtime compatibility and PTY lifecycle behavior.
- [ ] `KER-03` Outcome N records USD 0 and zero resources, or Outcome P passes the
  frozen latency, cost, reliability, concurrency, and cleanup gates.
- [ ] `KER-04` Outcome N records standby/billing as not exercised, or Outcome P
  measures standby/keepalive behavior and provider-reported billing impact.
- [ ] `KER-05` Selected-outcome failures fail closed and leave no orphan/resource.
- [ ] `KER-06` Outcome N has no credential, or Outcome P keeps it server-side and scoped.
- [ ] `KER-07` Outcome N records terms as not accepted/not selected, or Outcome P
  records current written public-use clearance.
- [ ] `KER-08` Zero-host builds and runs with the Kernel adapter/SDK/config removed.
- [ ] `KER-09` Every Kernel failure returns to zero-host without a Cloudflare call.

## Evidence

| Criterion | Method | Artifact/URL | Ref | UTC | Result | Reviewer |
|---|---|---|---|---|---|---|
| KER-01 | outcome/opt-in audit | pending | pending | pending | pending | pending |
| KER-02 | absence audit or compatibility/PTY suite | pending | pending | pending | pending | pending |
| KER-03 | zero-resource record or benchmark report | pending | pending | pending | pending | pending |
| KER-04 | not-exercised record or standby/cost experiment | pending | pending | pending | pending | pending |
| KER-05 | selected-outcome failure/cleanup suite | pending | pending | pending | pending | pending |
| KER-06 | credential-absence or secret-boundary review | pending | pending | pending | pending | pending |
| KER-07 | terms nonselection or written clearance | pending | pending | pending | pending | pending |
| KER-08 | adapter/SDK removal build and walkthrough | pending | pending | pending | pending | pending |
| KER-09 | no-cross-provider-failover test | pending | pending | pending | pending | pending |

## Risks and Rollback

- Risk: a browser-focused VM is an unstable general compute dependency.
  Treat runtime compatibility as measured, not guaranteed.
- Risk: a failed POC is rationalized away. Page 10 gates cannot be changed here.
- Rollback trigger: any hard gate fails or terms remain unclear.
- Rollback action: reject Kernel in page 12, remove/disable spike credentials and
  resources, and continue with zero-host. Another provider needs a new opt-in.

## Completion Gate

This page completes with a reviewed Outcome N record or a reviewed Outcome P
pass/fail report. No provider experiment is required for zero-host, and this
page never selects or triggers another provider; page 12 owns that decision.
