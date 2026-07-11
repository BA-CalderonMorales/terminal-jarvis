---
id: "08"
target: v0.1.13
title: Deterministic Demo Fixture
status: proposed
owner: demo-fixture-owner
reviewer: security-reviewer
depends_on: ["03", "05", "06"]
blocks: ["09", "10", "13", "15"]
---

# 08 - Deterministic Demo Fixture

## Objective

Create one offline, secret-free demonstration fixture that exercises the real
release binary and catalog contracts while preventing arbitrary execution and
clearly labeling simulated agent behavior.

## Fixture Contract

- Use the exact candidate/release Terminal Jarvis binary, not a web rewrite.
- Use an isolated `TERMINAL_JARVIS_HOME` and disposable sample workspace.
- Use a dedicated demo catalog or reviewed fixture binaries derived from tests.
- Clearly display that agent execution is simulated.
- Permit only a server-side grammar of approved commands.
- Never request or accept visitor provider credentials.
- Never expose shell escape, installer, updater, UI, arbitrary passthrough, or yolo.
- Produce deterministic output suitable for tests, recordings, and guided scenarios.
- The required fixture runs from local files with network disabled, provider
  variables cleared, an explicit executable path, and a minimal environment.
- Provider adapters consume the fixture only when selected; the fixture never
  imports a provider SDK or assumes an external scenario/hosting service exists.

## Work

- [ ] Define the allowed command grammar and argument validation.
- [ ] Define denied commands and expected policy errors.
- [ ] Build deterministic simulated harness executables for safe routing examples.
- [ ] Include representative ready, missing, stubbed, unsupported, and denied states.
- [ ] Seed a non-sensitive sample workspace with reset behavior.
- [ ] Package the fixture independently of user configuration and credentials.
- [ ] Add tests for command injection, separators, control characters, path escape,
  environment override, oversized input/output, and interrupted sessions.
- [ ] Add cases for symlinks, absolute executables, injected `PATH`, `env`, `--`,
  inherited provider variables, seeded canary secrets, invalid UTF-8, and output flooding.
- [ ] Generate a canonical walkthrough script used by every showcase surface.
- [ ] Add an explicit fixture format/version and compatibility test.
- [ ] Prove the offline fixture and static recording still build and run with
  Kernel, Cloudflare, Killercoda, Codespaces, and network access unavailable.

## Likely Areas

Existing fake executable test helpers, a new narrowly scoped demo-fixture area,
catalog fixtures, scripts for deterministic walkthroughs, and package/CI checks.
The implementation must not add a second production runtime.

## Acceptance Criteria

- [ ] `DEM-01` The fixture runs offline with no provider credentials.
- [ ] `DEM-02` It executes the exact candidate binary and identifies its version/ref.
- [ ] `DEM-03` Allowed commands cover the core value proposition and are deterministic.
- [ ] `DEM-04` Denied commands cannot reach a shell or third-party executable.
- [ ] `DEM-05` Simulated behavior is obvious in banner, output, and documentation.
- [ ] `DEM-06` Reset produces an identical clean state for every session.
- [ ] `DEM-07` The same fixture drives mandatory CI/static recording and every
  selected optional surface; unselected surfaces have reviewed decision records.
- [ ] `DEM-08` Offline execution ignores inherited credentials, reads no seeded
  secret, performs no network request, and runs with both provider adapters absent.

## Evidence

| Criterion | Command/test | Artifact | Ref | UTC | Result | Reviewer |
|---|---|---|---|---|---|---|
| DEM-01 | offline run | pending | pending | pending | pending | pending |
| DEM-02 | version/ref assertion | pending | pending | pending | pending | pending |
| DEM-03 | walkthrough golden | pending | pending | pending | pending | pending |
| DEM-04 | adversarial policy tests | pending | pending | pending | pending | pending |
| DEM-05 | content review | pending | pending | pending | pending | pending |
| DEM-06 | repeated reset hash | pending | pending | pending | pending | pending |
| DEM-07 | cross-surface fixture test | pending | pending | pending | pending | pending |
| DEM-08 | offline credential/network/adapter-absence test | pending | pending | pending | pending | pending |

## Risks and Rollback

- Risk: simulation is mistaken for real agent support. Label every simulated
  process and keep support claims sourced from page 05.
- Risk: wrapper filtering is bypassed. Parse a structured allowlist server-side;
  never pass user input to `sh -c`.
- Rollback trigger: arbitrary execution, credential access, or misleading output.
- Rollback action: disable interactive execution and fall back to the static recording.

## Completion Gate

Complete only after security review reproduces the adversarial policy suite,
the local static path works with every external surface absent, and each
selected optional showcase confirms it consumes the same fixture.
