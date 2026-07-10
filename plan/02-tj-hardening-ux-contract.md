---
id: "02"
target: v0.1.13
title: UX Contract
status: proposed
owner: cli-ux-owner
reviewer: automation-compatibility-reviewer
depends_on: ["01"]
blocks: ["03", "04", "05"]
---

# 02 - UX Contract

## Objective

Define one versioned user-facing contract for command discovery, readiness,
planning, execution, errors, compatibility aliases, and automation output.

## Principles

- The same state uses the same term, severity, exit behavior, and next action.
- Rich output is optimized for people; `--plain` is stable for scripts.
- Unsupported, untested, unavailable, stubbed, and blocked are distinct states.
- Terminal Jarvis describes upstream boundaries rather than claiming every
  harness behaves identically.
- Dangerous actions fail closed and require explicit intent.
- Existing aliases remain compatible unless a separately approved migration is documented.

## Work

- [ ] Inventory every public command, alias, global option, output title, error,
  exit code, and state-changing operation.
- [ ] Define canonical vocabulary for availability, support, environment,
  authentication, capability mode, and remediation.
- [ ] Define stable exit-code categories for usage, unavailable dependency,
  unsupported platform, failed upstream command, policy denial, and internal error.
- [ ] Specify rich, plain, no-color, non-TTY, and `TERM=dumb` behavior.
- [ ] Define whether diagnostics extend `check`/`--info` or introduce `doctor`;
  record the decision before page 03 starts.
- [ ] Define compatibility policy and deprecation requirements for aliases and
  plain-output fields.
- [ ] Define upstream stdout/stderr forwarding boundaries and redaction rules.
- [ ] Update the capability contract design before implementation touches schemas.
- [ ] Add contract fixtures that downstream pages can consume.

## Likely Areas

`src/cli/help.rs`, `src/cli/output*.rs`, `src/cli/compat*.rs`,
`src/cli/table*.rs`, `src/cli/style.rs`, `src/cli/args.rs`, CLI tests,
`docs/harness-capability-contract.md`, and README command tables.

## Acceptance Criteria

- [ ] `UXC-01` Every public command and alias maps to a documented journey and exit category.
- [ ] `UXC-02` State vocabulary is unambiguous and used by rich/plain contracts.
- [ ] `UXC-03` Plain-output compatibility and change policy are explicit.
- [ ] `UXC-04` Unsupported and stubbed capabilities cannot appear operational.
- [ ] `UXC-05` Error messages identify subject, failure, next action, and stable exit class.
- [ ] `UXC-06` Dangerous and state-changing behavior is explicitly classified.
- [ ] `UXC-07` Contract fixtures are approved by CLI, catalog, and documentation owners.

## Evidence

| Criterion | Method | Artifact | Ref | UTC | Result | Reviewer |
|---|---|---|---|---|---|---|
| UXC-01 | command inventory comparison | pending | pending | pending | pending | pending |
| UXC-02 | vocabulary fixture review | pending | pending | pending | pending | pending |
| UXC-03 | plain golden diff | pending | pending | pending | pending | pending |
| UXC-04 | negative contract tests | pending | pending | pending | pending | pending |
| UXC-05 | error matrix tests | pending | pending | pending | pending | pending |
| UXC-06 | side-effect classification review | pending | pending | pending | pending | pending |
| UXC-07 | owner approvals | pending | pending | pending | pending | pending |

## Risks and Rollback

- Risk: cleanup breaks consumers of `--plain`. Preserve existing fields by
  default and require explicit migration evidence for incompatible changes.
- Risk: uniform wording hides upstream differences. Keep support metadata
  factual and harness-specific.
- Rollback trigger: golden output or alias compatibility changes without approval.
- Rollback action: restore the previous output contract and keep new behavior behind an opt-in.

## Completion Gate

Complete only after all contract fixtures and compatibility decisions are
reviewed. Pages 03-05 cannot become ready before this page is complete.
