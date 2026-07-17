---
status: deferred
activation: explicit-maintainer-commission
release: unassigned
---

# Deferred Hosted Demo

Hosted terminals, provider sandboxes, public PTYs, and external interactive
showcases are not part of v0.1.13. Issue #135 asks for hosted GitHub automation
meaning CI/CD, not a hosted execution product. Building provider infrastructure
would add accounts, credentials, terms, billing, abuse, privacy, availability,
and cleanup risks without solving the release's reported user pain.

The safe offline fixture/transcript in Phase 03 remains in scope. It uses the
real candidate binary but has no arbitrary execution or external service.

## Activation Conditions

Hosted work requires all of the following before implementation begins:

- a separate issue and release target with a named user outcome;
- explicit maintainer approval of public/private scope and acceptable cost;
- an independent security and cost reviewer;
- a provider-neutral contract and frozen evaluation rubric;
- written terms/acceptable-use clearance for the intended public behavior;
- an approved account, credential, region, concurrency, monthly/total budget,
  expiry, cleanup owner, and kill switch;
- a decision between zero-host and exactly one hosted provider. There is no
  runtime provider selection or paid-provider failover.

Missing approval selects zero-host. It never authorizes account creation,
credential retrieval, payment, provider experiments, or deployment.

## Non-Negotiable Hosted Gates

If activated, the new plan must prove:

- commands are allowlisted and argument-validated; no arbitrary shell, file
  escape, network pivot, secret access, or unrestricted agent mode exists;
- visitors never provide provider keys and provider secrets remain server-side,
  scoped, rotatable, redacted, and absent from logs and client assets;
- authentication/abuse controls, rate limits, concurrency ceilings, reservation,
  inclusive cost accounting, hard budget stops, and denial-of-wallet tests fail closed;
- session creation, timeout, cancellation, crash, and ambiguous provider results
  reconcile to zero orphan resources and a known billing state;
- telemetry is minimized, retention/deletion is explicit, and seeded-secret
  tests cover logs, metrics, traces, errors, recordings, and support artifacts;
- private staging, failure injection, provider outage, quota exhaustion,
  credential failure, kill switch, rollback, and provider exit are rehearsed;
- desktop/mobile, keyboard, focus, screen-reader, contrast, responsive layout,
  and recovery behavior pass before any public link;
- every failure returns to the offline zero-host experience and never invokes a
  second provider;
- independent security, privacy, terms, cost, accessibility, and release review
  approves the exact deployment ref before public exposure.

## Deliberately Unselected for v0.1.13

- Kernel or Cloudflare proofs of concept;
- Killercoda, Codespaces, or other external guided surfaces;
- provider adapters, brokers, manifests, credentials, or deployment routes;
- hosted session operations, analytics, feedback SaaS, or public-compute FinOps;
- publication of a static marketing application.

History retains the earlier detailed provider plan if this work is ever
commissioned. It should not remain on the v0.1.13 critical path.
