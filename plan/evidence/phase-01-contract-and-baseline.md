# Phase 01 Contract and Baseline

Observed at `2026-07-17T04:59:27Z` on
`78b10f4542c9710d3714d86926c72b5ce0896656` (`release/0.1.13`). This artifact
freezes the contract that Phases 02-04 must implement and verify. Product
metadata intentionally remains `0.1.12` until Phase 04.

## Audit Boundary

The audit was local and nonpublishing. It installed or updated no harness,
submitted no prompt, ran no real agent, used no provider credential, changed no
registry or tap, and created no release or tag. Safe probes used the development
binary, temporary homes, controlled environments, and redirected streams. The
only catalog lifecycle command executed was an existing deterministic yolo
placeholder that prints guidance and exits 1.

Host and tools:

| Field | Value |
|---|---|
| OS | Ubuntu 24.04.3 LTS under WSL2 |
| Kernel | `6.18.33.2-microsoft-standard-WSL2` |
| Architecture/libc | x86_64, glibc 2.39 |
| Shell | Bash 5.2.21 |
| Rust | rustc 1.95.0, cargo 1.95.0 |
| Node/npm | Node 22.22.0, npm 10.9.4 |
| Ruby | 3.2.3 |
| Terminal | non-TTY baseline with `TERM=dumb`; pseudo-TTY probes used `script` |

## Reproduced Baseline

| Command or probe | Result |
|---|---|
| `scripts/verify.sh` | pass; format, clippy, 150 Rust tests, file length, catalog shape, CLI smoke, integration hardening, security, distribution smoke, and coverage completed |
| catalog shape | pass; 25 harnesses, 225 capability descriptors, 9 capabilities per harness |
| npm wrapper suite | pass; 17/17 tests |
| release preflight | pass; Cargo/npm metadata aligned at 0.1.12 |
| package host check | pass; `linux-x64-gnu` / `x86_64-unknown-linux-gnu` |
| line coverage | pass; 94.79%, threshold 90% |
| `ruby scripts/check-plan.rb` | pass; 4 phases, 38 criteria, overall proposed |
| `git diff --check` | pass |
| `cargo-audit` | unavailable; explicitly skipped by the baseline security script |
| `cargo-deny` | unavailable; explicitly skipped by the baseline security script |
| full mutation | installed but intentionally not run; `TJ_MUTATION=1` was not set |
| strict local CI/native matrix | not run in Phase 01; required against the candidate in Phase 04 |

The default verification pass is evidence that existing mechanics still work,
not evidence that every current product claim is true.

### Exact gap probes

All results below are from the frozen ref with isolated home/config state.

| Arguments or setup | Exit | stdout | stderr | Finding |
|---|---:|---|---|---|
| `--help`, `-h`, `help` | 0 | identical top help | empty | no command-specific help |
| `--plain list` | 0 | 25 plain rows | empty | prefix presentation flag works |
| `list --plain` | 0 | rich table | empty | suffix flag is silently ignored |
| `list unexpected` | 0 | rich table | empty | extra argument is silently ignored |
| `--json list` | 2 | empty | unknown flag | JSON is absent |
| `list --json` | 0 | rich table | empty | suffix JSON flag is silently ignored |
| empty PATH, `--plain check` | 0 | 25 binary-missing rows | empty | zero-ready assessment reports success |
| `OPENCODE_API_KEY=` | 0 | env reported ready | empty | empty value counts as present |
| directory named `opencode` on PATH | 0 | binary reported found | empty | existence is mistaken for executability |
| `--plain run aider yolo` | 1 | child text and TJ failure diagnostic | empty | failed stderr/diagnostic cross into stdout |
| missing `opencode`, `run opencode` | 2 | empty | missing-binary error | availability is misclassified as usage |
| `install opencode --dry-run` | 2 | empty | usage error | lifecycle preview is absent |
| `--update --dry-run` | 0 | update preview | empty | only self-update has preview |

Child stdout is inherited, child stderr is captured, successful child stderr is
discarded, failed stderr is converted lossily to text, and signal termination is
collapsed to exit 1. Color is decided from stdout's TTY even for stderr. Width
currently resolves as `39 -> 100`, `40 -> 40`, `80 -> 80`, `100 -> 100`,
`121 -> 120`, and invalid/unset to 100.

