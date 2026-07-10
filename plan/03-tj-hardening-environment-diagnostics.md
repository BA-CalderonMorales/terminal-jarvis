---
id: "03"
target: v0.1.13
title: Environment Diagnostics
status: proposed
owner: diagnostics-owner
reviewer: security-privacy-reviewer
depends_on: ["02"]
blocks: ["05", "07", "08"]
---

# 03 - Environment Diagnostics

## Objective

Provide one deterministic, scriptable, privacy-safe diagnosis of the installed
Terminal Jarvis environment and actionable remediation for common failures.

## Required Diagnostic Surface

- Terminal Jarvis version, distribution channel, executable path, and path shadowing.
- OS, architecture, shell, TTY state, terminal width, color decision, and relevant locale.
- Catalog and gate source/path plus active harness selection.
- Active harness support state, resolved binary path/version, and environment readiness.
- Cache/config locations and permission/readability state.
- Update route in dry-run form.
- Stable diagnostic codes for support and issue reporting.

Values of environment variables that may contain credentials MUST never be
printed. Only approved variable names and presence states may be emitted.

## Work

- [ ] Resolve the page 02 decision: extend an existing command or add one canonical command.
- [ ] Define rich output, stable machine output, and exit semantics.
- [ ] Build a strict allowlist of diagnostic fields and secret-name patterns.
- [ ] Detect PATH shadowing and conflicting Cargo/npm/Homebrew/manual installs.
- [ ] Detect unsupported OS/architecture and known harness constraints.
- [ ] Add a support-bundle mode that is local, deterministic, and redacted.
- [ ] Add seeded-secret, unusual-path, missing-home, corrupt-config, missing-catalog,
  non-UTF8-safe, and permission-denied tests.
- [ ] Ensure diagnostics never execute an agent, installer, update, or network request.
- [ ] Document stable diagnostic codes and remediation mapping.

## Likely Areas

`src/cli/output.rs`, `src/cli/version.rs`, `src/cli/cache.rs`,
`src/cli/compat_config.rs`, `src/context/`, `src/security/checks.rs`, new
focused tests, and support documentation.

## Acceptance Criteria

- [ ] `DIA-01` One canonical command reports every approved diagnostic field.
- [ ] `DIA-02` Machine output is deterministic and has a versioned schema.
- [ ] `DIA-03` Seeded credentials, tokens, home names, and sensitive paths are redacted.
- [ ] `DIA-04` PATH/channel conflicts produce a specific remediation and exit class.
- [ ] `DIA-05` Unsupported, missing, and malformed states are distinguishable.
- [ ] `DIA-06` Diagnostics have no network or third-party process side effects.
- [ ] `DIA-07` A support bundle can be attached to an issue without manual cleanup.

## Evidence

| Criterion | Command/test | Artifact | Ref | UTC | Result | Reviewer |
|---|---|---|---|---|---|---|
| DIA-01 | pending | pending | pending | pending | pending | pending |
| DIA-02 | pending | pending | pending | pending | pending | pending |
| DIA-03 | adversarial redaction tests | pending | pending | pending | pending | pending |
| DIA-04 | path-shadow fixture matrix | pending | pending | pending | pending | pending |
| DIA-05 | negative-state matrix | pending | pending | pending | pending | pending |
| DIA-06 | offline/process-spawn assertion | pending | pending | pending | pending | pending |
| DIA-07 | support-bundle review | pending | pending | pending | pending | pending |

## Risks and Rollback

- Risk: a support bundle leaks secrets. Use field allowlisting, adversarial tests,
  and no raw environment/config dump.
- Risk: platform probes are brittle. Treat unavailable probes as explicit unknowns.
- Rollback trigger: any secret emission or destructive/network side effect.
- Rollback action: disable bundle generation and retain only previously safe diagnostics.

## Completion Gate

Complete only after redaction and no-side-effect tests pass on the native OS
matrix and the diagnostic schema is reviewed.
