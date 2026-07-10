---
id: "04"
target: v0.1.13
title: Presentation Consistency
status: proposed
owner: cli-presentation-owner
reviewer: cross-platform-reviewer
depends_on: ["02"]
blocks: ["06", "07"]
---

# 04 - Presentation Consistency

## Objective

Make human-facing and automation-facing output predictable across terminal
sizes, TTY modes, color settings, operating systems, and shells without hiding
upstream harness output.

## Work

- [ ] Create golden fixtures for help, list, check/diagnostics, show, plan,
  current/use, security, gate, version, update dry-run, and representative errors.
- [ ] Test widths 40, 80, 100, and 120 columns plus absent/invalid `COLUMNS`.
- [ ] Test TTY and non-TTY behavior, `--plain`, `--no-color`, `NO_COLOR`,
  `TERM=dumb`, CI environments, and redirected streams.
- [ ] Verify long harness names, descriptions, paths, Unicode user data, and long
  unbroken words never corrupt table frames or overlap content.
- [ ] Define wrapping, truncation, ordering, headings, severity, and stderr rules.
- [ ] Verify Windows newline/path handling and macOS/Linux shell behavior.
- [ ] Preserve upstream process byte streams where the UX contract requires forwarding.
- [ ] Add an intentional golden-update procedure requiring reviewer approval.

## Likely Areas

`src/cli/output*.rs`, `src/cli/table*.rs`, `src/cli/style.rs`,
`tests/cli_presentation_tests.rs`, `tests/cli_help_tests.rs`, and
`scripts/core-command-matrix.sh`.

## Acceptance Criteria

- [ ] `PRE-01` All core rich outputs have approved golden fixtures.
- [ ] `PRE-02` No generated line exceeds the declared width where wrapping applies.
- [ ] `PRE-03` Plain output remains stable, line-oriented, and decoration-free.
- [ ] `PRE-04` Color is absent whenever flags, environment, or terminal state require it.
- [ ] `PRE-05` Ordering and wording are deterministic across supported platforms.
- [ ] `PRE-06` Errors use stderr/exit behavior from the UX contract.
- [ ] `PRE-07` Golden changes fail CI until explicitly reviewed.

## Evidence

| Criterion | Test/workflow | Artifact | Ref | UTC | Result | Reviewer |
|---|---|---|---|---|---|---|
| PRE-01 | pending | pending | pending | pending | pending | pending |
| PRE-02 | width matrix | pending | pending | pending | pending | pending |
| PRE-03 | plain golden diff | pending | pending | pending | pending | pending |
| PRE-04 | color/TTY matrix | pending | pending | pending | pending | pending |
| PRE-05 | native OS comparison | pending | pending | pending | pending | pending |
| PRE-06 | error stream tests | pending | pending | pending | pending | pending |
| PRE-07 | intentional-update test | pending | pending | pending | pending | pending |

## Risks and Rollback

- Risk: exact snapshots become noisy. Snapshot only the public contract and keep
  internal formatting tests focused.
- Risk: truncation removes remediation. Wrap essential text; truncate only
  explicitly nonessential fields.
- Rollback trigger: approved scripts or terminals regress.
- Rollback action: restore previous renderer behavior while retaining failing regression fixtures.

## Completion Gate

Complete only when the matrix passes on Linux, macOS, and Windows and an
independent reviewer approves intentional output changes.