## Catalog Truth Baseline

Descriptor presence is not support evidence. The 225 current rows classify by
command shape as follows; "operational-looking" does not mean verified.

| Capability | Help fallback | Fail-closed placeholder | Operational-looking | Total |
|---|---:|---:|---:|---:|
| download | 0 | 0 | 25 | 25 |
| update | 0 | 0 | 25 | 25 |
| headless | 23 | 0 | 2 | 25 |
| version | 1 | 0 | 24 | 25 |
| stats | 25 | 0 | 0 | 25 |
| models | 25 | 0 | 0 | 25 |
| security | 25 | 0 | 0 | 25 |
| yolo | 0 | 23 | 2 | 25 |
| ui | 0 | 0 | 25 | 25 |
| **Total** | **99** | **23** | **103** | **225** |

There are 98 literal `--help` descriptors, one disguised help fallback
(`cursor-agent:version`), and 23 deterministic yolo placeholders: 122/225 rows
are semantically fallback or placeholder. There are also 52 `sh` descriptors,
31 `npm` descriptors, and eight curl-pipe commands. All 25 downloads, all 25
updates, and all 25 interactive UI rows can produce effects if executed.

The v0.1.12 schema cannot encode support, evidence, effect, network,
interaction, platform, required executable, upstream source, freshness, or a
first-class guarantee. The parser silently overwrites duplicate keys and ignores
unknown keys. Existing validation proves shape, not support truth.

## Frozen Support Contract

Every capability row has exactly one support state:

| State | Required meaning and behavior |
|---|---|
| `verified` | Current safe evidence passed in every declared environment. |
| `expected` | Deterministic TJ coverage and a current upstream source exist, but current real smoke does not. |
| `manual` | Safe committed human procedure exists; automation must not claim or attempt it. |
| `stub` | TJ returns guidance without spawning the descriptor command. |
| `unsupported` | The declared environment is known incompatible and rejects with recovery guidance. |
| `disabled` | TJ safety policy blocks the operation before gate or child launch. |
| `unknown` | Evidence is insufficient; fail closed and never render as supported. |

Each row has exactly one primary evidence mode: `deterministic`,
`disposable-real`, `manual`, or `unsupported`. Support display always includes
the state; only `verified` may use the word "verified". Expected, manual, stub,
unsupported, disabled, and unknown never contribute to a verified count.

Side effects use orthogonal fields rather than one overloaded label:

- `effect = read-only | state-changing | dangerous`
- `network = true | false`
- `interaction = noninteractive | interactive`

Every row also records supported platform IDs, required executable, environment
mode/names, an upstream or internal source, and `verified_at` in UTC. Duplicate
or unknown metadata keys are errors. Contradictory combinations are errors.

Guard rules:

1. Stub, unsupported, disabled, unknown, and platform-incompatible rows stop
   before security gates or child launch.
2. Networked state-changing rows have a side-effect-free preview.
3. State-changing execution requires explicit intent; dangerous execution also
   requires a separate dangerous opt-in and an explicit harness/capability.
4. Interactive/manual rows do not run in noninteractive automation.
5. Empty or whitespace-only credential variables are missing; values and value
   lengths are never emitted.
6. Diagnostics use no network and spawn no child.

Freshness at candidate freeze:

- verified and first-class disposable-real evidence: at most 30 days old;
- expected or manual upstream review: at most 90 days old;
- stub, unsupported, disabled, and unknown policy review: performed on the
  candidate ref;
- stale verified evidence downgrades to expected or unknown and cannot satisfy a
  first-class guarantee.

## First-Class Decision

Issue #135 names OpenCode, Codex, Claude Code, Gemini, and Hermes as candidates,
not as already-proven integrations. Current targeted evidence promotes 0/5.

| Candidate | Help fallbacks | Placeholders | Operational-looking | Phase 01 decision |
|---|---:|---:|---:|---|
| OpenCode | 3 | 0 | 6 | candidate; not yet first-class |
| Codex | 3 | 0 | 6 | candidate; not yet first-class |
| Claude Code | 4 | 1 | 4 | candidate; not yet first-class |
| Gemini | 4 | 1 | 4 | candidate; not yet first-class |
| Hermes | 4 | 1 | 4 | candidate; not yet first-class |

