---
id: "13"
target: v0.1.13
title: Hosted Demo Implementation
status: proposed
owner: demo-platform-owner
reviewer: security-accessibility-reviewer
delivery_mode: pending-page-12-decision
depends_on: ["08", "12"]
blocks: ["14", "15"]
---

# 13 - Hosted Demo Implementation

## Objective

Implement the selected provider outcome as a polished, bounded terminal
experience while preserving the exact fixture, provider adapter, and no-host
fallback contracts.

## Delivery Modes

### Hosted mode: `kernel-hosted` or `cloudflare-hosted`

Implement the following architecture:

```text
static demo page + xterm.js
          |
authenticated/rate-limited session broker
          |
provider-neutral sandbox adapter
          |
ephemeral sandbox + restricted demo process
          |
pinned Terminal Jarvis binary and demo fixture
```

No provider API key, process identifier, raw provider URL, or privileged control
endpoint may be exposed to the browser.

Hosted execution implements session create, ready, input, output, resize,
interrupt, close, expiration, reconnect, provider failure, and kill-switch
fallback through a typed protocol.

### No-host mode: `zero-host`

Do not deploy xterm, a broker, an adapter, provider credentials, sandbox routes,
or billable compute. Deliver page 09 as the primary experience, publish a
versioned static/guided manifest, and prove through route, secret, deployment,
and provider-resource inventories that no executable hosted endpoint exists.

## Mode-Specific Work

For hosted mode: enforce the page 08 command grammar server-side, pin the binary
and checksum, bound input/output and process lifetime, add a fake adapter, and
implement every lifecycle/failure state above.

For zero-host mode: pin the static cast and guided scenario to the page 08
fixture, expose clear static/Killercoda/Codespaces choices, add drift checks,
and ensure any experimental hosted route returns the zero-host experience
without creating a provider session.

## Common Work

- [ ] Replace `delivery_mode: pending-page-12-decision` with the page 12 value.
- [ ] Implement every step for the selected mode and record other modes `not-selected`.
- [ ] Display release/ref, fixture version, checksum, and simulation status.
- [ ] Handle every failure state in the selected mode with a clear recovery.
- [ ] Implement keyboard, focus, screen-reader status, contrast, responsive sizing,
  reduced-motion, copy behavior, and mobile constraints.
- [ ] Keep provider code behind an adapter in hosted mode; prove the adapter and
  privileged routes are absent in zero-host mode.
- [ ] Make the selected experience the first view, not a marketing landing page.
- [ ] Add desktop/mobile E2E tests; include terminal pixel/nonblank checks only in hosted mode.

## Acceptance Criteria

- [ ] `HST-01` A visitor completes the canonical walkthrough through the selected surface.
- [ ] `HST-02` The pinned binary/fixture source, checksum, ref, and simulation state are visible.
- [ ] `HST-03` Hosted input cannot bypass policy; zero-host has no executable input endpoint.
- [ ] `HST-04` All failure/lifecycle states in the selected mode are recoverable.
- [ ] `HST-05` Hosted secrets stay behind the broker; zero-host has no provider secrets or privileged IDs.
- [ ] `HST-06` Desktop, mobile, keyboard, and screen-reader acceptance checks pass.
- [ ] `HST-07` Hosted outage/kill switch or zero-host routing delivers page 09 without a broken page.

## Evidence

| Criterion | Test/method | Artifact/URL | Ref | UTC | Result | Reviewer |
|---|---|---|---|---|---|---|
| HST-01 | selected-mode E2E walkthrough | pending | pending | pending | pending | pending |
| HST-02 | version/checksum assertion | pending | pending | pending | pending | pending |
| HST-03 | protocol attack or executable-route absence audit | pending | pending | pending | pending | pending |
| HST-04 | selected-mode lifecycle/failure suite | pending | pending | pending | pending | pending |
| HST-05 | secret boundary or secret/resource absence audit | pending | pending | pending | pending | pending |
| HST-06 | accessibility/viewport review | pending | pending | pending | pending | pending |
| HST-07 | outage/kill-switch or zero-host route drill | pending | pending | pending | pending | pending |

## Risks and Rollback

- Risk: a terminal-looking UI creates an expectation of a full shell. Banner and
  policy errors must state the bounded demo scope without obscuring the product.
- Risk: reconnect duplicates sessions. Session ownership and idempotent cleanup are required.
- Rollback trigger: policy bypass, credential exposure, uncontrolled lifecycle, or provider outage.
- Rollback action: activate kill switch and serve the zero-cost showcase only.

## Completion Gate

Complete only after all common criteria have selected-mode evidence. Hosted
traffic remains disabled until page 14 completes; zero-host completes only
after absence of hosted routes, secrets, deployments, and resources is reviewed.
