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
verified manifest + protected mode gate
          |
static zero-host page + optional hosted terminal UI
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
recovery through a typed protocol. Every failure activates zero-host without
attempting a second provider.

### No-host mode: `zero-host`

Do not deploy a terminal execution client, broker, adapter, provider SDK,
provider credentials, sandbox routes, or billable compute. Deliver the
self-contained page 09 artifact as the primary experience and generate a
versioned `provider: none` manifest. External static/scenario publication needs
its own opt-in. Prove through dependency, route, secret, deployment, account,
and provider-resource inventories that no executable hosted endpoint exists.

## Mode-Specific Work

For hosted mode: enforce the page 08 command grammar server-side, pin the binary
and checksum, bound input/output and process lifetime, add a fake adapter, and
implement every lifecycle/failure state above in private synthetic staging.

For zero-host mode: pin the local static cast/transcript to the page 08 fixture,
add drift checks, omit unselected external links, and ensure any experimental
hosted route returns the zero-host experience without importing an adapter or
creating a provider session.

Zero-host lifecycle/failure evidence replaces session events with deterministic
static cases: missing/corrupt cast, missing/corrupt/stale manifest, checksum or
fixture mismatch, offline load, unsupported browser feature, optional link
disabled/unavailable, and rollback-manifest restoration. Each case must retain
the transcript/downloadable evidence and make no provider call.

## Surface Registry Contract

`demo/surfaces.json` has `schema_version: 1` and exactly one entry for each of
`local-static`, `pages`, `killercoda`, `codespaces`, `analytics`, `feedback`,
and `hosted-terminal`. Each entry has a unique ID, kind, active boolean, cost
class, location, owner, and opt-in ID. `local-static` is always active,
`maintainer-zero`, points to a tracked repository file, and has no opt-in.

Every active external location is HTTPS and resolves through an unexpired
human-approved record of the matching kind. Inactive entries set
`opt_in_id: null`; retaining a stale approval reference is invalid. Exactly one
hosted-terminal entry is active in hosted mode and none is active in zero-host;
its opt-in ID must equal the selected manifest's provider opt-in ID. Unknown or
duplicate kinds fail validation instead of being treated as publication.

## Common Work

- [ ] Replace `delivery_mode: pending-page-12-decision` with the page 12 value.
- [ ] Implement every step for the selected mode and record other modes `not-selected`.
- [ ] Display release/ref, fixture version, checksum, and simulation status.
- [ ] Handle every failure state in the selected mode with a clear recovery.
- [ ] Implement keyboard, focus, screen-reader status, contrast, responsive sizing,
  reduced-motion, copy behavior, and mobile constraints.
- [ ] Keep provider code behind an adapter in hosted mode; prove the adapter and
  privileged routes are absent in zero-host mode.
- [ ] Validate missing/unknown/stale/conflicting manifests, missing adapters and
  credentials, expired opt-ins, disabled kill switch, quota errors, and provider
  outages without making any cross-provider call.
- [ ] Build and run zero-host after removing each provider adapter, provider SDK,
  configuration file, secret name, route, and deployment binding.
- [ ] Generate `demo/surfaces.json` with every local/static, Pages, Killercoda,
  Codespaces, analytics, feedback, and hosted surface marked active/inactive,
  cost class, URL/route, owner, and matching opt-in ID; active external entries
  without a valid record fail validation.
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
- [ ] `HST-08` Zero-host startup, manifest validation, static walkthrough, and
  rollback pass with Kernel and Cloudflare adapters/SDKs removed independently and together.

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
| HST-08 | adapter/SDK removal build and E2E walkthrough | pending | pending | pending | pending | pending |

## Risks and Rollback

- Risk: a terminal-looking UI creates an expectation of a full shell. Banner and
  policy errors must state the bounded demo scope without obscuring the product.
- Risk: reconnect duplicates sessions. Session ownership and idempotent cleanup are required.
- Rollback trigger: policy bypass, credential exposure, uncontrolled lifecycle, or provider outage.
- Rollback action: execute page 14's canonical static-first cleanup sequence and
  serve the zero-host showcase only.

## Completion Gate

Complete only after all common criteria have selected-mode evidence. Private,
synthetic hosted staging may be used for page 14 tests, but public traffic
remains disabled until page 14 completes. Zero-host completes only after hosted
dependencies, routes, secrets, account references, deployment bindings, and
resources are absent from the candidate and adapter-removal behavior is reviewed.
External account-wide absence is a release-owner attestation, not something a
credential-free Luna agent is expected to query.