A candidate is promoted only if all of these hold:

1. all 9/9 rows are classified and deterministically take the declared path;
2. package discovery/install and version/help pass without credentials in a
   pinned disposable environment;
3. update preview is exact and side-effect free;
4. every promoted capability has current disposable-real evidence;
5. safe start/stop is attempted only when proven non-agentic; otherwise it is
   recorded as manual, expected, disabled, or unsupported and excluded from the
   guarantee;
6. no smoke submits a prompt, authenticates, accepts terms, mutates a repository,
   or performs an agent action.

First-class is therefore a capability-level evidence set, never a blanket
boolean that turns nine descriptors into nine support claims. Phase 03 records a
promotion or non-promotion decision for all 5/5 candidates.

## Frozen CLI Contract

### Surface and parsing

Canonical surfaces are:

```text
help, version, list, check, current, use, show, plan, run,
install, update, auth, config, cache, security, gate,
experimental, direct-harness invocation, self-update
```

Compatibility aliases are `tools -> list`, `status -> check`,
`info <harness> -> show <harness>`, `install <harness> -> run <harness>
download`, `update <harness> -> run <harness> update`, and `<harness> ... -> run
<harness> ui ...`. `templates` and `db` remain explicit removed-command stubs
for v0.1.13 but exit nonzero.

Parser rules:

1. Every token is consumed, forwarded after an explicit boundary, or rejected;
   silent ignores are zero.
2. Global flags work anywhere before `--`. Tokens after `--` belong to a child
   only for run/direct execution.
3. `run --help` is local help; `run <harness> --help` remains child help for
   compatibility.
4. `help <command>` and `<command> --help` produce the same command-specific
   help. Each canonical command supports `-h` and `--help`.
5. Unexpected arguments and unknown flags fail before catalog loading or side
   effects.
6. `--plain` and `--json` are mutually exclusive. JSON and colorless modes
   imply zero ANSI. `--no-color` changes decoration only.

Public surface outcomes:

| Surface | Effect | Successful result | JSON |
|---|---|---|---|
| help/version/list/show/current | read-only | requested data | schema v1 object |
| check/status/security | read-only | assessment plus remediation | schema v1 object |
| plan | read-only | exact quoted argv/effect/intent preview | schema v1 object |
| use | reversible local state | selected harness acknowledgment | schema v1 object |
| auth/config/cache compatibility | read-only | real status; guidance-only stubs exit 4 | schema v1 object |
| gate status/list | read-only | gate state/data | schema v1 object |
| gate enable/disable | reversible local state | selected state; explicit verb is intent | schema v1 object |
| gate run | child execution | byte-preserved child streams | rejected before launch |
| install/update/self-update | networked state change | confirmed operation or dry-run | result/preview object |
| run/direct | child execution | byte-preserved child streams | rejected before launch |
| dangerous capability | dangerous child execution | separately opted-in, confirmed operation | rejected before launch |
| experimental dashboard | read-only feature-gated data | dashboard data | schema v1 object |
| templates/db removed stubs | unavailable | remediation, exit 4 | schema v1 error object |

`check` is the canonical diagnostic command; no `doctor` synonym is added. It
reports allowlisted Terminal Jarvis version/distribution/path, OS/architecture/
libc/shell, TTY/color/width decisions, catalog/gate source, active harness,
support state, executable readiness, environment name/state, config/cache state,
path conflict, update route, and remediation. Harness version is reported from
trusted local metadata or as `unknown:not-probed`; diagnostics never execute a
harness to discover it.

`check` exits 0 only when core diagnostics pass and the active harness is ready,
or at least one harness is ready if none is active. Zero usable harnesses exits
4. Executable readiness requires a regular executable file (including PATHEXT
rules on Windows). Environment values are trimmed; zero characters means empty.

Sensitive paths replace the home prefix with `~` and a temporary root with
`$TMP`; paths outside allowlisted program/config/cache locations expose only the
minimum basename/state needed for remediation. Credential names and
missing/empty/present/malformed state may be shown; values and lengths may not.

