---
id: "05"
target: v0.1.13
title: Harness Support Model
status: proposed
owner: catalog-owner
reviewer: harness-maintainer-reviewer
depends_on: ["02", "03"]
blocks: ["06", "08", "16"]
---

# 05 - Harness Support Model

## Objective

Replace implied blanket support with catalog-derived facts for every harness,
capability, operating system, architecture, shell, and lifecycle operation.

## Required Model

Each harness/capability must distinguish at least:

- `tested`: verified in the declared environment and freshness window;
- `expected`: upstream documents it, but Terminal Jarvis has not recently verified it;
- `stub`: intentionally displays help/guidance instead of performing the capability;
- `unsupported`: known not to work in the environment;
- `disabled`: intentionally blocked by Terminal Jarvis policy;
- `unknown`: insufficient evidence; never rendered as supported.

The model must also express side-effect class, supported OS/architecture/shell,
required executable, environment mode, install/update channel, upstream source,
and last verification evidence without adding per-harness Rust branches.

## Work

- [ ] Audit all 25 root descriptors and all 225 capability descriptors.
- [ ] Design the smallest backward-compatible TOML schema that captures support truth.
- [ ] Document schema tradeoffs before adding Rust fields or dependencies.
- [ ] Update contracts, parser, validator, and embedded catalog generation.
- [ ] Classify help-only headless entries and dangerous placeholders accurately.
- [ ] Classify download/update side effects separately from read-only capabilities.
- [ ] Add invalid/ambiguous metadata fixtures that must fail closed.
- [ ] Generate the supported-agent table and capability truth report from metadata.
- [ ] Define freshness and upstream-drift ownership for top-tier versus community harnesses.

## Likely Areas

`harnesses/**/index.toml`, `src/contracts/`, `src/catalog/parser.rs`,
`src/catalog/validate.rs`, `src/catalog/embedded.rs`, catalog contract/edge tests,
and `docs/harness-capability-contract.md`.

## Acceptance Criteria

- [ ] `SUP-01` Every harness/capability row has one explicit support state.
- [ ] `SUP-02` OS, architecture, shell, and side-effect claims are machine-readable.
- [ ] `SUP-03` Ambiguous or invalid claims fail catalog validation.
- [ ] `SUP-04` Stubbed and unsupported paths cannot be rendered as operational.
- [ ] `SUP-05` The generated support report accounts for all 25 x 9 rows.
- [ ] `SUP-06` Existing catalogs either migrate cleanly or fail with actionable guidance.
- [ ] `SUP-07` No per-harness execution branch or external Rust dependency is introduced.

## Evidence

| Criterion | Command/test | Artifact | Ref | UTC | Result | Reviewer |
|---|---|---|---|---|---|---|
| SUP-01 | catalog coverage report | pending | pending | pending | pending | pending |
| SUP-02 | schema fixture review | pending | pending | pending | pending | pending |
| SUP-03 | invalid metadata tests | pending | pending | pending | pending | pending |
| SUP-04 | CLI negative tests | pending | pending | pending | pending | pending |
| SUP-05 | generated matrix count | pending | pending | pending | pending | pending |
| SUP-06 | migration tests | pending | pending | pending | pending | pending |
| SUP-07 | dependency/design review | pending | pending | pending | pending | pending |

## Risks and Rollback

- Risk: metadata becomes stale. Include freshness evidence and render expired
  verification as expected/unknown rather than tested.
- Risk: schema complexity leaks into Rust branches. Keep classification data-driven.
- Rollback trigger: a catalog migration silently changes commands or support claims.
- Rollback action: reject the new catalog and restore the previous embedded schema.

## Completion Gate

Complete only when the generated truth report has no unclassified row and
catalog/CLI reviewers approve the schema and migration behavior.
