---
id: "11"
target: v0.1.13
title: Kernel Headless PTY Proof of Concept
status: proposed
owner: kernel-poc-owner
reviewer: independent-platform-reviewer
depends_on: ["10"]
blocks: ["12"]
---

# 11 - Kernel Headless PTY Proof of Concept

## Objective

Determine whether Kernel can reliably and economically host the restricted
Terminal Jarvis demo using a headless session and process PTY APIs. This is a
bounded experiment, default-off, with no public launch.

## API References to Revalidate at Execution Time

- [Kernel process and filesystem CLI reference](https://www.kernel.sh/docs/reference/cli/browsers)
- [PTY resize endpoint](https://www.kernel.sh/docs/api-reference/browsers/resize-a-pty-backed-process-terminal)
- [Standby behavior](https://www.kernel.sh/docs/browsers/standby)
- [Live-view and CDP lifecycle](https://www.kernel.sh/docs/introduction/create)
- [Kernel pricing and concurrency](https://www.kernel.sh/docs/info/pricing)

Provider APIs, pricing, limits, and terms are time-sensitive. Capture fresh
official-source evidence when this page moves to `ready`.

## Experiment Sequence

1. Create a project-scoped Kernel credential and the lowest safe concurrency cap.
2. Create a headless session with a five-minute timeout.
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

## Measurements

- At least 30 cold starts and 30 command sessions, within approved credits.
- p50/p95/max create, setup, ready, command round-trip, and delete latency.
- Active seconds and provider-reported usage/cost per session.
- Setup/download failure rate and checksum mismatch behavior.
- Stream ordering, UTF-8/ANSI fidelity, resize behavior, backpressure, and reconnect behavior.
- Orphan count after deliberate disconnect and cleanup failure injection.
- Five-concurrent-session behavior on the Developer plan or current equivalent.

## Unknowns That Must Be Resolved

- Whether process/SSE activity keeps a headless session active or requires CDP.
- Whether the current Linux GNU binary runs on Kernel's current image.
- Whether process, filesystem, and PTY APIs are available on the intended plan.
- Whether egress can be restricted sufficiently for a public demo.
- Whether project concurrency and spend controls are hard or advisory.
- Whether Kernel confirms this public end-user demo under applicable terms.

## Work

- [ ] Keep spike code isolated and provider-specific behind an adapter boundary.
- [ ] Use synthetic data and the page 08 fixture only.
- [ ] Run the page 10 benchmark and failure suite without changing thresholds.
- [ ] Capture usage/cost from provider records, not estimates alone.
- [ ] Obtain written public-demo/terms confirmation before a go decision.
- [ ] Delete all experiment sessions, credentials not needed afterward, and test artifacts.
- [ ] Produce a signed POC report with raw secret-free measurements.

## Acceptance Criteria

- [ ] `KER-01` Exact candidate binary passes checksum and runtime compatibility checks.
- [ ] `KER-02` PTY spawn, stream, stdin, resize, signal, and exit work correctly.
- [ ] `KER-03` All page 10 latency, cost, reliability, and cleanup gates pass.
- [ ] `KER-04` Standby/keepalive behavior and billing impact are measured.
- [ ] `KER-05` Failure injections fail closed and leave no orphan after reconciliation.
- [ ] `KER-06` API credentials remain server-side and project-scoped.
- [ ] `KER-07` Public-demo terms/acceptable-use clearance is recorded.

## Evidence

| Criterion | Method | Artifact/URL | Ref | UTC | Result | Reviewer |
|---|---|---|---|---|---|---|
| KER-01 | compatibility/checksum sample | pending | pending | pending | pending | pending |
| KER-02 | PTY protocol suite | pending | pending | pending | pending | pending |
| KER-03 | benchmark report | pending | pending | pending | pending | pending |
| KER-04 | standby/cost experiment | pending | pending | pending | pending | pending |
| KER-05 | failure/cleanup suite | pending | pending | pending | pending | pending |
| KER-06 | secret boundary review | pending | pending | pending | pending | pending |
| KER-07 | written terms record | pending | pending | pending | pending | pending |

## Risks and Rollback

- Risk: a browser-focused VM is an unstable general compute dependency.
  Treat runtime compatibility as measured, not guaranteed.
- Risk: a failed POC is rationalized away. Page 10 gates cannot be changed here.
- Rollback trigger: any hard gate fails or terms remain unclear.
- Rollback action: reject Kernel in page 12, remove/disable spike credentials and
  resources, and evaluate the declared fallback.

## Completion Gate

This page completes with either a reviewed pass or a reviewed fail report. It
does not select the provider; page 12 owns that decision.