### Output, streams, color, and width

- Rich/plain primary data goes to stdout; warnings, progress, TJ diagnostics,
  and errors go to stderr.
- Plain is stable line-oriented text with zero ANSI and no table borders.
- No-color retains rich layout with zero ANSI.
- JSON emits exactly one UTF-8 object on stdout for handled outcomes, including
  handled errors, and leaves stderr empty. Its envelope is
  `schema_version`, `command`, `ok`, `exit_code`, `data`, and `error`.
- Schema version is integer `1`. Fields may be added compatibly; removal,
  renaming, or type changes require a new schema version and deprecation note.
- Child stdout and stderr are forwarded byte-for-byte to their matching parent
  streams on success and failure. TJ's child-failure diagnostic goes to stderr.
- Color decisions use the destination stream. `NO_COLOR`, `TERM=dumb`,
  `--no-color`, `--plain`, `--json`, or a non-TTY destination each force zero
  ANSI on that destination.
- Width is based on display cells. Invalid or absent `COLUMNS` resolves to 100;
  a valid value clamps to 40..120. No TJ-authored line exceeds 40, 80, 100, or
  120 in the corresponding matrix cell. Child bytes are not reflowed.

Stable exit classes:

| Exit | Meaning |
|---:|---|
| 0 | requested outcome achieved |
| 2 | CLI usage, unknown option, or unexpected argument |
| 3 | invalid Terminal Jarvis config, catalog, or session state |
| 4 | unavailable, not ready, unsupported, disabled, stub, or removed surface |
| 5 | safety gate or explicit-intent requirement not satisfied |
| 126 | child found but not executable |
| 127 | child executable not found |

After a child starts, preserve its exact exit code. On Unix, a signal becomes
`128 + signal`. Child exit/status detail is also included in TJ's stderr
diagnostic. Spawn, config, readiness, and policy failures never masquerade as
usage errors.

### Lifecycle intent

Install, harness update, self-update, and dangerous capabilities support
`--dry-run`; dry-run launches no gate or child and writes no state. Interactive
state-changing execution shows the exact plan and asks for confirmation.
Non-TTY execution fails within one second unless it receives `--no-input` plus
an exact operation-bound token such as `--confirm=download:opencode`.
`--no-input` guarantees no prompt. Dangerous execution additionally requires an
explicit capability and `--allow-dangerous`; prompt text never implies yolo.

`use` and local gate selection are immediate, explicit, reversible, and
idempotent. Guidance-only `auth set`, `config reset`, cache maintenance, and
removed commands exit 4 rather than claiming a mutation succeeded.

## Platform and Distribution Matrix

The core binary has five positive native targets and six delivery paths. Every
cell is tested with staged candidate inputs.

| Native target | Cargo | npm global | npx | Homebrew | direct archive | direct executable |
|---|---|---|---|---|---|---|
| Linux x64 GNU | required | required | required | required | tar.gz | required |
| Linux ARM64 GNU | required | required | required | required | tar.gz | required |
| macOS x64 | required | required | required | required | tar.gz | required |
| macOS ARM64 | required | required | required | required | tar.gz | required |
| Windows x64 MSVC | required | required | required | unsupported | zip | `.exe` |

This is 29 positive cells and one deterministic unsupported cell. Node must be
at least 18.17 for npm/npx. A redundant Windows tarball is not a second claimed
user path unless it receives separate evidence.

Shell launch evidence has seven required cells: Bash on both Linux targets, the
documented default shell on both macOS targets, and PowerShell, Command Prompt,
and Git Bash on Windows x64. Core binary support does not imply that a
platform-incompatible harness descriptor is supported.

Only GNU libc is a positive prebuilt Linux claim. musl/Alpine x64 and ARM64,
Termux/Android, Windows ARM, 32-bit systems, and unlisted architectures reject
deterministically unless a Phase 04 artifact expands the matrix. WSL2 GNU is
`expected` via the matching Linux target until the staged fixture is recorded;
WSL1 is unclaimed. The npm wrapper must detect libc/Android rather than silently
selecting a GNU asset.

Docker or an equivalent container is a pinned GNU Linux validation baseline,
not a magic installer for third-party harnesses and not a published product.

## Numeric Success and Abort Thresholds

| Area | Release threshold |
|---|---|
| catalog | 25/25 harnesses and 225/225 rows present and classified exactly once; 0 duplicates/unclassified rows |
| row metadata | 225/225 contain state, tier, effect/network/interaction, platform, executable, source, and freshness |
| truth | 0 non-verified rows rendered/counting as verified |
| first-class | 5/5 candidate decisions; 100% of every promoted guarantee has deterministic and disposable-real evidence |
| safe real smoke | 0 prompts, credentials, terms acceptance, repository mutations, or agent actions |
| CLI surface | 100% of canonical commands, subcommands, aliases, options, help forms, and removed stubs covered |
| parser | 0 ignored/unconsumed tokens; 100% unexpected arguments rejected before effects |
| JSON | exactly 1 document, schema version 1, for every supported invocation |
| secrets | 0 protected values or value lengths in any stream/artifact |
| streams | 0 bytes lost or crossed for child stdout/stderr; 0 child-exit mismatches |
| readiness | 0 empty/whitespace env or non-executable path false positives; zero-ready never exits 0 |
| lifecycle | 0 effects before confirmed intent; 0 noninteractive prompts/hangs; every covered operation has dry-run |
| presentation | 0 ANSI in disabled cells; 0 width overruns at 40/80/100/120 |
| deterministic fixture | 225/225 rows exactly once; 0 network calls, external writes, or real agents |
| native target | 5/5 staged binaries execute the shared fixture natively |
| distribution | 29/29 positive cells pass; 1/1 unsupported cell rejects; 0 silent skips |
| shell/libc | 7/7 shell cells and 2/2 GNU libc targets pass |
| source quality | 100% focused/full tests pass; Rust files at most 100 lines; line coverage at least 90% |
| release gates | checker, verify, strict local CI, complete mutation, package, local-CD, diff, and status gates all exit 0 |
| mutation | `cargo mutants` exits 0 with 0 missed or timed-out mutants in the candidate report |
| publication safety | 0 tags, uploads, publishes, registry/tap mutations, hosted resources, provider accounts, or paid services |

Any denominator miss, claimed-cell skip/failure, support contradiction, secret
leak, unguarded dangerous spawn, artifact identity mismatch, unavailable claimed
native evidence, version drift, or publication side effect aborts the release.

## Issue #135 Crosswalk

| Release-confidence requirement | Existing v0.1.12 evidence | Remaining gate |
|---|---|---|
| integration hardening, not feature sprawl | integration-hardening and core-command scripts exist | all phases; no hosted/provider scope |
| walk every capability where practical | 25×9 descriptor planning exists | truthful schema in 02; exact fixture in 03 |
| safe start/stop/verify supported harnesses | limited fake OpenCode coverage | bounded first-class disposable smoke in 03 |
| Cargo/npm/npx/Homebrew/direct delivery | packaging, wrapper, formula, five-target tag workflow exist | staged install/recovery parity in 03/04 |
| Docker baselines, not magic installers | no baseline exists | pinned validation-only baseline in 03 |
| first-class candidate set | candidates are named but 0/5 proven | 5/5 decisions and capability guarantees in 03 |
| explicit unsupported OS/harness behavior | partial wrapper platform errors | row/platform guards in 02 and negative fixture in 03 |
| easy update path | distribution-aware self-update and npm cache exist | unified preview/provenance/recovery in 02/04 |
| native release confidence | five native tag jobs and checksums exist | nonpublishing candidate matrix in 04 |

No issue #135 release-confidence requirement is unmapped. Hosted terminals,
providers, adapters, accounts, FinOps, analytics, and public sandboxes are
inapplicable to v0.1.13 and remain governed by `plan/deferred-hosted-demo.md`.

## Cost and Authority Boundary

Validation requires zero provider accounts, zero provider credentials, zero
hosted execution resources, and USD 0 maintainer spend. Public upstream package
downloads are permitted only in disposable evidence environments. Phase 04 may
freeze an evidence-ready commit but may not merge, tag, publish, upload, mutate
a registry/tap, or claim independent human approval.
